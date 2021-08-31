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
	make init-rust-i686

TOOLCHAIN_XTENSA := toolchain/xtensa

DL_ASSETS := https://github.com/AnimaGUS-minerva/RIOT-rust-module-studio/releases/download/assets-0.1

RUSTC_TBZ2 := assets-0.1-rustc.tbz2
LLVM_TBZ2 := assets-0.1-llvm.tbz2
init-rust-xtensa:
	@if [ ! -d "$(TOOLCHAIN_XTENSA)/rustc" ]; then \
        echo "[1/4] Setting up xtensa/rustc ..."; \
        (cd $(TOOLCHAIN_XTENSA); curl -O -L $(DL_ASSETS)/$(RUSTC_TBZ2); tar xfj $(RUSTC_TBZ2)); \
        echo "[2/4] Setting up xtensa/llvm ..."; \
        (cd $(TOOLCHAIN_XTENSA); curl -O -L $(DL_ASSETS)/$(LLVM_TBZ2); tar xfj $(LLVM_TBZ2)); \
        echo "[3/4] Configuring rustc xtensa ..."; \
        rustup component add rustfmt; \
        rustup toolchain link xtensa $(TOOLCHAIN_XTENSA)/rustc/rust_build; \
        cargo install bindgen; \
        cargo install cargo-xbuild; \
        fi
	@echo "[4/4] Testing rustc xtensa ..."
	@RUST_MODULE_STUDIO=$(CURDIR) source ./examples/esp32.setup && \
        if [[ `rustc +xtensa --version` =~ rustc.* ]]; then \
            echo rustc xtensa version LGTM; else false; \
            fi

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

QEMU_XTENSA_TGZ := qemu-d558d21678-20210520.tgz
init-qemu-xtensa:
	@if [ ! -e "$(TOOLCHAIN_XTENSA)/qemu/$(QEMU_XTENSA_TGZ)" ]; then \
        echo "Setting up xtensa/qemu/qemu ..."; \
        (cd $(TOOLCHAIN_XTENSA)/qemu; curl -O -L $(DL_ASSETS)/$(QEMU_XTENSA_TGZ); tar xfz $(QEMU_XTENSA_TGZ)); \
        fi
	find $(TOOLCHAIN_XTENSA)/qemu

init-rust-i686:
	rustup toolchain install stable-i686-unknown-linux-gnu
	rustup target add i686-unknown-linux-gnu

NAMES := esp32-no_std native-sockets xbd-base xbd-micropython
test:
	for name in $(NAMES); do \
        make -C ./examples/$$name test; \
        done

