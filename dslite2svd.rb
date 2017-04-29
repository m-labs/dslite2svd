require 'bundler/setup'
require 'oga'
require 'builder'

input  = ARGV[0]
output = ARGV[1]

ti_doc = Oga.parse_xml(File.read(input))
svd = Builder::XmlMarkup.new(target: File.open(output, 'w'), indent: 2)

def if_not_empty(x)
  yield x unless x.empty?
end

def common_id_prefix(nodes)
  if nodes.one?
    /^$/
  else
    prefix = nodes.first.get('id').dup
    nodes.each do |node|
      until node.get('id').start_with?(prefix) && node.get('id') != prefix
        prefix.sub!(/_[^_]+_?$/, '_')
      end
    end
    /^#{Regexp.escape prefix}/
  end
end

def strip_id_prefix(node, prefix)
  id = node.get('id')
  id = id.sub(prefix, '')
  if id =~ /^[0-9]/
    id = "_#{id}"
  end
  id
end

device = ti_doc.xpath('device').first
svd.device(schemaVersion: '1.1',
           'xmlns:xs': 'http://www.w3.org/2001/XMLSchema-instance',
           'xs:noNamespaceSchemaLocation': 'CMSIS-SVD.xsd') do |x|
  x.vendor('Texas Instruments')
  x.vendorID('TI')
  x.name(device.get('id'))
  x.series(device.xpath('property[id=FilterString]').first.get('Value'))
  x.version(device.get('HW_revision'))
  if device.get('description').empty?
    x.description('(no description)')
  else
    x.description(device.get('description'))
  end
  #TODO x.cpu()
  x.addressUnitBits(8)
  x.width(32)

  instances = device.xpath('router//instance')
  seen_instances = {}
  x.peripherals do |x|
    instances.each do |instance|
      if %w(cs_dap_0 cortex_m3_0 cortex_m4_0 fpu).include?(instance.get('id').downcase)
        next
      end

      mod_path = File.join(File.dirname(input), instance.get('href'))
      mod_doc = Oga.parse_xml(File.read(mod_path))
      mod = mod_doc.xpath('module').first

      x.comment! instance.get('href')

      seen_instance = seen_instances[instance.get('href')]
      if seen_instance
        peripheral_attrs = {derivedFrom: seen_instance}
      else
        peripheral_attrs = {}
        seen_instances[instance.get('href')] = instance.get('id')
      end

      x.peripheral(peripheral_attrs) do |x|
        x.name(instance.get('id'))
        unless seen_instance
          if_not_empty(mod.get('HW_revision')) { |v| x.version(v) }
          if_not_empty(mod.get('description')) { |v| x.description(v) }
        end
        x.baseAddress(instance.get('baseaddr'))
        x.addressBlock do |x|
          x.offset(0)
          x.size(instance.get('size'))
          x.usage('registers')
          unless %w(s n p).include?(instance.get('permissions'))
            raise "Unknown permissions in #{instance.inspect}"
          end
          x.protection(instance.get('permissions'))
        end

        next if seen_instance

        registers = mod.xpath('register')
        register_id_prefix = common_id_prefix(registers)
        seen_registers = []
        x.registers do |x|
          registers.each do |register|
            if seen_registers.include?(register.get('id'))
              # This happens in a few places; the intended behavior is probably merging
              # the register definitions, but since all of those lack bitfields anyway,
              # we don't care.
              puts "Ignoring duplicate register #{instance.get('id')}.#{register.get('id')}"
              next
            else
              seen_registers.push(register.get('id'))
            end

            x.register do |x|
              x.name(strip_id_prefix(register, register_id_prefix))
              x.displayName(register.get('acronym'))
              x.description(register.get('description'))
              if register.get('offset').nil?
                raise "No register offset in #{register.inspect}"
              end
              x.addressOffset(register.get('offset'))
              x.size(register.get('width'))

              access_map = {
                'R'   => 'read-only',
                'RO'  => 'read-only',
                'RW'  => 'read-write',
                'R/W' => 'read-write',
                'W'   => 'write-only',
                'WO'  => 'write-only',
                'N'   => 'read-write', # N means it can be different types (eg: RW1C or RO)
              }

              bitfields = register.xpath('bitfield')
              next if bitfields.empty?

              if bitfields.size == 1 && bitfields.first.get('width') == register.get('width')
                # sole bitfield; so just a register
                x.access(access_map.fetch(bitfields.first.get('rwaccess')))
                next
              end

              bitfield_id_prefix = common_id_prefix(bitfields)
              x.fields do
                bitfields.each do |bitfield|
                  x.field do |x|
                    x.name(strip_id_prefix(bitfield, bitfield_id_prefix))
                    x.description(bitfield.get('description'))
                    x.lsb(bitfield.get('end'))
                    x.msb(bitfield.get('begin'))
                    x.access(access_map.fetch(bitfield.get('rwaccess')))
                    if !bitfield.get('range').empty?
                      raise "Unexpected non-empty range in #{bitfield}"
                    end

                    bitenums = bitfield.xpath('bitenum')
                    next if bitenums.empty?

                    bitenum_id_prefix = common_id_prefix(bitenums)
                    x.enumeratedValues do |x|
                      bitenums.each do |bitenum|
                        x.enumeratedValue do |x|
                          x.name(strip_id_prefix(bitenum, bitenum_id_prefix))
                          x.description(bitenum.get('description'))

                          value = Integer(bitenum.get('value'))
                          value >>= Integer(bitfield.get('end'))
                          x.value("0x#{value.to_s(16)}")

                          if !bitenum.get('token')
                            raise "Unexpected non-empty token in #{bitenum}"
                          end
                        end
                      end
                    end
                  end
                end
              end
            end
          end
        end
      end
    end
  end
end
