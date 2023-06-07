MODULE = minerva_gnrc

SRC = border_router.c
ifneq (,$(filter esp_eth,$(FEATURES_USED)))
  SRC += esp_eth_init.c
endif

include $(RIOTBASE)/Makefile.base