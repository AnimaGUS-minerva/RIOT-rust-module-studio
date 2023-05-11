SHELL := /bin/bash
STUDIO := $(shell realpath ../..)

TOOLCHAIN_XTENSA := $(STUDIO)/toolchain/xtensa

#==== clang esp from 'toolchain/xtensa/rust-build'
CLANG_ESP := $(HOME)/.espressif/tools/xtensa-esp32-elf-clang/esp-13.0.0-20211203-x86_64-unknown-linux-gnu
#==== clang esp from https://github.com/gschorcht/riotdocker-Xtensa-ESP/blob/master/riotbuild/Dockerfile
#CLANG_ESP :=
#====

export CARGO_HOME := $(STUDIO)/target/cargo
export LIBCLANG_PATH := $(CLANG_ESP)/lib
export XARGO_RUST_SRC := $(TOOLCHAIN_XTENSA)/rust-build/rust-src-1.57.0.2/rust-src/lib/rustlib/src/rust/library
export PATH := $(CLANG_ESP)/bin:$(PATH)

esp32-build-module:
	@echo "Buidling 'target/xtensa-esp32-none-elf/release/*.a'"
	XTENSA_GCC=$(CLANG_ESP)/bin/xtensa-esp32-elf-gcc \
		cargo +esp xbuild --lib --release --target xtensa-esp32-none-elf $(CARGO_OPTS)

RIOT_BASE ?= $(STUDIO)/RIOT
RIOT_BOARD ?= esp32-wroom-32
SRC_DIR ?= ./main
esp32-build-riot:
	CONTINUE_ON_EXPECTED_ERRORS=1 WERROR=0 BOARD=$(RIOT_BOARD) \
		RIOTBASE=$(RIOT_BASE) make -C $(SRC_DIR)

RIOT_ESP32_BIN ?= ./main.esp32.bin
RIOT_ESP32_ELF ?= ./main/bin/$(RIOT_BOARD)/main.elf
esp32-build-bin:
	python3 $(TOOLCHAIN_XTENSA)/esptool/esptool.py --chip esp32 elf2image \
		-o $(RIOT_ESP32_BIN) $(RIOT_ESP32_ELF)

EMU_TIMEOUT ?= 0
EMU_ESP32_NIC ?= user,model=open_eth,id=lo
esp32-run-riot: esp32-build-bin
	RIOT_ESP32_BIN=$(RIOT_ESP32_BIN) \
		cargo run --manifest-path ../runner/Cargo.toml esp32 $(EMU_TIMEOUT) $(EMU_ESP32_NIC)
