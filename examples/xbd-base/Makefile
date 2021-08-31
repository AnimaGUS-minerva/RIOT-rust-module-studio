all: build-native

include ../native.mk
include ../xtensa.mk

# native

RIOT_ELF := ./riot/bin/native/riot.elf
build-native:
	make native-build-module
	make native-build-riot
	ldd $(RIOT_ELF) && file $(RIOT_ELF)

run-native: build-native
	timeout 4 $(RIOT_ELF) || true

# esp32

build-esp32:
	make esp32-build-module
	make esp32-build-riot

run-esp32: build-esp32
	cargo run --bin emu || true

#

test:
	make build-native
	make run-native
	make run-esp32

clean:
	rm -rf target ./riot/bin
