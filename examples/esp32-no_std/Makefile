all: build

include ../esp32.mk

build:
	make esp32-build-module
	make esp32-build-riot

run: build
	cargo run --manifest-path ../runner/Cargo.toml esp32

test: build
	cargo run --manifest-path ../runner/Cargo.toml esp32 4000

clean:
	rm -rf target
