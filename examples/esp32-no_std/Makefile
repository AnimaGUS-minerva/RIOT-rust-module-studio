all: build

include ../esp32.mk

build:
	make esp32-build-module
	make esp32-build-riot

run: build
	cargo run --bin runner esp32

test: build
	cargo run --bin runner esp32 4000

clean:
	rm -rf target
