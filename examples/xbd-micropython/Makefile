all: build-native

include ../native.mk
include ../esp32.mk

# submodule

get-submodule:
	[ -e "micropython/.git" ] || \
		(git submodule init micropython && git submodule update)

sync-submodule:
	git submodule update --remote micropython && \
		cd micropython && git checkout 1.16-minerva && git pull

# build directory

CUSTOM_BOARD ?= native
ifeq ($(CUSTOM_BOARD),esp32)
    MP_BUILD_DIR := ./riot/bin/esp32-wroom-32/pkg/micropython
else ifeq ($(CUSTOM_BOARD),native)
	MP_BUILD_DIR := ./riot/bin/native/pkg/micropython
endif

MP_GENHDR_DIR = $(MP_BUILD_DIR)/genhdr
mp-build-dir: get-submodule
	mkdir -p $(MP_BUILD_DIR)
	[ -e $(MP_BUILD_DIR)/.git ] || \
		git -c advice.detachedHead=false clone micropython $(MP_BUILD_DIR)
	@# Kludge: prevent missing 'qstrdefs.generated.h' from breaking build
	if [ ! -d $(MP_GENHDR_DIR) ]; then \
		mkdir -p $(MP_GENHDR_DIR) && \
		touch $(MP_GENHDR_DIR)/qstrdefs.generated.h; \
	fi
	@# Kludge: crudely include custom modules...
	rsync -av ./riot/custom/* $(MP_BUILD_DIR)/ports/riot/

# FIXME wrong version tag is shown on starting repl...
# sed -i -E 's/(MICROPY_GIT_.*) (.*)/\1 "CUSTOM"/g' $(MP_GENHDR_DIR)/mpversion.h

# native

build-native:
	CUSTOM_BOARD=native make mp-build-dir
	make native-build-module
	CUSTOM_BOARD=native make native-build-riot

run-native: build-native
	make native-run-riot

test-native: build-native
	EMU_TIMEOUT=4000  make native-run-riot

# esp32

build-esp32:
	CUSTOM_BOARD=esp32 make mp-build-dir
	make esp32-build-module
	CUSTOM_BOARD=esp32 make esp32-build-riot

run-esp32: build-esp32
	make esp32-run-riot

test-esp32: build-esp32
	EMU_TIMEOUT=4000  make esp32-run-riot

# std

test-std-debug:
	cargo +nightly-x86_64-unknown-linux-gnu test --features std

#

run: run-native

test:
	make test-native
	make test-esp32
	make test-std-debug

clean:
	rm -rf target ./riot/bin
