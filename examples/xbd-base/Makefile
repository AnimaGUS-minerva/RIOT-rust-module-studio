all: build-native

include ../native.mk
include ../esp32.mk

# native

build-native:
	make native-build-module
	make native-build-riot

run-native: build-native
	cargo run --bin runner native

test-native: build-native
	cargo run --bin runner native 4000

# esp32

build-esp32:
	make esp32-build-module
	make esp32-build-riot

run-esp32: build-esp32
	cargo run --bin runner esp32

test-esp32: build-esp32
	cargo run --bin runner esp32 4000

#

run: run-native

test:
	make test-native
	make test-esp32

clean:
	rm -rf target ./riot/bin
