PROGRAM_DIR := prog
PROGRAM_SOURCES := $(wildcard prog/*)
PROGRAM_ASSEMBLY := $(wildcard prog/*_asm)
PROGRAM_BINARY := prog.bin
PROGRAM_HEX := prog.bin.hex

RC := cargo
R_RUN_FLAGS := run -q
R_WATCH_FLAGS := watch -q -c -x
R_WATCH_COMMAND := 'run -q --bin vm -- $(PROGRAM_BINARY)'

# -----------------------------------------------------------

all: run
.PHONY: all

run: gen-hex
	$(RC) $(R_RUN_FLAGS) --bin vm -- $(PROGRAM_HEX)
.PHONY: run

# This generates the binary file from the hex code written manually
# Example:
# prog/add:
# 01 0A
# 01 08
# .....
# .....
# 05 09
gen-bin:
	$(RC) $(R_RUN_FLAGS) --bin bin -- $(PROGRAM_DIR)/add > $(PROGRAM_BINARY)
.PHONY: gen-bin

# This generates the hex file from the assembly code written manually
# Example:
# prog/add_asm:
# PUSH 10
# PUSH 8
# ADDS
# POP A
# SIG $09
gen-hex:
	$(RC) $(R_RUN_FLAGS) --bin asm -- $(PROGRAM_DIR)/add_asm > $(PROGRAM_HEX)
.PHONY: gen-hex


build:
	cargo build
.PHONY: build

format:
	cargo fmt -- --config-path .rustfmt.toml
.PHONY: format

watch: gen-bin $(PROGRAM_SOURCES) $(PROGRAM_ASSEMBLY)
	$(RC) $(R_WATCH_FLAGS) $(R_WATCH_COMMAND)
