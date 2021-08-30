all: build-native

include ../xtensa.mk

get-submodule:
	[ -e "micropython/.git" ] || \
		(git submodule init micropython && git submodule update)
sync-submodule:
	git submodule update --remote micropython && \
		cd micropython && git checkout 1.16-minerva && git pull

MP_BUILD_DIR = ./riot/bin/esp32-wroom-32/pkg/micropython
MP_GENHDR_DIR = $(MP_BUILD_DIR)/genhdr
build-dir: get-submodule
	mkdir -p $(MP_BUILD_DIR)
	[ -e $(MP_BUILD_DIR)/.git ] || \
		git -c advice.detachedHead=false clone micropython $(MP_BUILD_DIR)
	@# HACK: Prevent missing 'qstrdefs.generated.h' from stopping build
	if [ ! -d $(MP_GENHDR_DIR) ]; then \
		mkdir -p $(MP_GENHDR_DIR) && \
		touch $(MP_GENHDR_DIR)/qstrdefs.generated.h; \
	fi

build-esp32: build-dir
	make build-module
	CUSTOM_BOARD=ESP32 make build-riot

run-esp32: build-esp32
	cargo run --bin emu || true

test:
	make run-esp32

clean:
	rm -rf ./riot/bin
