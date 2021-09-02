CFLAGS += -Wno-unused-parameter -Wno-error

CUSTOM_BOARD ?= native
ifeq ($(CUSTOM_BOARD),esp32)
    PKG_BUILD_DIR = $(CURDIR)/../bin/esp32-wroom-32/pkg/micropython
else ifeq ($(CUSTOM_BOARD),native)
    PKG_BUILD_DIR = $(CURDIR)/../bin/native/pkg/micropython
endif

PKG_SOURCE_DIR = $(PKG_BUILD_DIR)
all:
	BUILD=$(PKG_BUILD_DIR) "$(MAKE)" -C $(PKG_SOURCE_DIR)/ports/riot
