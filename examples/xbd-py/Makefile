all: build-native

include ../native.mk
include ../esp32.mk

# submodules

get-python-voucher:
	[ -e "python-rfc8366-voucher/.git" ] || \
        (git submodule init python-rfc8366-voucher && git submodule update)
sync-python-voucher:
	git submodule update --remote python-rfc8366-voucher && \
        cd python-rfc8366-voucher && git checkout main && git pull
get-voucher: get-python-voucher
	[ -e "python-rfc8366-voucher/voucher/.git" ] || \
        (cd python-rfc8366-voucher && git submodule init voucher && git submodule update)
PYTHON_VOUCHER_REPO := $(CURDIR)/python-rfc8366-voucher

micropython: get-micropython
get-micropython:
	[ -e "micropython/.git" ] || \
		(git submodule init micropython && git submodule update)
sync-micropython:
	git submodule update --remote micropython && \
		cd micropython && git checkout v1.17-riot-voucher && git pull
MP_PORTS_RIOT_DIR := $(CURDIR)/micropython/ports/riot

RIOT_RELATIVE_PATH := ../../../../../RIOT

# native

build-native: get-voucher micropython
	CARGO_OPTS="--manifest-path $(PYTHON_VOUCHER_REPO)/voucher/if/Cargo.toml"  make native-build-module
	ln -sf $(RIOT_RELATIVE_PATH) $(MP_PORTS_RIOT_DIR)/
	PYTHON_VOUCHER_REPO=$(PYTHON_VOUCHER_REPO)  make -C $(MP_PORTS_RIOT_DIR)

RIOT_NATIVE_ELF := $(MP_PORTS_RIOT_DIR)/main/bin/native/main.elf

run-native: build-native
	RIOT_NATIVE_ELF=$(RIOT_NATIVE_ELF)  make native-run-riot

test-native:
	EMU_TIMEOUT=4000  make run-native

# esp32

build-esp32: get-voucher micropython
	CARGO_OPTS="--manifest-path $(PYTHON_VOUCHER_REPO)/voucher/if/Cargo.toml"  make esp32-build-module
	ln -sf $(RIOT_RELATIVE_PATH) $(MP_PORTS_RIOT_DIR)/
	RIOT_ESP32_BIN=$(CURDIR)/main.esp32.bin \
		RIOT_ESP32_ELF=$(MP_PORTS_RIOT_DIR)/main/bin/esp32-wroom-32/main.elf \
		PYTHON_VOUCHER_REPO=$(PYTHON_VOUCHER_REPO) \
		CUSTOM_BOARD=esp32 SRC_DIR=$(MP_PORTS_RIOT_DIR)  make esp32-build-riot

run-esp32: build-esp32
	RIOT_ESP32_ELF=$(MP_PORTS_RIOT_DIR)/main/bin/esp32-wroom-32/main.elf \
		make esp32-run-riot

test-esp32:
	EMU_TIMEOUT=16000  make run-esp32

# misc.

test-python3-mbedtls2:
	make test -C python3/mbedtls2  # dev/debug

#

run: run-native

test:
	make clean && make test-native
	make clean && make test-esp32
	make test-python3-mbedtls2

clean:
	rm -rf target micropython
