DEVICES := svd/tm4c123x.xml svd/tm4c129x.xml

.PHONY: all
all: $(DEVICES)

.PHONY: clean
clean:
	rm -f $(DEVICES)

svd/%.xml: overlay/%-interrupts.xml overlay/%.xml.patch dslite2svd.rb
	ruby dslite2svd.rb $(filter %.xml,$^) $@
	patch --backup -p1 <overlay/$(notdir $@).patch
	@if [ -e $@.orig ]; then \
		diff -u --label a/$@ $@.orig --label b/$@ $@ >overlay/$(notdir $@).patch; \
		rm $@.orig; \
	fi
	@if which xmllint >/dev/null; then \
		xmllint --schema CMSIS-SVD.xsd --noout $@; \
	fi

svd/tm4c123x.xml:: targetdb/devices/tm4c123gh6pm.xml
svd/tm4c129x.xml:: targetdb/devices/tm4c1294ncpdt.xml
