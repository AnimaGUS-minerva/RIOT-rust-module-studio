all: build

include ../xtensa.mk

build:
	make build-module
	make build-riot

test: build

clean:
	rm -rf target
