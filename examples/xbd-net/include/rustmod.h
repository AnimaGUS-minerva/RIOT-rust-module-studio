#include "minerva_gcoap.h"

void rustmod_start(
        void (*)(uint32_t),
        void (*)(uint32_t),
        void (*)(uint32_t, void (*)(void *), void *, void **),
        void (*)(char *, char *, void *));
void xbd_resp_handler(
        const gcoap_request_memo_t *memo, coap_pkt_t* pdu, const sock_udp_ep_t *remote);