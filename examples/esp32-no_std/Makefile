all: build

include ../xtensa.mk

build:
	make build-module
	make build-riot

run:
	cargo run --bin emu

test: build

clean:
	rm -rf target
