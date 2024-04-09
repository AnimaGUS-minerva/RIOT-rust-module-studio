/*
 * Copyright (c) 2015-2017 Ken Bannister. All rights reserved.
 *
 * This file is subject to the terms and conditions of the GNU Lesser
 * General Public License v2.1. See the file LICENSE in the top level
 * directory for more details.
 */

/**
 * @ingroup     examples
 * @{
 *
 * @file
 * @brief       gcoap CLI support
 *
 * @author      Ken Bannister <kb2ma@runbox.com>
 * @author      Hauke Petersen <hauke.petersen@fu-berlin.de>
 * @author      Hendrik van Essen <hendrik.ve@fu-berlin.de>
 *
 * @}
 */

/*
 * Copyright (C) 2023 ANIMA Minerva toolkit
 */

#include "net/sock/util.h"
#include "minerva_xbd.h"

extern size_t xbd_blockwise_state_index(void);
extern char * xbd_blockwise_addr_ptr(size_t idx);
extern char * xbd_blockwise_uri_ptr(size_t idx);
extern size_t xbd_blockwise_hdr_copy(const uint8_t *buf, size_t buf_sz, size_t idx);

extern void xbd_blockwise_async_gcoap_next(
        size_t idx,
        const char *addr, size_t addr_len,
        const char *uri, size_t uri_len,
        const char *hdr, size_t hdr_len);
extern void xbd_blockwise_async_gcoap_complete(size_t idx);

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

void xbd_gcoap_req_send(
        char *addr, char *uri,
        uint8_t method, uint8_t *payload, size_t payload_len,
        bool blockwise, size_t idx,
        void *context, gcoap_resp_handler_t resp_handler) {
    uint8_t buf[CONFIG_GCOAP_PDU_BUF_SIZE];
    size_t hdr_len;

    if (blockwise && (hdr_len = xbd_blockwise_hdr_copy(&buf[0], CONFIG_GCOAP_PDU_BUF_SIZE, idx))) {
        printf("@@ sending non-first msg (idx=%u)\n", idx);
    } else {
        coap_pkt_t pdu;
        gcoap_req_init(&pdu, &buf[0], CONFIG_GCOAP_PDU_BUF_SIZE, method, uri);

        unsigned msg_type = COAP_TYPE_NON;
        coap_hdr_set_type(pdu.hdr, msg_type);
        hdr_len = coap_opt_finish(&pdu, payload_len ? COAP_OPT_FINISH_PAYLOAD : COAP_OPT_FINISH_NONE);

        printf("@@ sending msg (ID=%u)\n", coap_get_id(&pdu));
    }
    printf("@@ xbd_gcoap_req_send(): addr: %s, uri: %s hdr_len: %u\n", addr, uri, hdr_len);

    if (blockwise) {
        printf("@@ debug - blockwise_state_index: %u\n", idx);
    }

    printf("@@ payload: %p payload_len: %d\n", payload, payload_len);
    if (payload_len) {
        memcpy(buf + hdr_len /* (== `pdu.payload`) */, payload, payload_len);
    }

    if (!_send(&buf[0], hdr_len + payload_len, addr, context, resp_handler)) {
        puts("gcoap_cli: msg send failed");
    } else {
        /* send Observe notification for /cli/stats */
        notify_observers();
    }
}

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
}

static void _resp_handler_blockwise_async(const gcoap_request_memo_t *memo, coap_pkt_t* pdu,
                                          const sock_udp_ep_t *remote, coap_block1_t *block) {
    size_t idx = xbd_blockwise_state_index();

    if (block->more) {
        char *last_addr = xbd_blockwise_addr_ptr(idx);
        char *last_uri = xbd_blockwise_uri_ptr(idx);

        size_t last_uri_len = strlen(last_uri);

        unsigned msg_type = coap_get_type(pdu);

        if (block->blknum == 0 && !last_uri_len) {
            puts("Path too long; can't complete blockwise");
            return;
        }

//            if (_proxied) {
//                gcoap_req_init(pdu, (uint8_t *)pdu->hdr, CONFIG_GCOAP_PDU_BUF_SIZE,
//                               COAP_METHOD_GET, NULL);
//            }
//            else {
            gcoap_req_init(pdu, (uint8_t *)pdu->hdr, CONFIG_GCOAP_PDU_BUF_SIZE,
                           COAP_METHOD_GET, last_uri);
//            }

        if (msg_type == COAP_TYPE_ACK) {
            coap_hdr_set_type(pdu->hdr, COAP_TYPE_CON);
        }
        block->blknum++;
        coap_opt_add_block2_control(pdu, block);

//            if (_proxied) {
//                coap_opt_add_proxy_uri(pdu, last_uri);
//            }

        (void)memo;
        (void)remote;
        size_t len = coap_opt_finish(pdu, COAP_OPT_FINISH_NONE);
        xbd_blockwise_async_gcoap_next(idx,
                last_addr, strlen(last_addr),
                last_uri, last_uri_len,
                (char *)pdu->hdr, len);
    } else {
        puts("--- blockwise complete ---");
        xbd_blockwise_async_gcoap_complete(idx);
    }
}

uint8_t xbd_resp_handler(
        const gcoap_request_memo_t *memo, coap_pkt_t* pdu, const sock_udp_ep_t *remote,
        uint8_t **payload, size_t *payload_len, void **context
) {
    _resp_handler(memo, pdu, remote);

    *context = memo->context;

    printf("@@ memo->state: %d\n", memo->state);
    if (memo->state == GCOAP_MEMO_TIMEOUT ||
        (memo->state != GCOAP_MEMO_RESP_TRUNC && memo->state != GCOAP_MEMO_RESP)) {
        *payload = NULL;
        *payload_len = 0;
    } else {
        *payload = pdu->payload_len ? pdu->payload : NULL;
        *payload_len = pdu->payload_len;
    }

    coap_block1_t block;
    if (coap_get_block2(pdu, &block)) { // ask for next block if present
        _resp_handler_blockwise_async(memo, pdu, remote, &block);
    }

    return memo->state;
}