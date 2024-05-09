/*
 * Copyright (C) 2024 ANIMA Minerva toolkit
 */

#include <stdio.h>
#include <shell.h>
#include <errno.h>
#include <ztimer.h>

#define ENABLE_DEBUG 0
#include "debug.h"

#ifdef MINERVA_BOARD_NATIVE

#include "native_internal.h"
#include "async_read.h"

extern void xbd_async_shell_on_char(char ch);

// cf. https://github.com/RIOT-OS/RIOT/blob/master/cpu/native/periph/uart.c
static void io_signal_handler(int fd, void *arg) {
    DEBUG("@@ io_signal_handler(): ^^\n");

    (void) arg;

    while (1) {
        char c;
        int status = real_read(fd, &c, 1); // via 'native_internal.h'

        if (status == 1) {
            xbd_async_shell_on_char(c);
        } else {
            if (status == -1 && errno != EAGAIN) {
                DEBUG("@@ error: cannot read from fd\n");
            }

            xbd_async_shell_on_char('\0');
            break;
        }
    }

    native_async_read_continue(fd);
}

static bool init_async_shell_done = false;

int xbd_async_shell_init(void) {
    DEBUG("@@ xbd_async_shell_init(): ^^\n");

    if (!init_async_shell_done) {
        init_async_shell_done = true;
    } else {
        DEBUG("@@ xbd_async_shell_init(): [error] already initialized\n");
        return 1;
    }

    native_async_read_setup();
    native_async_read_add_handler(0, NULL, io_signal_handler);

#if 0// debug
    while (1) { ztimer_sleep(ZTIMER_MSEC, 500); }
    assert(0); // should be never reached
#endif

    return 0;
}
#else
int xbd_async_shell_init(void) {
    printf("@@ xbd_async_shell_init(): TODO - support non-native board\n");
    return 2;
}
#endif /* MINERVA_BOARD_NATIVE */

size_t xbd_async_shell_bufsize(void) {
    return SHELL_DEFAULT_BUFSIZE;
}

void xbd_async_shell_prompt(void) {
    printf("a> ");
}