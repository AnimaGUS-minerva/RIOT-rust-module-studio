/*
 * Copyright (C) 2023 ANIMA Minerva toolkit
 */

#include <stdlib.h>
#include <xtimer.h>
#include <ztimer.h>

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