all: build

include ../esp32.mk

build:
	make esp32-build-module
	make esp32-build-riot

run:
	cargo run --bin emu

test:
	make build
	make run || true

clean:
	rm -rf target
