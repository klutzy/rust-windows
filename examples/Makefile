RUSTC=rustc.exe
SRC=hello.rs
EXE=$(patsubst %.rs, %.exe, $(SRC))
OBJ=$(patsubst %.rs, %.o, $(SRC))

.PHONY: all
all: $(EXE)

%.exe: %.rs ../libwin32.dummy
	$(RUSTC) -L .. -o $@ $<

.PHONY: clean
clean:
	rm -rf $(EXE) $(OBJ)
