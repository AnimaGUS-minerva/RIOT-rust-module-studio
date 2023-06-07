#include <stdio.h>
#include <xtimer.h>
#include <shell.h>
#include <msg.h>

#include "border_router.h"

#if defined(MINERVA_DEBUG_ETH_MINIMAL)
#include "netdev_eth_minimal.h"
#define MINERVA_NETDEV_ETH_INIT minerva_netdev_eth_minimal_init
#elif defined(MINERVA_BOARD_ESP32_ETH)
#include "minerva_esp_eth.h"
#define MINERVA_NETDEV_ETH_INIT minerva_gnrc_esp_eth_init
#endif

#ifdef MINERVA_BOARD_ESP32_ETH
#include "esp_eth_netdev.h"
extern esp_eth_netdev_t _esp_eth_dev;
extern void esp_eth_setup(esp_eth_netdev_t* dev);
static int esp32_eth_init(void) {
    esp_eth_setup(&_esp_eth_dev);
    return MINERVA_NETDEV_ETH_INIT(&_esp_eth_dev.netdev);
}
#endif

/* @@
 * todo - more stuff........
 *      - RIOT/tests/nanocoap_cli
 *      - RIOT/examples/rust-gcoap
 */

//

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
#endif

    find_ifces(&outer_interface, &inner_interface);
    set_ips(outer_interface, inner_interface);

    //start_shell(null);
    start_shell(shell_commands_minerva);

    /* should be never reached */
    return 0;
}
