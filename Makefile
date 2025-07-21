PROGRAM_DIR := prog
PROGRAM_SOURCES := $(wildcard prog/*)
PROGRAM_BINARY := prog.bin

RC := cargo
R_RUN_FLAGS := run -q
R_WATCH_FLAGS := watch -q -c -x
R_WATCH_COMMAND := 'run -q --bin vm -- $(PROGRAM_BINARY)'

# -----------------------------------------------------------

all: run
.PHONY: all

run: gen-bin
	$(RC) $(R_RUN_FLAGS) --bin vm -- $(PROGRAM_BINARY)
.PHONY: run

gen-bin:
	$(RC) $(R_RUN_FLAGS) --bin asm -- $(PROGRAM_DIR)/add > $(PROGRAM_BINARY)
.PHONY: gen-bin

build:
	cargo build
.PHONY: build

watch: gen-bin $(PROGRAM_SOURCES)
	$(RC) $(R_WATCH_FLAGS) $(R_WATCH_COMMAND)
