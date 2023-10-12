#include <stdio.h>
#include <shell.h>
#include <msg.h>
#include "rustmod.h"
#include "minerva_border_router.h"
#include "minerva_gcoap.h"
#include "minerva_xbd.h"

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
void start_shell(const shell_command_t *shell_commands /* `null`able */) {
    char line_buf[SHELL_DEFAULT_BUFSIZE];
    shell_run(shell_commands, line_buf, SHELL_DEFAULT_BUFSIZE);
}

//

//---------------- TODO into minerva_gcoap API
static void _resp_handler(const gcoap_request_memo_t *memo, coap_pkt_t* pdu,
                          const sock_udp_ep_t *remote)
{
    (void)remote;       /* not interested in the source currently */

    if (memo->state == GCOAP_MEMO_TIMEOUT) {
        printf("gcoap: timeout for msg ID %02u\n", coap_get_id(pdu));
        return;
    }
    else if (memo->state == GCOAP_MEMO_RESP_TRUNC) {
        /* The right thing to do here would be to look into whether at least
         * the options are complete, then to mentally trim the payload to the
         * next block boundary and pretend it was sent as a Block2 of that
         * size. */
        printf("gcoap: warning, incomplete response; continuing with the truncated payload\n");
    }
    else if (memo->state != GCOAP_MEMO_RESP) {
        printf("gcoap: error in response\n");
        return;
    }

    coap_block1_t block;
    if (coap_get_block2(pdu, &block) && block.blknum == 0) {
        puts("--- blockwise start ---");
    }

    char *class_str = (coap_get_code_class(pdu) == COAP_CLASS_SUCCESS)
                            ? "Success" : "Error";
    printf("gcoap: response %s, code %1u.%02u", class_str,
                                                coap_get_code_class(pdu),
                                                coap_get_code_detail(pdu));
    if (pdu->payload_len) {
        unsigned content_type = coap_get_content_type(pdu);
        if (content_type == COAP_FORMAT_TEXT
                || content_type == COAP_FORMAT_LINK
                || coap_get_code_class(pdu) == COAP_CLASS_CLIENT_FAILURE
                || coap_get_code_class(pdu) == COAP_CLASS_SERVER_FAILURE) {
            /* Expecting diagnostic payload in failure cases */
            printf(", %u bytes\n%.*s\n", pdu->payload_len, pdu->payload_len,
                                                          (char *)pdu->payload);
        }
        else {
            printf(", %u bytes\n", pdu->payload_len);
            od_hex_dump(pdu->payload, pdu->payload_len, OD_WIDTH_DEFAULT);
        }
    }
    else {
        printf(", empty payload\n");
    }

#if 0 //@@ not support next block for now
    /* ask for next block if present */
    if (coap_get_block2(pdu, &block)) {
        if (block.more) {
            unsigned msg_type = coap_get_type(pdu);
            if (block.blknum == 0 && !strlen(_last_req_path)) {
                puts("Path too long; can't complete blockwise");
                return;
            }

            if (_proxied) {
                gcoap_req_init(pdu, (uint8_t *)pdu->hdr, CONFIG_GCOAP_PDU_BUF_SIZE,
                               COAP_METHOD_GET, NULL);
            }
            else {
                gcoap_req_init(pdu, (uint8_t *)pdu->hdr, CONFIG_GCOAP_PDU_BUF_SIZE,
                               COAP_METHOD_GET, _last_req_path);
            }

            if (msg_type == COAP_TYPE_ACK) {
                coap_hdr_set_type(pdu->hdr, COAP_TYPE_CON);
            }
            block.blknum++;
            coap_opt_add_block2_control(pdu, &block);

            if (_proxied) {
                coap_opt_add_proxy_uri(pdu, _last_req_path);
            }

            int len = coap_opt_finish(pdu, COAP_OPT_FINISH_NONE);
            gcoap_req_send((uint8_t *)pdu->hdr, len, remote,
                           _resp_handler, memo->context);
        }
        else {
            puts("--- blockwise complete ---");
        }
    }
#endif //@@ not support next block for now
}

