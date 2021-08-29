all: build-native

include ../xtensa.mk

# native

RIOT_ELF := ./riot/bin/native/riot.elf
build-native:
	cargo +stable-i686-unknown-linux-gnu build --lib --release --target i686-unknown-linux-gnu
	cd ./riot && BOARD=native RIOTBASE=$(CURDIR)/../../RIOT make
	ldd $(RIOT_ELF)
	file $(RIOT_ELF)

run-native: build-native
	timeout 4 $(RIOT_ELF) || true

# esp32

build-esp32:
	make build-module
	make build-riot

run-esp32: build-esp32
	cargo run --bin emu || true

#

test:
	make build-native
	make run-native
	make run-esp32

clean:
	rm -rf target ./riot/bin
