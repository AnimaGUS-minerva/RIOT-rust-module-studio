#include <stdio.h>
#include <xtimer.h>
#include <ztimer.h>
#include <shell.h>
#include <msg.h>
#include "rustmod.h"
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

// ---- "minerva_xbd.h" !!

static void xbd_usleep(uint32_t delay) {
    putchar('.');
    xtimer_usleep(delay);
}

static void xbd_ztimer_msleep(uint32_t delay) {
    putchar('.');
    ztimer_sleep(ZTIMER_MSEC, delay);
}

static void xbd_ztimer_set(uint32_t delay, void (*cb_handler)(void *), void *arg_ptr, void **timeout_pp) {
    printf("@@ xbd_ztimer_set(): delay(ms): %d\n", delay);

    ztimer_t *timeout = (ztimer_t *) calloc(sizeof(ztimer_t), 1);
    timeout->callback = cb_handler;
    timeout->arg = arg_ptr;

    *timeout_pp = timeout;
    printf("@@ xbd_ztimer_set(): *timeout_pp (= timeout_ptr): %p\n", *timeout_pp);

    ztimer_set(ZTIMER_MSEC, timeout, delay);
}

// !!!!
//static void xbd_gcoap_client_send(/* addr,msg */, void (*cb_handler)(void *), void *arg_ptr, void **timeout_pp) {
//    printf("@@ xbd_ztimer_set(): delay(ms): %d\n", delay);
//
////    ztimer_t *timeout = (ztimer_t *) calloc(sizeof(ztimer_t), 1);
////    timeout->callback = cb_handler;
////    timeout->arg = arg_ptr;
////
////    *timeout_pp = timeout;
////
//
//    //ztimer_set(ZTIMER_MSEC, timeout, delay);
//    //==== !!!!
//    //bytes_sent = gcoap_req_send(buf, len, remote, _resp_handler, NULL); // client.c
//    bytes_sent = gcoap_req_send(buf, len, remote, _resp_handler_xx, NULL); // !!!! ??
//}
static void xbd_gcoap_req_send(void/* TODO */) {
    uint8_t buf[CONFIG_GCOAP_PDU_BUF_SIZE];
    coap_pkt_t pdu;
    size_t len;
    int code_pos = 1; // get
    char *uri = "/hello";

    gcoap_req_init(&pdu, &buf[0], CONFIG_GCOAP_PDU_BUF_SIZE, code_pos, uri);
    unsigned msg_type = COAP_TYPE_NON;
    coap_hdr_set_type(pdu.hdr, msg_type);
    len = coap_opt_finish(&pdu, COAP_OPT_FINISH_NONE);
    printf("!!!! gcoap_cli: sending msg ID %u, %u bytes\n", coap_get_id(&pdu), (unsigned) len);

    // !!!! WIP
    //sock_udp_ep_t *remote;
    //size_t bytes_sent = gcoap_req_send(buf, len, remote, _resp_handler_xx, NULL);
    //printf("@@ bytes_sent: %d", bytes_sent);
}
// !!!! WIP
//static void _resp_handler__xx(const gcoap_request_memo_t *memo, coap_pkt_t* pdu,
//                              const sock_udp_ep_t *remote) {
//    yy_data = _resp_handler__mod(memo, pdu, remote);
//    arg = pack(yy_data, tag_gcoap_client)
//    callbacks::add_gcoap_client_callback(arg_ptr); // impl same as add_timeout_callback ??
//}

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

    if (1) { // oneshot CoAP client
        char *addr = "[" IP6_FIXTURE_SERVER "]:5683";
        //char *payload = "/.well-known/core";
        char *payload = "/hello"; //@@ for 'libcoap-minimal/server'
        char *argv[] = {"coap", "get", addr, payload};
        int argc = sizeof(argv) / sizeof(argv[0]);

        printf("@@ main(): coap get %s %s\n", addr, payload);
        gcoap_cli_cmd(argc, argv);
    }

    if (1) {
        printf("@@ main(): before calling rustmod\n");
        rustmod_start(xbd_usleep, xbd_ztimer_msleep, xbd_ztimer_set, xbd_gcoap_req_send);
        printf("@@ main(): after calling rustmod\n");
    }

    //start_shell(null);
    start_shell(shell_commands_minerva);

    /* should be never reached */
    return 0;
}