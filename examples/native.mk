SHELL := /bin/bash
RUST_MODULE_STUDIO := $(shell realpath ../..)

native-build-module:
	@echo "Building 'target/i686-unknown-linux-gnu/release/*.a'"
	cargo build --lib --release --target i686-unknown-linux-gnu $(CARGO_OPTS)

RIOT_NATIVE_ELF ?= ./main/bin/native/main.elf
native-build-riot:
	cd ./main && BOARD=native RIOTBASE=$(RUST_MODULE_STUDIO)/RIOT make
	ldd $(RIOT_NATIVE_ELF) && file $(RIOT_NATIVE_ELF)

native-run-riot:
	RIOT_NATIVE_ELF=$(RIOT_NATIVE_ELF) \
		cargo run --manifest-path ../runner/Cargo.toml native $(EMU_TIMEOUT)
