all: run
.PHONY: all

run:
	cargo run
.PHONY: run

build:
	cargo build
.PHONY: build

watch:
	cargo watch -c -q -x 'run -q'
