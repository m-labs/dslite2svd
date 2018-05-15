DEVICES  := tm4c123x tm4c129x
SVD2RUST ?= svd2rust

.SECONDARY:

.PHONY: all
all: $(patsubst %,crates/%/src/lib.rs,$(DEVICES))

.PHONY: purge
purge:
	rm -f $(patsubst %,svd/%-vendor.xml,$(DEVICES))
	rm -f $(patsubst %,svd/%.xml,$(DEVICES))
	touch -t 198001010000 $(patsubst %,crates/%/src/lib.rs,$(DEVICES))

.PHONY: rebuild
rebuild: touch all

.PHONY: touch
touch:
	touch ./targetdb/devices/*.xml

svd/%-vendor.xml: data/%.xml overlay/%-interrupts.xml dslite2svd.rb
	ruby dslite2svd.rb $(filter %.xml,$^) $@

svd/%.xml: overlay/%.patch svd/%-vendor.xml
	cp svd/$*-vendor.xml $@
	patch --backup -p1 -i $<
	@if [ -e $@.orig ]; then \
		diff -U6 --label a/svd/$*.xml $@.orig --label b/svd/$*.xml $@ >$<.new; \
		if ! diff -q $< $<.new; then \
			mv $<.new $<; \
		else \
			rm $<.new; \
		fi; \
		rm $@.orig; \
	fi
	@if which xmllint >/dev/null; then \
		xmllint --schema CMSIS-SVD.xsd --noout $@; \
	fi

crates/%/src/lib.rs: svd/%.xml
	cd crates/$* && svd2rust -i ../../$< && rm -rf src && form -i lib.rs -o src/ && rm lib.rs && cargo fmt
