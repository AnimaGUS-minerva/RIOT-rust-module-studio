#include <stdio.h>
#include <shell.h>
#include <msg.h>

#include "minerva_border_router.h"
#include "minerva_gcoap.h"
#include "minerva_xbd.h"
#include "rustmod.h"

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

    if (0) {
        if (outer_interface) {
            puts("@@ main(): initializing CoAP server (hint: check with `> coap info`)");
            server_init();

            // hit the internal server
            test_gcoap_req("get", "[::1]:5683", "/.well-known/core");
        }

        // hit the external 'libcoap-minimal/server'
        test_gcoap_req("get", "[" IP6_FIXTURE_SERVER "]:5683", "/hello");

        return 0;
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