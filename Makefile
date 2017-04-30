DEVICES := crates/tm4c123x/src/lib.rs crates/tm4c129x/src/lib.rs
SVD2RUST ?= svd2rust

.PHONY: all
all: $(DEVICES)

.PHONY: clean
clean:
	rm -f $(DEVICES)

svd/%.xml: overlay/%-interrupts.xml overlay/%.xml.patch dslite2svd.rb Makefile
	ruby dslite2svd.rb $(filter %.xml,$^) $@
	patch --backup -p1 <overlay/$(notdir $@).patch
	@if [ -e $@.orig ]; then \
		diff -u --label a/$@ $@.orig --label b/$@ $@ >overlay/$(notdir $@).patch.new; \
		if ! diff -q overlay/$(notdir $@).patch overlay/$(notdir $@).patch.new; then \
			mv overlay/$(notdir $@).patch.new overlay/$(notdir $@).patch; \
			echo "updated patch"; \
		else \
			rm overlay/$(notdir $@).patch.new; \
		fi; \
		rm $@.orig; \
	fi
	@if which xmllint >/dev/null; then \
		xmllint --schema CMSIS-SVD.xsd --noout $@; \
	fi

crates/%/src/lib.rs: svd/%.xml
	$(SVD2RUST) -i $< >$@
	rustfmt $@
	cargo build --manifest-path crates/$*/Cargo.toml

svd/tm4c123x.xml:: targetdb/devices/tm4c123gh6pm.xml
svd/tm4c129x.xml:: targetdb/devices/tm4c1294ncpdt.xml
