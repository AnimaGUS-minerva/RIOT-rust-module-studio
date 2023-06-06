MODULE = minerva_gnrc

SRC = border_router.c

ifneq (,$(filter minerva_gnrc_esp,$(FEATURES_USED)))
  DIRS += esp
endif

include $(RIOTBASE)/Makefile.base