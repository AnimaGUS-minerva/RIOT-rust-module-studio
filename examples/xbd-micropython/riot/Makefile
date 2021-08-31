# name of your application
APPLICATION = riot

# If no BOARD is found in the environment, use this default:
BOARD ?= native

# This has to be the absolute path to the RIOT base directory:
RIOTBASE ?= $(CURDIR)/../..

#==== @@
USEMODULE += micropython
EXTERNAL_MODULE_DIRS += $(CURDIR)/micropython

ifeq ($(CUSTOM_BOARD),ESP32)
    CFLAGS += -DCUSTOM_BOARD_ESP32
endif

CFLAGS += -DCUSTOM_USE_LIBFOO
INCLUDES += -I$(CURDIR)/../include
ifeq ($(CUSTOM_BOARD),ESP32)
    ARCHIVES += $(CURDIR)/../target/xtensa-esp32-none-elf/release/libfoo.a
else ifeq ($(CUSTOM_BOARD),NATIVE)
    ARCHIVES += $(CURDIR)/../target/i686-unknown-linux-gnu/release/libfoo.a
else
    ARCHIVES += $(CURDIR)/../target/i686-unknown-linux-gnu/release/libfoo.a
endif
#====

# include boot.py as header
BLOBS += boot.py

# configure MicroPython's heap size
MP_RIOT_HEAPSIZE ?= 8192U

# MicroPython needs a larger stack
CFLAGS += '-DTHREAD_STACKSIZE_MAIN=THREAD_STACKSIZE_DEFAULT*4'

# use miniterm (instead of the default pyterm) in order to support control
# characters (CTRL-D ...)
RIOT_TERMINAL ?= miniterm

# enable modmachine support for peripherals if available
FEATURES_OPTIONAL += periph_adc
FEATURES_OPTIONAL += periph_spi

# For now `examples/%/tests" still rely on the test applicaton being reset after
# a terminal is opened to synchronize.
TESTRUNNER_RESET_AFTER_TERM ?= 1

include $(RIOTBASE)/Makefile.include
