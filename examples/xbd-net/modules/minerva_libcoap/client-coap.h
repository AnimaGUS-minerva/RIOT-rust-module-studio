/*
 * client-coap.h -- RIOT client example
 *
 * Copyright (C) 2023-2024 Jon Shallow <supjps-libcoap@jpshallow.com>
 *
 * SPDX-License-Identifier: BSD-2-Clause
 *
 * This file is part of the CoAP library libcoap. Please see README for terms
 * of use.
 */

#ifndef CLIENT_COAP_H
#define CLIENT_COAP_H

#ifdef __cplusplus
extern "C" {
#endif

#define COAP_CLIENT_URI_DEFAULT "coap://[fe80::405:5aff:fe15:9b7f]/.well-known/core"

int test_libcoap_req(char *req, char *uri);//@@

/* Start up the CoAP Client */
void client_coap_init(int argc, char **argv);

int libcoap_cli_cmd(int argc, char **argv) {//@@
    if (argc < 2) {
        printf("usage: %s <uri> (e.g. %s)\n", argv[0], COAP_CLIENT_URI_DEFAULT);
    } else {
        client_coap_init(argc, argv);
    }

    return 0;
}

#ifdef __cplusplus
}
#endif

#endif /* CLIENT_COAP_H */
