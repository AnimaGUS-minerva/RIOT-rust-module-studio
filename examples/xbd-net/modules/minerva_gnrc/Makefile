MODULE = minerva_gnrc

SRC = border_router.c
ifneq (,$(filter esp_eth,$(FEATURES_USED)))
  SRC += esp_eth_init.c
endif

##

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

##

include $(RIOTBASE)/Makefile.base