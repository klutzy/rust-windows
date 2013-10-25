RUSTC=rustc.exe
SRC=hello.rs
RES=hello.rc
EXE=$(patsubst %.rs, %.exe, $(SRC))
RC=$(patsubst %.rc, %.rc.o, $(RES))

.SUFFIXES:

.PHONY: all
all: $(EXE)

%.exe: %.rs ../libwin32.dummy %.rc.o
	$(RUSTC) -L .. -o $@ $< --link-args $*.rc.o

%.rc.o: %.rc
	windres $< $@

.PHONY: clean
clean:
	rm -rf $(EXE) $(RC)
