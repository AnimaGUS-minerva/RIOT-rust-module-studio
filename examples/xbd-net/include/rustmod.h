typedef void (*xbd_fn_ptr_t)(void);
typedef struct xbd_fn_t {
    const char *name;
    xbd_fn_ptr_t ptr;
} xbd_fn_t;

void rustmod_start(const xbd_fn_t *, size_t);
bool get_kludge_force_no_async(void); // !!

ssize_t xbd_riot_board_handler(coap_pkt_t *pdu, uint8_t *buf, size_t len, coap_request_ctx_t *ctx);

void xbd_on_sock_udp_evt(sock_udp_t *sock, sock_async_flags_t type, void *arg);
