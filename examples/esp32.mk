SHELL := /bin/bash
RUST_MODULE_STUDIO := $(shell realpath ../..)

TOOLCHAIN_XTENSA := $(RUST_MODULE_STUDIO)/toolchain/xtensa

esp32-build-module:
	@#==== begin: compiler-builtins workaround
	@if [ ! -d "$(TOOLCHAIN_XTENSA)/xtensa-esp32-none-elf" ]; then \
        echo "Setting up pre-built xtensa-esp32-none-elf to workaround build errors of the compiler_builtins crate"; \
        (cd $(TOOLCHAIN_XTENSA); curl -O -L https://github.com/AnimaGUS-minerva/RIOT-rust-module-studio/releases/download/assets-0.1/xtensa-esp32-none-elf--with-compiler-builtins-0.1.32.tbz2; tar xfj xtensa-esp32-none-elf--with-compiler-builtins-0.1.32.tbz2) \
        fi
	@mkdir -p target/sysroot/lib/rustlib && \
        cd target/sysroot/lib/rustlib && \
        ln -sf $(TOOLCHAIN_XTENSA)/xtensa-esp32-none-elf .
	@#==== end: compiler-builtins workaround
	RUST_MODULE_STUDIO=$(RUST_MODULE_STUDIO) source ../esp32.setup && \
        cargo +xtensa xbuild --lib --release --target xtensa-esp32-none-elf
	ls -lrt target/xtensa-esp32-none-elf/release/*.a

RIOT_PATH := $(TOOLCHAIN_XTENSA)/riot
RIOT_BASE ?= $(RUST_MODULE_STUDIO)/RIOT
esp32-build-riot:
	RUST_MODULE_STUDIO=$(RUST_MODULE_STUDIO) source ../esp32.setup && cd ./riot && \
		RIOT_PATH=${RIOT_PATH} RIOT_BASE=${RIOT_BASE} CONTINUE_ON_EXPECTED_ERRORS=1 \
		$(TOOLCHAIN_XTENSA)/riot/riot-build

esp32-clean:
	rm -rf $(TOOLCHAIN_XTENSA)/xtensa-esp32-none-elf
