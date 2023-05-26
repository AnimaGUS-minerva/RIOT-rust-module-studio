APPLICATION = main
BOARD ?= esp32-ethernet-kit-v1_0
RIOTBASE ?= $(CURDIR)/../../../RIOT

MINERVA_NET_MODULE_DIR := $(CURDIR)/../modules
EXTERNAL_MODULE_DIRS += $(MINERVA_NET_MODULE_DIR)

DEBUG_ETH_MINIMAL ?= 0
ifeq ($(DEBUG_ETH_MINIMAL),0)
  USEMODULE += shell
  USEMODULE += shell_commands
  USEMODULE += ps
endif

INCLUDES += -I$(APPDIR)
INCLUDES += -I$(CURDIR)/../include# @@ rustmod

SHOULD_RUN_KCONFIG =?

#-------- @@ cf. https://github.com/gschorcht/RIOT_ESP_NOW_WiFi_Border_Router
# platform specific modules and settings
GNRC_NETIF_NUMOF := 2

# Include packages that pull up and auto-init the link layer.
# NOTE: 6LoWPAN will be included if IEEE802.15.4 devices are present
USEMODULE += gnrc_netdev_default

ifeq ($(BOARD),native)
  CFLAGS += -DMINERVA_BOARD_NATIVE

  # cf. https://github.com/RIOT-OS/RIOT/tree/master/cpu/native#setting-up-a-virtual-network
  USEMODULE += auto_init_gnrc_netif
else ifeq ($(BOARD),esp32-ethernet-kit-v1_0)
  CFLAGS += -DMINERVA_BOARD_ESP32

  #USEMODULE += esp_wifi
  #USEMODULE += esp_now
  USEMODULE += esp_eth
  FEATURES_REQUIRED += arch_esp32

  ifeq ($(DEBUG_ETH_MINIMAL),1)
    CFLAGS += -DMINERVA_DEBUG_ETH_MINIMAL
    # Use `minerva_netdev_eth_minimal_init()`
    USEMODULE += minerva_esp32_minimal
    INCLUDES += -I$(MINERVA_NET_MODULE_DIR)/minerva_esp32_minimal/include
  else
    # Use `minerva_netdev_eth_gnrc_init()`
    USEMODULE += minerva_esp32_gnrc
    INCLUDES += -I$(MINERVA_NET_MODULE_DIR)/minerva_esp32_gnrc/include
  endif
else
  $(error BOARD set is not supported)
endif

# Specify the mandatory networking modules for 6LoWPAN border router
USEMODULE += gnrc_sixlowpan_border_router_default
# Add forwarding table
USEMODULE += fib

#-------- Additional networking modules that can be dropped if not needed
USEMODULE += gnrc_icmpv6_echo
#==== native (using tap1), ok
# > ping fe80::2cca:caff:fe9f:13c5  # ping to linux tap1
#==== esp32 (using tap0/br0), ok
# > ping fe80::a00:27ff:fefd:b6f8  # ping to linux br0

USEMODULE += shell_cmd_gnrc_udp
#==== native (using tap1)
#---- linux-to-riot, ok
# > udp server start 8899
# $ nc -6uv fe80::2cca:caff:fe9f:13c6%tap1 8899  # send to riot udp server
#---- riot-to-linux, ok
# $ nc -6ul 8877  # linux udp server
# > udp send fe80::2cca:caff:fe9f:13c5 8877 hello  # send to linux tap1
#==== esp32 (using tap0/br0)
#---- linux-to-riot, ok
# > udp server start 8899
# $ nc -6uv fe80::200:ff:fe00:3%br0 8899  # send to riot udp server
#---- riot-to-linux, ok
# $ nc -6ul 8877  # linux udp server
# > udp send fe80::a00:27ff:fefd:b6f8 8877 hello  # send to linux br0
#--------

# Optionally include RPL as a routing protocol.
#USEMODULE += gnrc_rpl

# Comment this out to disable code in RIOT that does safety checking
# which is not needed in a production environment but helps in the
# development process:
DEVELHELP ?= 1

# Change this to 0 show compiler invocation lines by default:
QUIET ?= 1

# Prefix has to be /64
IPV6_PREFIX ?= 2001:db8::

# Outer address of the router and the length of its prefix if IPV6_AUTO=0
IPV6_AUTO ?= 1
#IPV6_AUTO ?= 0# @@
IPV6_ADDR ?= fd19:aaaa::1
IPV6_ADDR_LEN ?= 64
IPV6_DEF_RT ?= fd19:aaaa::2

# Pass as CFLAGS to program
CFLAGS += -DBR_IPV6_PREFIX=\"$(IPV6_PREFIX)\"
ifeq (0,$(IPV6_AUTO))
  CFLAGS += -DBR_IPV6_ADDR=\"$(IPV6_ADDR)\" -DBR_IPV6_ADDR_LEN=$(IPV6_ADDR_LEN) -DBR_IPV6_DEF_RT=\"$(IPV6_DEF_RT)\"
endif

# Might need more than the default of 2 addresses.
CFLAGS += -DGNRC_NETIF_IPV6_ADDRS_NUMOF=4
#-------- @@

include $(RIOTBASE)/Makefile.include
