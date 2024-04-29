/*
 * Copyright (C) 2024 ANIMA Minerva toolkit
 */

#ifndef MINERVA_LIBCOAP_H
#define MINERVA_LIBCOAP_H

#ifdef __cplusplus
extern "C" {
#endif

int libcoap_cli_cmd(int argc, char **argv);
int test_libcoap_req(char *req, char *uri);

#ifdef __cplusplus
}
#endif

#endif /* MINERVA_LIBCOAP_H */