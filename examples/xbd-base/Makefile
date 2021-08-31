all: build-native

include ../native.mk
include ../esp32.mk

# native

build-native:
	make native-build-module
	make native-build-riot

run-native: build-native
	timeout 4 ./riot/bin/native/riot.elf || true

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
