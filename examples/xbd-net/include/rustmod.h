#include "minerva_gcoap.h"

typedef void (*xbd_fn_ptr_t)(void);
typedef struct xbd_fn_t {
    const char *name;
    xbd_fn_ptr_t ptr;
} xbd_fn_t;

void rustmod_start(const xbd_fn_t *, size_t);
void xbd_resp_handler(
        const gcoap_request_memo_t *memo, coap_pkt_t* pdu, const sock_udp_ep_t *remote);