PROGRAM_DIR := prog
PROGRAM_SOURCES := $(wildcard prog/*)
PROGRAM_ASSEMBLY := $(wildcard prog/*_asm)
PROGRAM_BINARY := prog.bin
PROGRAM_HEX := prog.hex

RC := cargo
R_RUN_FLAGS := run -q
R_WATCH_FLAGS := watch -q -c -x
R_WATCH_COMMAND := 'run -q --bin vm -- $(PROGRAM_HEX)'

# -----------------------------------------------------------

all: run
.PHONY: all

run: gen-hex
	$(RC) $(R_RUN_FLAGS) --bin vm -- $(PROGRAM_HEX)
.PHONY: run

gen-hex:
	$(RC) $(R_RUN_FLAGS) --bin asm -- $(PROGRAM_DIR)/add_asm > $(PROGRAM_HEX)
.PHONY: gen-hex

build:
	cargo build
.PHONY: build

format:
	cargo fmt -- --config-path .rustfmt.toml
.PHONY: format

test:
	cargo test --tests
.PHONY: test

watch: gen-hex $(PROGRAM_SOURCES) $(PROGRAM_ASSEMBLY)
	$(RC) $(R_WATCH_FLAGS) $(R_WATCH_COMMAND)
