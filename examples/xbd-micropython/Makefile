all: build-native

include ../native.mk
include ../esp32.mk

# TODO bump RIOT version across the upper layers
RIOT_TAG := 2021.10
clone-RIOT:
	@if [ ! -e RIOT ]; then \
		git clone https://github.com/RIOT-OS/RIOT RIOT; \
		cd RIOT && git checkout -q $(RIOT_TAG); \
	fi

# submodule

micropython: get-submodule

get-submodule:
	[ -e "micropython/.git" ] || \
		(git submodule init micropython && git submodule update)

sync-submodule:
	git submodule update --remote micropython && \
		cd micropython && git checkout v1.17-riot-voucher && git pull

MP_PORTS_RIOT_DIR := $(CURDIR)/micropython/ports/riot

# native

build-native: micropython clone-RIOT
	CARGO_FEATURES="--features 'x86'"  make native-build-module
	ln -sf ../../../RIOT $(MP_PORTS_RIOT_DIR)/
	make -C $(MP_PORTS_RIOT_DIR)

RIOT_NATIVE_ELF := $(MP_PORTS_RIOT_DIR)/main/bin/native/main.elf

run-native: build-native
	RIOT_NATIVE_ELF=$(RIOT_NATIVE_ELF)  make native-run-riot

test-native:
	EMU_TIMEOUT=4000  make run-native

# esp32

build-esp32: micropython clone-RIOT
	CARGO_FEATURES="--features 'xtensa'"  make esp32-build-module
	ln -sf ../../../RIOT $(MP_PORTS_RIOT_DIR)/
	RIOT_BASE=$(MP_PORTS_RIOT_DIR)/RIOT \
		RIOT_ESP32_IMG=$(CURDIR)/riot.esp32.bin \
		RIOT_ESP32_ELF=$(MP_PORTS_RIOT_DIR)/main/bin/esp32-wroom-32/main.elf \
		make esp32-build-riot-micropython

run-esp32: build-esp32
	make esp32-run-riot

test-esp32:
	EMU_TIMEOUT=16000  make run-esp32

# misc.

test-std:
	cargo +nightly-x86_64-unknown-linux-gnu test --features x86_64
test-i686:
	cargo test --target i686-unknown-linux-gnu --features x86

#

run: run-native

test:
	make clean && make test-native
	##TODO##make clean && make test-esp32
	make test-std
	make test-i686

clean:
	rm -rf target micropython
