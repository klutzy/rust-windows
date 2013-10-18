RUSTC=rustc.exe
SRC=win32.rs ll/all.rs ll/windef.rs ll/platform.rs window.rs

.PHONY: all
all: libwin32.dummy

libwin32.dummy: $(SRC)
	$(RUSTC) --lib -o $@ $< && touch libwin32.dummy

.PHONY: test
test: $(SRC)
	$(RUSTC) --test --lib -o $@ $<


.PHONY: examples
examples: libwin32.dummy
	$(MAKE) -C examples


.PHONY: clean
clean:
	rm -rf libwin32.dummy win32*.dll test*.dll
	$(MAKE) -C examples clean
