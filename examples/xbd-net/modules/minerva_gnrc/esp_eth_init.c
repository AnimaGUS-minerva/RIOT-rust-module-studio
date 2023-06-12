#include <stdio.h>
#include "esp_eth_params.h"
#include "net/gnrc/netif/ethernet.h"

#ifdef MINERVA_DEBUG_ETH_MANUAL
static char _esp_eth_stack[ESP_ETH_STACKSIZE];
static gnrc_netif_t _netif;
#endif

int minerva_gnrc_esp_eth_init(netdev_t *device) {
    puts("minerva_gnrc_esp_eth_init(): ^^");
#ifdef MINERVA_DEBUG_ETH_MANUAL
    // cf. 'RIOT/sys/net/gnrc/netif/init_devs/auto_init_esp_eth.c'
    printf("@@ &_netif: %p\n", &_netif);
    gnrc_netif_ethernet_create(
            &_netif, _esp_eth_stack, ESP_ETH_STACKSIZE, ESP_ETH_PRIO, "netif-esp-eth", device);
#else
    assert(0);
#endif

    return 0;
}
