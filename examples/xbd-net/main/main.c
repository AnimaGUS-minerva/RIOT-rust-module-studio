/*
 * Copyright (C) 2022 HAW Hamburg
 *
 * This file is subject to the terms and conditions of the GNU Lesser
 * General Public License v2.1. See the file LICENSE in the top level
 * directory for more details.
 */

/**
 * @ingroup     tests
 * @{
 *
 * @file
 * @brief       Test application for ESP ethernet peripheral
 *
 * @author      Leandro Lanzieri <leandro.lanzieri@haw-hamburg.de>
 *
 * @}
 */

#include <stdio.h>

#include "shell.h"
#ifdef MINERVA_BOARD_ESP32
#include "netdev_eth_minimal.h"
#include "init_dev.h"
#include "assert.h"
#include "net/netdev.h"
#include "esp_eth_netdev.h"
#include "esp_eth_params.h"
#endif

#ifdef WIP_ADHOC_GNRC//--------@@ cf. https://github.com/gschorcht/RIOT_ESP_NOW_WiFi_Border_Router
//@@#include <stdio.h>

#include <net/gnrc/ipv6/nib.h>
#include <net/gnrc/ipv6.h>
#include <net/gnrc/netapi.h>
#include <net/gnrc/netif.h>
#ifdef MODULE_GNRC_RPL
#include <net/gnrc/rpl.h>
#endif
#include <net/ethernet.h>
#include <net/ipv6/addr.h>
//@@#include <net/netdev.h>
#include <net/netopt.h>
#include <xtimer.h>
//@@#include <shell.h>
#include <msg.h>
#endif//--------@@

#ifdef MINERVA_BOARD_ESP32
extern void esp_eth_setup(esp_eth_netdev_t* dev);
extern esp_eth_netdev_t _esp_eth_dev;
#endif

#ifdef MINERVA_BOARD_ESP32
#ifdef WIP_ADHOC_GNRC//--------@@
#include "net/gnrc/netif/ethernet.h"

/** statically allocated memory for the MAC layer thread */
static char _esp_eth_stack[ESP_ETH_STACKSIZE];

static gnrc_netif_t _netif;

//@@ todo?? refactor with `auto_init_esp_eth()` ?? RIOT/sys/net/gnrc/netif/init_devs/auto_init_esp_eth.c
int netdev_eth_gnrc_init_devs(void) { // @@
    netdev_t *device = &_esp_eth_dev.netdev;

    printf("@@ &_netif: %p\n", &_netif);

    // cf. 'RIOT/sys/net/gnrc/netif/init_devs/auto_init_esp_eth.c'
    esp_eth_setup(&_esp_eth_dev);
    gnrc_netif_ethernet_create(&_netif, _esp_eth_stack, ESP_ETH_STACKSIZE, ESP_ETH_PRIO,
                               "netif-esp-eth", device);

    return 0;
}
#else//--------@@
int netdev_eth_minimal_init_devs(netdev_event_cb_t cb) {
    netdev_t *device = &_esp_eth_dev.netdev;

    /* setup the specific driver */
    esp_eth_setup(&_esp_eth_dev);

    /* set the application-provided callback */
    device->event_callback = cb;

    /* initialize the device driver */
    int res = device->driver->init(device);
    puts(res == 0 ? "ok" : "oh no"); // @@
    assert(!res);

    return 0;
}
#endif//--------@@
#endif//MINERVA_BOARD_ESP32


#ifdef WIP_ADHOC_GNRC//--------@@
static msg_t main_msg_queue[16];

static gnrc_netif_t *outer_interface = NULL;
static gnrc_netif_t *inner_interface = NULL;

static int find_interfaces(void)
{
    uint16_t mtu;
    gnrc_netif_t *netif = NULL;

    outer_interface = inner_interface = NULL;

    while ((netif = gnrc_netif_iter(netif))) {
        printf("@@ (found) netif: %p\n", (void *)netif);
        gnrc_netapi_get(netif->pid, NETOPT_MAX_PDU_SIZE, 0, &mtu, sizeof(mtu));
        printf("@@ mtu: %d (ETHERNET_DATA_LEN=%d)\n", mtu, ETHERNET_DATA_LEN);

        if (!outer_interface && (mtu == ETHERNET_DATA_LEN)) {
            outer_interface = netif;
        } else if (!inner_interface && (mtu != ETHERNET_DATA_LEN)) {
            inner_interface = netif;
        }

        if (outer_interface && inner_interface)
            break;
    }
    printf("@@ (native|esp-eth|esp-wifi) outer_interface: %p\n", (void *)outer_interface);
    printf("@@ (esp-now) inner_interface: %p\n", (void *)inner_interface);

    if (outer_interface) { // @@
        ipv6_addr_t addrs[GNRC_NETIF_IPV6_ADDRS_NUMOF];

        printf("@@ GNRC_NETIF_IPV6_ADDRS_NUMOF: %d\n", GNRC_NETIF_IPV6_ADDRS_NUMOF); // @@ via Makefile
        gnrc_netapi_get(outer_interface->pid, NETOPT_IPV6_ADDR, 0, &addrs, sizeof(addrs));

        char addrstr[IPV6_ADDR_MAX_STR_LEN];
        printf("@@ addrs[0]: %s\n", ipv6_addr_to_str(addrstr, &addrs[0], sizeof(addrstr)));
#if defined(MINERVA_BOARD_NATIVE)
        printf("@@ hint - for `native` board, try `ping6 %s%%tap1` in a new shell\n", addrstr);
#elif defined(MINERVA_BOARD_ESP32)
        printf("@@ hint - for `esp32` board, try `ping6 %s%%br0` in a new shell\n", addrstr);
#endif
    }

    // @@ ignore `inner_interface`
//    if (!outer_interface || !inner_interface) {
//        printf("Unable to find interfaces.\n");
//        return -1;
//    }

    return 0;
}

