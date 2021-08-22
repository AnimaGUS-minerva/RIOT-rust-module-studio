all: build

include ../xtensa.mk

build:
	make build-module
	make build-riot

run:
	cargo run --bin emu

test:
	make build
	make run || true

clean:
	rm -rf target
