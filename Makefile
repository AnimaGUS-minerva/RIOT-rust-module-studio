SHELL := /bin/bash

# 'test' or 'ci'
TARGET ?= test
ci:
	TARGET=ci make test

init:
	make init-rust-xtensa
	make init-esp-idf
	make init-riot-xtensa
	make init-qemu-xtensa
	make init-rust-toolchains

TOOLCHAIN_XTENSA := toolchain/xtensa

RUST_BUILD_MODULE := $(TOOLCHAIN_XTENSA)/rust-build
init-rust-xtensa:
	git submodule init $(RUST_BUILD_MODULE)
	git submodule update
	@echo "Configuring rustc esp ..."
	cd $(RUST_BUILD_MODULE) && ./install-rust-toolchain.sh
	@echo "Testing rustc esp ..."
	@RUST_MODULE_STUDIO=$(CURDIR) source ./examples/esp32.setup && \
        if [[ `rustc +esp --version` =~ rustc.* ]]; then \
            echo rustc esp version LGTM; else false; \
            fi
	cargo install cargo-xbuild

IDF_MODULE := $(TOOLCHAIN_XTENSA)/esp-idf
init-esp-idf:
	git submodule init $(IDF_MODULE)
	git submodule update
	@#cd $(IDF_MODULE) && git submodule update --init --recursive
	@#====
	cd $(IDF_MODULE) && git submodule update --init components/esptool_py/esptool
	$(IDF_MODULE)/install.sh

XTENSA_ESP32_ELF_RIOT_TGZ := xtensa-esp32-elf-linux64-1.22.0-80-g6c4433a-5.2.0.tar.gz
init-riot-xtensa:
	git submodule init RIOT
	git submodule update
	@echo "Setting up xtensa-esp32-elf for RIOT per https://github.com/espressif/esp-at/issues/215#issuecomment-508597652"
	@if [ ! -e "$(TOOLCHAIN_XTENSA)/riot/$(XTENSA_ESP32_ELF_RIOT_TGZ)" ]; then \
        echo "Setting up xtensa/riot/xtensa-esp32-elf ..."; \
        (cd $(TOOLCHAIN_XTENSA)/riot; curl -O -L $(DL_ASSETS)/$(XTENSA_ESP32_ELF_RIOT_TGZ); tar xfz $(XTENSA_ESP32_ELF_RIOT_TGZ)); \
        fi
	@echo "Setting up esp-idf (f198339ec; v3.1) headers for RIOT per https://github.com/gschorcht/riotdocker-Xtensa-ESP/blob/master/Dockerfile"
	git clone $(IDF_MODULE) $(TOOLCHAIN_XTENSA)/riot/esp-idf
	cd $(TOOLCHAIN_XTENSA)/riot/esp-idf && \
        git checkout -q f198339ec09e90666150672884535802304d23ec

DL_ASSETS := https://github.com/AnimaGUS-minerva/RIOT-rust-module-studio/releases/download/assets-0.1

QEMU_XTENSA_TGZ := qemu-d558d21678-20210520.tgz
init-qemu-xtensa:
	@if [ ! -e "$(TOOLCHAIN_XTENSA)/qemu/$(QEMU_XTENSA_TGZ)" ]; then \
        echo "Setting up xtensa/qemu/qemu ..."; \
        (cd $(TOOLCHAIN_XTENSA)/qemu; curl -O -L $(DL_ASSETS)/$(QEMU_XTENSA_TGZ); tar xfz $(QEMU_XTENSA_TGZ)); \
        fi
	find $(TOOLCHAIN_XTENSA)/qemu

init-rust-toolchains:
	rustup toolchain install nightly-x86_64-unknown-linux-gnu
	rustup toolchain install nightly-i686-unknown-linux-gnu
	rustup target add x86_64-unknown-linux-gnu
	rustup target add i686-unknown-linux-gnu --toolchain nightly
	rustup default nightly
	rustup show

##NAMES := esp32-no_std xbd-base xbd-psa xbd-py \
##
NAMES := xbd-base xbd-psa xbd-py \
	native-sockets native-lwip
test:
	for name in $(NAMES); do \
        make -C ./examples/$$name test || exit 1; \
        done

