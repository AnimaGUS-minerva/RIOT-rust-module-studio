CFLAGS += -Wno-unused-parameter -Wno-error

ifeq ($(CUSTOM_BOARD),ESP32)
    PKG_BUILD_DIR = $(CURDIR)/../bin/esp32-wroom-32/pkg/micropython
else ifeq ($(CUSTOM_BOARD),NATIVE)
    PKG_BUILD_DIR = $(CURDIR)/../bin/native/pkg/micropython
else
    PKG_BUILD_DIR = $(CURDIR)/../bin/native/pkg/micropython
endif

PKG_SOURCE_DIR = $(PKG_BUILD_DIR)
all:
	BUILD=$(PKG_BUILD_DIR) "$(MAKE)" -C $(PKG_SOURCE_DIR)/ports/riot
