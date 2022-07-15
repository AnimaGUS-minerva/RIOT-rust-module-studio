all: build-native

include ../native.mk
include ../esp32.mk

# native

build-native:
	make native-build-module
	make native-build-riot

run-native: build-native
	make native-run-riot

test-native: build-native
	EMU_TIMEOUT=4000  make native-run-riot

# esp32

build-esp32:
	make esp32-build-module
	make esp32-build-riot

run-esp32: build-esp32
	make esp32-run-riot

test-esp32: build-esp32
	EMU_TIMEOUT=4000  make esp32-run-riot

#

run: run-native

test:
	make test-native
	make test-esp32 || exit 0  # WIP

clean:
	rm -rf target ./riot/bin
