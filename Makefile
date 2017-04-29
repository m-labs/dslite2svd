DEVICES := svd/tm4c123x.xml svd/tm4c129x.xml

.PHONY: all
all: $(DEVICES)

.PHONY: clean
clean:
	rm -f $(DEVICES)

svd/%.xml:
	ruby dslite2svd.rb $< $@
	patch -p1 <patches/$(notdir $(basename $@)).patch
	if which xmllint >/dev/null; then xmllint --schema CMSIS-SVD.xsd --noout $@; fi

svd/tm4c123x.xml:: targetdb/devices/tm4c123gh6pm.xml
svd/tm4c129x.xml:: targetdb/devices/tm4c1294ncpdt.xml
