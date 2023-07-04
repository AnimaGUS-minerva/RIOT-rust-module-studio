#include <stdio.h>
#include <xtimer.h>
#include <shell.h>
#include <msg.h>
#include "minerva_border_router.h"
#include "minerva_gcoap.h"

//

#if defined(MINERVA_DEBUG_ETH_MINIMAL)
#include "netdev_eth_minimal.h"
#define MINERVA_NETDEV_ETH_INIT minerva_netdev_eth_minimal_init
#elif defined(MINERVA_DEBUG_ETH_MANUAL)
#include "minerva_esp_eth.h"
#define MINERVA_NETDEV_ETH_INIT minerva_gnrc_esp_eth_init
#endif

#if defined(MINERVA_DEBUG_ETH_MINIMAL) || defined(MINERVA_DEBUG_ETH_MANUAL)
#include "esp_eth_netdev.h"
extern esp_eth_netdev_t _esp_eth_dev;
extern void esp_eth_setup(esp_eth_netdev_t* dev);
static int debug_esp32_eth_init(void) {
    esp_eth_setup(&_esp_eth_dev);
    return MINERVA_NETDEV_ETH_INIT(&_esp_eth_dev.netdev);
}
#endif

//

#ifdef MINERVA_BOARD_NATIVE
#define IP6_FIXTURE_SERVER "fe80::20be:cdff:fe0e:44a1" // IP6_FIXTURE_TAP1
#else
#define IP6_FIXTURE_SERVER "fe80::a00:27ff:fefd:b6f8" // IP6_FIXTURE_BR0
#endif

static const shell_command_t shell_commands_minerva[] = {
    { "coap", "CoAP example", gcoap_cli_cmd },
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

#if defined(MINERVA_DEBUG_ETH_MINIMAL) || defined(MINERVA_DEBUG_ETH_MANUAL)
    if (debug_esp32_eth_init()) {
        puts("Error initializing eth devices");
        return 1;
    }
#endif

    find_ifces(&outer_interface, &inner_interface);
    set_ips(outer_interface, inner_interface);

    if (outer_interface) {
        puts("@@ main(): initializing CoAP server (hint: check with `> coap info`)");
        server_init();
    }

    if (1) {
        char *addr = "[" IP6_FIXTURE_SERVER "]:5683";
        //char *payload = "/.well-known/core";
        char *payload = "/hello"; //@@ for 'libcoap-minimal/server'
        char *argv[] = {"coap", "get", addr, payload};
        int argc = sizeof(argv) / sizeof(argv[0]);
        printf("@@ main(): coap get %s %s\n", addr, payload);
        gcoap_cli_cmd(argc, argv);
    }

    //start_shell(null);
    start_shell(shell_commands_minerva);

    /* should be never reached */
    return 0;
}
