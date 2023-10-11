/*
 * Copyright (C) 2023 ANIMA Minerva toolkit
 */

#include "minerva_xbd.h"

void xbd_usleep(uint32_t delay) {
    putchar('.');
    xtimer_usleep(delay);
}