#ifdef MINERVA_BOARD_ESP32
static int set_ips(void)
{
#if defined(BR_IPV6_ADDR) && defined(BR_IPV6_ADDR_LEN)
    /* Add configured outer address */
    ipv6_addr_t addr;
    ipv6_addr_from_str(&addr, BR_IPV6_ADDR);
    if (gnrc_netif_ipv6_addr_add(outer_interface, &addr, BR_IPV6_ADDR_LEN, 0) < 0) {
        printf("Failed setting outer address.\n");
        return -1;
    }
#endif

#ifdef BR_IPV6_DEF_RT
    /* Add default route */
    ipv6_addr_t defroute = IPV6_ADDR_UNSPECIFIED;
    ipv6_addr_from_str(&addr, BR_IPV6_DEF_RT);
    if (gnrc_ipv6_nib_ft_add(&defroute, 0, &addr, outer_interface->pid, 0) < 0) {
        printf("Failed setting default route.\n");
        return -1;
    }
#endif

    /* Turn off router advert on outer interface, it's really not our job. */
    gnrc_ipv6_nib_change_rtr_adv_iface(outer_interface, false);

    /* Add inner address based on prefix and interface iid */
    // @@ ignore `inner_interface`
//    eui64_t iid;
//    ipv6_addr_t prefix;
//    ipv6_addr_from_str(&prefix, BR_IPV6_PREFIX);
//    if (gnrc_netapi_get(inner_interface->pid, NETOPT_IPV6_IID, 0, &iid, sizeof(iid)) < 0) {
//        printf("Failed getting wireless interface iid.\n");
//        return -1;
//    }
//    ipv6_addr_set_aiid(&prefix, iid.uint8);
//    if (gnrc_netif_ipv6_addr_add(inner_interface, &prefix, 64, 0) < 0) {
//        printf("Failed setting outer address.\n");
//        return -1;
//    }

#if GNRC_IPV6_NIB_CONF_MULTIHOP_P6C
    /* Add as authoritative border router */
    if (gnrc_ipv6_nib_abr_add(&prefix) < 0) {
        printf("Failed adding prefix as authoritative border router.\n");
        return -1;
    }
#endif

#ifdef MODULE_GNRC_RPL
    /* Configure rpl */
    if (gnrc_rpl_init(inner_interface->pid) < 0) {
        printf("Failed initializing RPL on inner wireless interface.\n");
        return -1;
    }
    gnrc_rpl_instance_t *inst = gnrc_rpl_instance_get(GNRC_RPL_DEFAULT_INSTANCE);
    if (inst) {
        gnrc_rpl_instance_remove(inst);
    }
    if (!gnrc_rpl_root_init(GNRC_RPL_DEFAULT_INSTANCE, &prefix, false, false)) {
        printf("Failed initializing RPL root.\n");
        return -1;
    }
#endif

    return 0;
}
#endif//MINERVA_BOARD_ESP32

#endif//--------@@

int main(void)
{
    puts("Test application for ESP ethernet peripheral");

#ifdef MINERVA_BOARD_ESP32
#ifdef WIP_ADHOC_GNRC
    int res = netdev_eth_gnrc_init_devs();
#else
    int res = netdev_eth_minimal_init();
#endif
    if (res) {
        puts("Error initializing devices");
        return 1;
    }
#endif//MINERVA_BOARD_ESP32

#ifdef WIP_ADHOC_GNRC//--------@@
    /* we need a message queue for the thread running the shell in order to
     * receive potentially fast incoming networking packets */
    msg_init_queue(main_msg_queue, sizeof(main_msg_queue) / sizeof(main_msg_queue[0]));
    puts("RIOT border router example application");

    if (find_interfaces() >= 0) {
#ifdef MINERVA_BOARD_ESP32
        set_ips();
#endif
    }
#endif//--------@@

    /* start the shell */
    puts("Initialization successful - starting the shell now");

    char line_buf[SHELL_DEFAULT_BUFSIZE];
    shell_run(NULL, line_buf, SHELL_DEFAULT_BUFSIZE);

    return 0;
}
