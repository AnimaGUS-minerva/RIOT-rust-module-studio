all: build

include ../esp32.mk

build:
	make esp32-build-module
	make esp32-build-riot

run: build
	make esp32-run-riot

test: build
	EMU_TIMEOUT=4000  make esp32-run-riot

clean:
	rm -rf target
