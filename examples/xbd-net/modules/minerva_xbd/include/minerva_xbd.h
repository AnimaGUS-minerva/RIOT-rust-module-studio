/*
 * Copyright (C) 2023 ANIMA Minerva toolkit
 */

#ifndef MINERVA_XBD_H
#define MINERVA_XBD_H



#ifdef __cplusplus
extern "C" {
#endif

void xbd_usleep(uint32_t delay);
void xbd_ztimer_msleep(uint32_t delay, bool debug);
void xbd_ztimer_set(uint32_t delay, void (*cb_handler)(void *), void *arg_ptr, void **timeout_pp);

#ifdef __cplusplus
}
#endif

#endif /* MINERVA_XBD_H */
/** @} */
