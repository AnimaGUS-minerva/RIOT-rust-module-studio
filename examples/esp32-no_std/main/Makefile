APPLICATION = main
BOARD ?= native
RIOTBASE ?= $(CURDIR)/../../../RIOT

INCLUDES += -I$(CURDIR)/../include
# esp_idf_ver.h
INCLUDES += -I$(RIOTBASE)/pkg/esp32_sdk/esp-idf/components
ARCHIVES += $(CURDIR)/../target/xtensa-esp32-none-elf/release/librustmod.a

include $(RIOTBASE)/Makefile.include
