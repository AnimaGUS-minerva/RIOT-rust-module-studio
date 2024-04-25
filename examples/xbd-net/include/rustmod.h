typedef void (*xbd_fn_ptr_t)(void);
typedef struct xbd_fn_t {
    const char *name;
    xbd_fn_ptr_t ptr;
} xbd_fn_t;

void rustmod_start(const xbd_fn_t *, size_t);