#include "net/sock/util.h" //@@ for `sock_udp_name2ep()`
//@@static size_t _send(uint8_t *buf, size_t len, char *addr_str)
static size_t _send(uint8_t *buf, size_t len, char *addr_str, void *context, gcoap_resp_handler_t resp_handler) //@@
{
    size_t bytes_sent;
    sock_udp_ep_t *remote;
    sock_udp_ep_t new_remote;

//    if (_proxied) {
//        remote = &_proxy_remote;
//    }
//    else {
        if (sock_udp_name2ep(&new_remote, addr_str) != 0) {
            return 0;
        }

        if (new_remote.port == 0) {
            if (IS_USED(MODULE_GCOAP_DTLS)) {
                new_remote.port = CONFIG_GCOAPS_PORT;
            }
            else {
                new_remote.port = CONFIG_GCOAP_PORT;
            }
        }

        remote = &new_remote;
//    }

    //@@bytes_sent = gcoap_req_send(buf, len, remote, _resp_handler, NULL);
    bytes_sent = gcoap_req_send(buf, len, remote, resp_handler, context);//@@
    if (bytes_sent > 0) {
        req_count++;
    }
    return bytes_sent;
}
//--------
extern void _xbd_resp_handler(
        const gcoap_request_memo_t *memo, coap_pkt_t* pdu, const sock_udp_ep_t *remote,
        uint8_t **payload, size_t *payload_len, void **context
) {
    _resp_handler(memo, pdu, remote);

    *context = memo->context;

    // @@ TODO return the C error to Rust
    if (memo->state == GCOAP_MEMO_TIMEOUT || memo->state != GCOAP_MEMO_RESP) {
        *payload = NULL;
        *payload_len = 0;
    } else {
        *payload = pdu->payload_len ? pdu->payload : NULL;
        *payload_len = pdu->payload_len;
    }
}
static void xbd_gcoap_req_send(char *addr, char *uri, void *context) {
    uint8_t buf[CONFIG_GCOAP_PDU_BUF_SIZE];
    coap_pkt_t pdu;
    size_t len;

    gcoap_req_init(&pdu, &buf[0], CONFIG_GCOAP_PDU_BUF_SIZE, 1 /* get */, uri);
    unsigned msg_type = COAP_TYPE_NON;
    coap_hdr_set_type(pdu.hdr, msg_type);
    len = coap_opt_finish(&pdu, COAP_OPT_FINISH_NONE);
    printf("@@ xbd_gcoap_req_send(): addr: %s, uri: %s\n", addr, uri);
    printf("    sending msg ID %u, %u bytes\n", coap_get_id(&pdu), (unsigned) len);

    //printf("@@ context: %p\n", context);
    if (!_send(&buf[0], len, addr, context, xbd_resp_handler)) {
        puts("gcoap_cli: msg send failed");
    } else {
        /* send Observe notification for /cli/stats */
        notify_observers();
    }
}

//

static const xbd_fn_t xbd_fns[] = {
    { "xbd_usleep", (xbd_fn_ptr_t)xbd_usleep },
    { "xbd_ztimer_msleep", (xbd_fn_ptr_t)xbd_ztimer_msleep },
    { "xbd_ztimer_set", (xbd_fn_ptr_t)xbd_ztimer_set },
    { "xbd_gcoap_req_send", (xbd_fn_ptr_t)xbd_gcoap_req_send },
};
static const size_t xbd_fns_sz = sizeof(xbd_fns) / sizeof(xbd_fns[0]);

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

    if (0) {
        // hit the internal server
        test_gcoap_req("get", "[::1]:5683", "/.well-known/core");

        // hit the external 'libcoap-minimal/server'
        test_gcoap_req("get", "[" IP6_FIXTURE_SERVER "]:5683", "/hello");
    }

    if (1) {
        rustmod_start(xbd_fns, xbd_fns_sz);

        /* !!!! WIP async shell
         * - https://github.com/rust-lang/futures-rs/blob/master/futures-util/src/io/mod.rs
//! Asynchronous I/O.
//!
//! This module is the asynchronous version of `std::io`. It defines four
//! traits, [`AsyncRead`], [`AsyncWrite`], [`AsyncSeek`], and [`AsyncBufRead`],
//! which mirror the `Read`, `Write`, `Seek`, and `BufRead` traits of the
//! standard library. However, these traits integrate with the asynchronous
//! task system, so that if an I/O object isn't ready for reading (or writing),
//! the thread is not blocked, and instead the current task is queued to be
//! woken when I/O is ready.
//!
//! In addition, the [`AsyncReadExt`], [`AsyncWriteExt`], [`AsyncSeekExt`], and
//! [`AsyncBufReadExt`] extension traits offer a variety of useful combinators
//! for operating with asynchronous I/O objects, including ways to work with
//! them using futures, streams and sinks.
//!
//! This module is only available when the `std` feature of this
//! library is activated, and it is activated by default.
        */
    }

    start_shell(shell_commands_minerva);

    /* should be never reached */
    return 0;
}