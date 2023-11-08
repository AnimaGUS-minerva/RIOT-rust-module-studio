typedef void (*xbd_fn_ptr_t)(void);
typedef struct xbd_fn_t {
    const char *name;
    xbd_fn_ptr_t ptr;
} xbd_fn_t;

void rustmod_start(const xbd_fn_t *, size_t);

ssize_t xbd_riot_board_handler(coap_pkt_t *pdu, uint8_t *buf, size_t len, coap_request_ctx_t *ctx);