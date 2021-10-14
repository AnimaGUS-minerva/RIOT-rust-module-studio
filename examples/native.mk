SHELL := /bin/bash
RUST_MODULE_STUDIO := $(shell realpath ../..)

native-build-module:
	cargo +nightly-i686-unknown-linux-gnu build --lib --release \
		--target i686-unknown-linux-gnu $(CARGO_FEATURES)
	ls -lrt target/i686-unknown-linux-gnu/release/*.a

RIOT_ELF := ./riot/bin/native/riot.elf
native-build-riot:
	cd ./riot && BOARD=native RIOTBASE=$(RUST_MODULE_STUDIO)/RIOT make
	ldd $(RIOT_ELF) && file $(RIOT_ELF)

native-run-riot:
	cargo run --manifest-path ../runner/Cargo.toml native $(EMU_TIMEOUT)
