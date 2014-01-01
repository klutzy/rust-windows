RUSTC?=rustc.exe
RUST_OPTS?=
SRC=$(wildcard *.rs) $(wildcard ll/*.rs)

.PHONY: all
all: libwindows.dummy

libwindows.dummy: $(SRC)
	$(RUSTC) --lib lib.rs $(RUST_OPTS)
	touch $@

.PHONY: check
check: $(SRC)
	$(RUSTC) --test -o $@ lib.rs
	./$@


.PHONY: examples
examples: libwindows.dummy
	$(MAKE) -C examples RUST_OPTS="$(RUST_OPTS)"


.PHONY: clean
clean:
	rm -rf libwindows.dummy librust-windows-*.rlib rust-windows-*.dll *.exe
	$(MAKE) -C examples clean
