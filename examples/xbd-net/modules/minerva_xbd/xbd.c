/*
 * Copyright (C) 2023 ANIMA Minerva toolkit
 */

#include <stdlib.h>
#include <xtimer.h>
#include <ztimer.h>

#include "net/sock/util.h"
#include "minerva_xbd.h"

void xbd_usleep(uint32_t delay) {
    putchar('.');
    xtimer_usleep(delay);
}

static bool blink = false;
void xbd_ztimer_msleep(uint32_t delay, bool debug) {
    if (debug) {
        //putchar('.');
        //====
        putchar((blink = !blink) ? '#' : ' ');
        putchar('\b');
    }

    ztimer_sleep(ZTIMER_MSEC, delay);
}

void xbd_ztimer_set(uint32_t delay, void (*cb_handler)(void *), void *arg_ptr, void **timeout_pp) {
    printf("@@ xbd_ztimer_set(): delay(ms): %d\n", delay);

    ztimer_t *timeout = (ztimer_t *) calloc(sizeof(ztimer_t), 1);
    timeout->callback = cb_handler;
    timeout->arg = arg_ptr;

    *timeout_pp = timeout;
    //printf("@@ xbd_ztimer_set(): *timeout_pp (= timeout_ptr): %p\n", *timeout_pp);

    ztimer_set(ZTIMER_MSEC, timeout, delay);
}

//

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

void xbd_gcoap_req_send(char *addr, char *uri, void *context, gcoap_resp_handler_t resp_handler) {
    uint8_t buf[CONFIG_GCOAP_PDU_BUF_SIZE];
    coap_pkt_t pdu;
    size_t len;

    gcoap_req_init(&pdu, &buf[0], CONFIG_GCOAP_PDU_BUF_SIZE, 1 /* get */, uri);
    unsigned msg_type = COAP_TYPE_NON;
    coap_hdr_set_type(pdu.hdr, msg_type);
    len = coap_opt_finish(&pdu, COAP_OPT_FINISH_NONE);
    printf("@@ xbd_gcoap_req_send(): addr: %s, uri: %s\n", addr, uri);
    printf("    sending msg ID %u, %u bytes\n", coap_get_id(&pdu), (unsigned) len);

    if (!_send(&buf[0], len, addr, context, resp_handler)) {
        puts("gcoap_cli: msg send failed");
    } else {
        /* send Observe notification for /cli/stats */
        notify_observers();
    }
}

//

