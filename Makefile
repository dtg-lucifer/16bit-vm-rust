RC := cargo
R_RUN_FLAGS := run -q
R_WATCH_FLAGS := watch -q -c -x

PROGRAM_DIR := prog

all: run
.PHONY: all

run: gen-bin
	$(RC) $(R_RUN_FLAGS) --bin vm -- prog.bin
.PHONY: run

gen-bin:
	$(RC) $(R_RUN_FLAGS) --bin asm -- prog/add > prog.bin
.PHONY: gen-bin

build:
	cargo build
.PHONY: build

watch: gen-bin
	$(RC) $(R_WATCH_FLAGS) 'run -q --bin vm -- prog.bin'
