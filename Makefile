SHELL := /bin/bash

# 'test' or 'ci'
TARGET ?= test
ci:
	TARGET=ci make test

init:
	make init-rust-xtensa

TOOLCHAIN_XTENSA := toolchain/xtensa
init-rust-xtensa:
	@if [ ! -d "$(TOOLCHAIN_XTENSA)/rustc" ]; then \
        echo "[1/4] Setting up xtensa/rustc ..."; \
        (cd $(TOOLCHAIN_XTENSA); curl -O -L https://github.com/j-devel/demo/releases/download/assets-0.1/assets-0.1-rustc.tbz2; tar xfj assets-0.1-rustc.tbz2); \
        echo "[2/4] Setting up xtensa/llvm ..."; \
        (cd $(TOOLCHAIN_XTENSA); curl -O -L https://github.com/j-devel/demo/releases/download/assets-0.1/assets-0.1-llvm.tbz2; tar xfj assets-0.1-llvm.tbz2); \
        echo "[3/4] Configuring rustc xtensa ..."; \
        rustup component add rustfmt; \
        rustup toolchain link xtensa $(TOOLCHAIN_XTENSA)/rustc/rust_build; \
        cargo install bindgen; \
        cargo install cargo-xbuild; \
    fi
	@echo "[4/4] Testing rustc xtensa ..."
	@RUST_MODULE_STUDIO=$(CURDIR) source ./examples/xtensa.setup && \
		if [[ `rustc +xtensa --version` =~ rustc.* ]]; then echo rustc xtensa version LGTM; else false; fi

NAMES := esp32-no_std
test:
	for name in $(NAMES); do \
		make -C ./examples/$$name test; done

