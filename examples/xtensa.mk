SHELL := /bin/bash
RUST_MODULE_STUDIO := $(shell realpath ../..)

TOOLCHAIN_XTENSA := $(RUST_MODULE_STUDIO)/toolchain/xtensa

init-rust-xtensa:
	@if [ ! -d "$(TOOLCHAIN_XTENSA)/rustc" ]; then \
		echo "Setting up xtensa/rustc ..."; \
		(cd $(TOOLCHAIN_XTENSA); curl -O -L https://github.com/j-devel/demo/releases/download/assets-0.1/assets-0.1-rustc.tbz2; tar xfj assets-0.1-rustc.tbz2); \
		echo "Setting up xtensa/llvm ..."; \
		(cd $(TOOLCHAIN_XTENSA); curl -O -L https://github.com/j-devel/demo/releases/download/assets-0.1/assets-0.1-llvm.tbz2; tar xfj assets-0.1-llvm.tbz2); \
		rustup component add rustfmt; \
		rustup toolchain link xtensa $(TOOLCHAIN_XTENSA)/rustc/rust_build; \
		cargo install bindgen; \
		cargo install cargo-xbuild; \
	fi	

build-xtensa: init-rust-xtensa
	@#==== begin: compiler-builtins workaround
	@if [ ! -d "$(TOOLCHAIN_XTENSA)/xtensa-esp32-none-elf" ]; then \
		echo "Setting up pre-built xtensa-esp32-none-elf to workaround build errors of the compiler_builtins crate"; \
		(cd $(TOOLCHAIN_XTENSA); curl -O -L https://github.com/j-devel/demo/releases/download/assets-0.1/xtensa-esp32-none-elf--with-compiler-builtins-0.1.32.tbz2; tar xfj xtensa-esp32-none-elf--with-compiler-builtins-0.1.32.tbz2) \
	fi
	@mkdir -p target/sysroot/lib/rustlib && \
		cd target/sysroot/lib/rustlib && \
		ln -sf $(TOOLCHAIN_XTENSA)/xtensa-esp32-none-elf .
	@#==== end: compiler-builtins workaround
	RUST_MODULE_STUDIO=$(RUST_MODULE_STUDIO) source ../xtensa.setup && \
		cargo +xtensa xbuild --release --target xtensa-esp32-none-elf
	ls -lrt target/xtensa-esp32-none-elf/release/*.a

clean-xtensa:
	rm -rf $(TOOLCHAIN_XTENSA)/xtensa-esp32-none-elf
