APPLICATION = riot
BOARD ?= native
RIOTBASE ?= $(CURDIR)/../../../RIOT

INCLUDES += -I$(CURDIR)/../include
ifeq ($(BOARD),native)
ARCHIVES += $(CURDIR)/../target/i686-unknown-linux-gnu/release/librustmod.a
else
ARCHIVES += $(CURDIR)/../target/xtensa-esp32-none-elf/release/librustmod.a
endif

include $(RIOTBASE)/Makefile.include
