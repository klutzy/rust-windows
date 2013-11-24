RUSTC=rustc.exe
RUST_OPTS=
SRC=$(wildcard *.rs) $(wildcard ll/*.rs)

.PHONY: all
all: libwin32.dummy

libwin32.dummy: $(SRC)
	$(RUSTC) --lib -o $@ win32.rs $(RUST_OPTS)
	touch libwin32.dummy

.PHONY: test
test: $(SRC)
	$(RUSTC) --test --lib -o $@ $<


.PHONY: examples
examples: libwin32.dummy
	$(MAKE) -C examples RUST_OPTS="$(RUST_OPTS)"


.PHONY: clean
clean:
	rm -rf libwin32.dummy win32*.dll test*.dll
	$(MAKE) -C examples clean
