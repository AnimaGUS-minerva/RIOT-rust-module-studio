SHELL := /bin/bash

# 'test' or 'ci'
TARGET ?= test
ci--:
	make init
	TARGET=ci make test
ci:#!!!!WIP
	make init
	make ci-fixture-net
	make -C ./examples/xbd-net test

ci-fixture-net:
	#---- tap0/br0 for board `esp32`
	sudo ip link add br0 type bridge
	##N/A##sudo ip addr flush dev $(ETH_IF)
	##N/A##sudo ip link set $(ETH_IF) master br0
	sudo ip tuntap add dev tap0 mode tap user $$(whoami)
	sudo ip link set tap0 master br0
	sudo ip link set dev br0 up
	sudo ip link set dev tap0 up
	#---- tap1 for board `native`
	sudo ip tuntap add dev tap1 mode tap user $$(whoami)
	sleep 1 && sudo ip link set tap1 down
	sleep 1 && sudo ip link set tap1 up
	#---- check
	ip a && brctl show

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

