SHELL := /bin/bash

# 'test' or 'ci'
TARGET ?= test
ci:
	make init
	TARGET=ci make test

init:
	git submodule init RIOT && git submodule update
	make init-rust-xtensa
	make init-qemu-xtensa
	make init-rust-toolchains

TOOLCHAIN_XTENSA := toolchain/xtensa

RUST_BUILD_MODULE := $(TOOLCHAIN_XTENSA)/rust-build
init-rust-xtensa:
	@echo "Configuring esptool ..."
	git submodule init $(TOOLCHAIN_XTENSA)/esptool && git submodule update
	@echo "Configuring rustc esp ..."
	git submodule init $(RUST_BUILD_MODULE) && git submodule update
	cd $(RUST_BUILD_MODULE) && ./install-rust-toolchain.sh --installation-mode reinstall
	@echo "Testing rustc esp ..."
	if [[ `rustc +esp --version` =~ rustc.* ]]; then \
       echo rustc esp version LGTM; else false; fi
	cargo install cargo-xbuild

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

NAMES := esp32-no_std xbd-base xbd-psa xbd-net xbd-py \
	native-sockets native-lwip
test:
	for name in $(NAMES); do \
        make -C ./examples/$$name test || exit 1; \
        done

