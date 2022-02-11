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

# native

build-native: micropython clone-RIOT
	CARGO_FEATURES="--features 'x86-lts'"  make native-build-module
	ln -sf ../../../RIOT ./micropython/ports/riot/
	make -C ./micropython/ports/riot

RIOT_NATIVE_ELF := ./micropython/ports/riot/main/bin/native/main.elf

run-native: build-native
	RIOT_NATIVE_ELF=$(RIOT_NATIVE_ELF)  make native-run-riot

test-native:
	EMU_TIMEOUT=4000  make run-native

# esp32

build-esp32: micropython clone-RIOT
	CARGO_FEATURES="--features 'xtensa-lts'"  make esp32-build-module
	ln -sf ../../../RIOT ./micropython/ports/riot/
	RIOT_BASE=$(CURDIR)/micropython/ports/riot/RIOT \
		make esp32-build-riot-micropython

run-esp32: build-esp32
	make esp32-run-riot

test-esp32:
	EMU_TIMEOUT=16000  make run-esp32

# std

test-std-lts:
	cargo +nightly-x86_64-unknown-linux-gnu test --features x86_64-lts
test-std:
	cargo +nightly-x86_64-unknown-linux-gnu test --features x86_64

#

run: run-native

test:
	make test-native
	##make clean && make test-esp32
	make test-std-lts
	make test-std

clean:
	rm -rf target micropython
