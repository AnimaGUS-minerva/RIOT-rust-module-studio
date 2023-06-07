#ifndef MINERVA_ESP_ETH_H
#define MINERVA_ESP_ETH_H

#include "net/netdev.h"

#ifdef __cplusplus
extern "C" {
#endif

int minerva_gnrc_esp_eth_init(netdev_t *device);

#ifdef __cplusplus
}
#endif

#endif /* MINERVA_ESP_ETH_H */
