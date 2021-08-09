SHELL := /bin/bash

# 'test' or 'ci'
TARGET ?= test
ci:
	TARGET=ci make test

init:
	make init-rust-xtensa

init-rust-xtensa:
	@echo todo
	@RUST_MODULE_STUDIO=$(CURDIR) source ./examples/xtensa.setup && \
		if [[ `rustc +xtensa --version` =~ rustc.* ]]; then echo rustc xtensa version LGTM; else false; fi

NAMES := esp32-no_std
test:
	for name in $(NAMES); do \
		make -C ./examples/$$name test; done

