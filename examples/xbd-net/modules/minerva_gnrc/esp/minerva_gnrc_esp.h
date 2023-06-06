#ifndef MINERVA_GNRC_ESP_H
#define MINERVA_GNRC_ESP_H

#include "net/netdev.h"

#ifdef __cplusplus
extern "C" {
#endif

int minerva_gnrc_esp_eth_init(netdev_t *device);

#ifdef __cplusplus
}
#endif

#endif /* MINERVA_GNRC_ESP_H */