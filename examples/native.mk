SHELL := /bin/bash
RUST_MODULE_STUDIO := $(shell realpath ../..)

native-build-module:
	cargo +stable-i686-unknown-linux-gnu build --lib --release --target i686-unknown-linux-gnu
	ls -lrt target/i686-unknown-linux-gnu/release/*.a

native-build-riot:
	cd ./riot && BOARD=native RIOTBASE=$(RUST_MODULE_STUDIO)/RIOT make
