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

void xbd_resp_handler(
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

//