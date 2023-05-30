
// TODO !!!! test esp-{wifi,now} on MCU
// TODO refactor BR_* constants into 'minerva_esp32_gnrc/Makefile'

//--------@@
#include <stdio.h>
// TODO cleanup
//#include <net/gnrc/ipv6/nib.h>
//??#include <net/gnrc/ipv6.h>
//#include <net/gnrc/netapi.h>
#include <net/gnrc/netif.h>
//#ifdef MODULE_GNRC_RPL
//  #include <net/gnrc/rpl.h>
//#endif

#include <net/ethernet.h>
#include <net/ipv6/addr.h>
#include <net/netopt.h>

#include <xtimer.h>
#include <shell.h>
#include <msg.h>
//--------@@

#if defined(MINERVA_BOARD_ESP32_ETH) || defined(MINERVA_BOARD_ESP32_WROOM32)
#ifndef MINERVA_DEBUG_ETH_MINIMAL
  #include "border_router.h"
#endif
#endif

#ifdef MINERVA_BOARD_ESP32_ETH
#ifdef MINERVA_DEBUG_ETH_MINIMAL
  #include "netdev_eth_minimal.h"
  #define MINERVA_NETDEV_ETH_INIT minerva_netdev_eth_minimal_init
#else
  #include "netdev_eth_gnrc.h"
  #define MINERVA_NETDEV_ETH_INIT minerva_netdev_eth_gnrc_init
#endif

#include "esp_eth_netdev.h"
extern esp_eth_netdev_t _esp_eth_dev;
extern void esp_eth_setup(esp_eth_netdev_t* dev);

static int esp32_eth_init(void) {
    esp_eth_setup(&_esp_eth_dev);
    return MINERVA_NETDEV_ETH_INIT(&_esp_eth_dev.netdev);
}
#endif

//--------@@
static void print_ifce(gnrc_netif_t *ifce) {
    printf("print_ifce(): ifce: %p\n", (void *)ifce);
    if (!ifce) return;

    ipv6_addr_t addrs[GNRC_NETIF_IPV6_ADDRS_NUMOF];
    printf("  GNRC_NETIF_IPV6_ADDRS_NUMOF: %d\n", GNRC_NETIF_IPV6_ADDRS_NUMOF); // @@ via Makefile
    gnrc_netapi_get(ifce->pid, NETOPT_IPV6_ADDR, 0, &addrs, sizeof(addrs));

    char addrstr[IPV6_ADDR_MAX_STR_LEN];
    printf("  addrs[0]: %s\n", ipv6_addr_to_str(addrstr, &addrs[0], sizeof(addrstr)));
    //printf("  hint - for `native` board, try `ping6 %s%%tap1` in a new shell\n", addrstr);
    //printf("  hint - for `esp32` board, try `ping6 %s%%br0` in a new shell\n", addrstr);
}
static void find_ifces(gnrc_netif_t **outer, gnrc_netif_t **inner) {
    uint16_t mtu;
    gnrc_netif_t *netif = NULL;

    *outer = *inner = NULL;
    while ((netif = gnrc_netif_iter(netif))) {
        printf("@@ (found) netif: %p\n", (void *)netif);
        gnrc_netapi_get(netif->pid, NETOPT_MAX_PDU_SIZE, 0, &mtu, sizeof(mtu));
        printf("@@ mtu: %d (ETHERNET_DATA_LEN=%d)\n", mtu, ETHERNET_DATA_LEN);

        if (!*outer && (mtu == ETHERNET_DATA_LEN)) {
            *outer = netif;
        } else if (!*inner && (mtu != ETHERNET_DATA_LEN)) {
            *inner = netif;
        }

        if (*outer && *inner)
            break;
    }

    printf("@@ (native|esp-eth|esp-wifi) outer: %p\n", (void *)*outer);
    print_ifce(*outer);

    printf("@@ (esp-now) inner: %p\n", (void *)*inner);
    print_ifce(*inner);
}
//--------@@

/* @@
 * todo - more stuff........
 *      - RIOT/tests/nanocoap_cli
 *      - RIOT/examples/rust-gcoap
 */

//

//extern int foo_cmd(int argc, char **argv);
static int foo_cmd(int argc, char **argv) {
    (void)argc; (void)argv; puts("foo"); return 0;
}

static const shell_command_t shell_commands_minerva[] = {
    { "foo", "Prints foo once", foo_cmd },
    { NULL, NULL, NULL }
};

void start_shell(const shell_command_t *shell_commands) {
    char line_buf[SHELL_DEFAULT_BUFSIZE];
    shell_run(shell_commands, line_buf, SHELL_DEFAULT_BUFSIZE);
}

//

static msg_t main_msg_queue[16];
static gnrc_netif_t *outer_interface = NULL;
static gnrc_netif_t *inner_interface = NULL;

int main(void) {
    /* we need a message queue for the thread running the shell in order to
     * receive potentially fast incoming networking packets */
    msg_init_queue(main_msg_queue, sizeof(main_msg_queue) / sizeof(main_msg_queue[0]));
    puts("@@ [xbd-net] main(): ^^");

#ifdef MINERVA_BOARD_ESP32_ETH
    if (esp32_eth_init()) {
        puts("Error initializing eth devices");
        return 1;
    }

#ifdef MINERVA_DEBUG_ETH_MINIMAL
    start_shell(NULL);
    return 0;
#endif
#endif

    find_ifces(&outer_interface, &inner_interface);
#if defined(MINERVA_BOARD_ESP32_ETH) || defined(MINERVA_BOARD_ESP32_WROOM32)
    set_ips(outer_interface, inner_interface);
#endif

    start_shell(shell_commands_minerva);
    return 0;
}
