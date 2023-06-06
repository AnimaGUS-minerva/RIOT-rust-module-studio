typedef int workaround_empty_translation_unit;
#ifdef MINERVA_BOARD_ESP32_ETH

#include <stdio.h>
#include "esp_eth_params.h"
#include "net/gnrc/netif/ethernet.h"

/** statically allocated memory for the MAC layer thread */
static char _esp_eth_stack[ESP_ETH_STACKSIZE];

static gnrc_netif_t _netif;

int minerva_netdev_eth_gnrc_init(netdev_t *device) {
    // cf. 'RIOT/sys/net/gnrc/netif/init_devs/auto_init_esp_eth.c'
    printf("@@ &_netif: %p\n", &_netif);
    gnrc_netif_ethernet_create(
            &_netif, _esp_eth_stack, ESP_ETH_STACKSIZE, ESP_ETH_PRIO, "netif-esp-eth", device);

    return 0;
}

#endif//MINERVA_BOARD_ESP32_ETH