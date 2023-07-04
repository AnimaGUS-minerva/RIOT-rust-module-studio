/* minimal CoAP server
 *
 * Copyright (C) 2018-2023 Olaf Bergmann <bergmann@tzi.org>
 * Copyright (C) 2023 ANIMA Minerva toolkit
 */

#include <cstring>
#include <cstdlib>
#include <cstdio>

#include "common.hh"

int
main(int argc, char* argv[]) {//@@
  coap_context_t  *ctx = nullptr;
  coap_address_t dst;
  coap_resource_t *resource = nullptr;
  coap_endpoint_t *endpoint = nullptr;
  int result = EXIT_FAILURE;;
  coap_str_const_t *ruri = coap_make_str_const("hello");
  coap_startup();

  /* resolve destination address where server should be sent */
  //if (resolve_address("localhost", "5683", &dst) < 0) {
  //==== @@
  if (argc < 3) {
    printf("@@ argc: %d\n", argc);
    printf("@@ usage: ./server port ep_addr (e.g. 5683 fe80::20be:cdff:fe0e:44a1%%tap1)\n");
    goto finish;
  }
  if (resolve_address(argv[2], argv[1], &dst) < 0) {//@@
  //if (resolve_address("fe80::20be:cdff:fe0e:44a1%tap1", "5683", &dst) < 0) {//@@ `nn` ok
  //if (resolve_address("fe80::a00:27ff:fefd:b6f8%br0", "5683", &dst) < 0) {//@@ `ee` FIXME
  /* @@ nn
v:1 t:NON c:GET i:7356 {a4e6} [ Uri-Path:hello ]
v:1 t:NON c:2.05 i:7356 {a4e6} [ ] :: 'world'
   */
  /* @@ ee
v:1 t:NON c:GET i:1a90 {0a44} [ Uri-Path:hello ]
v:1 t:NON c:2.05 i:1a90 {0a44} [ ] :: 'world'
Jun 27 11:11:22.346 CRIT coap_socket_send: Network is unreachable
   */
  //====
    coap_log_crit("failed to resolve address\n");
    goto finish;
  }

  /* create CoAP context and a client session */
  ctx = coap_new_context(nullptr);

  if (!ctx || !(endpoint = coap_new_endpoint(ctx, &dst, COAP_PROTO_UDP))) {
    coap_log_emerg("cannot initialize context\n");
    goto finish;
  }

  resource = coap_resource_init(ruri, 0);
  coap_register_handler(resource, COAP_REQUEST_GET,
                        [](auto, auto,
                           const coap_pdu_t *request,
                           auto,
                           coap_pdu_t *response) {
                          coap_show_pdu(COAP_LOG_WARN, request);
                          coap_pdu_set_code(response, COAP_RESPONSE_CODE_CONTENT);
                          coap_add_data(response, 5,
                                        (const uint8_t *)"world");
                          coap_show_pdu(COAP_LOG_WARN, response);
                        });
  coap_add_resource(ctx, resource);

  while (true) { coap_io_process(ctx, COAP_IO_WAIT); }

  result = EXIT_SUCCESS;
 finish:

  coap_free_context(ctx);
  coap_cleanup();

  return result;
}
