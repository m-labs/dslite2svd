DEVICES := \
	svd/tm4c1294ncpdt.xml \
	svd/tm4c123gh6pm.xml

.PHONY: all
all: $(DEVICES)

.PHONY: clean
clean:
	rm -f $(DEVICES)

svd/%.xml: targetdb/devices/%.xml
	ruby dslite2svd.rb $< $@
	if which xmllint >/dev/null; then xmllint --schema CMSIS-SVD.xsd --noout $@; fi
