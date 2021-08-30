CFLAGS += -Wno-unused-parameter -Wno-error

PKG_BUILD_DIR = $(CURDIR)/../bin/esp32-wroom-32/pkg/micropython
PKG_SOURCE_DIR = $(PKG_BUILD_DIR)
all:
	BUILD=$(PKG_BUILD_DIR) "$(MAKE)" -C $(PKG_SOURCE_DIR)/ports/riot
