all: build-native

include ../native.mk
include ../xtensa.mk

get-submodule:
	[ -e "micropython/.git" ] || \
		(git submodule init micropython && git submodule update)
sync-submodule:
	git submodule update --remote micropython && \
		cd micropython && git checkout 1.16-minerva && git pull

# build directory

CUSTOM_BOARD ?= NATIVE
ifeq ($(CUSTOM_BOARD),ESP32)
    MP_BUILD_DIR := ./riot/bin/esp32-wroom-32/pkg/micropython
else ifeq ($(CUSTOM_BOARD),NATIVE)
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

# native

RIOT_ELF := ./riot/bin/native/riot.elf
build-native:
	CUSTOM_BOARD=NATIVE make mp-build-dir
	make native-build-module
	CUSTOM_BOARD=NATIVE make native-build-riot
	ldd $(RIOT_ELF) && file $(RIOT_ELF)

run-native: build-native
	# !!!!!!!! timeout 4 $(RIOT_ELF) || true # processing hangs unless; sudo kill -9 zzzz
	$(RIOT_ELF) || true

# esp32

build-esp32:
	CUSTOM_BOARD=ESP32 make mp-build-dir
	make esp32-build-module
	CUSTOM_BOARD=ESP32 make esp32-build-riot

run-esp32: build-esp32
	cargo run --bin emu || true

test:
	#make run-native
	make build-native
	make run-esp32

clean:
	rm -rf ./riot/bin
