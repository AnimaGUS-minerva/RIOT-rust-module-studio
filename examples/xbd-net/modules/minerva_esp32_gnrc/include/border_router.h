#ifndef MINERVA_BORDER_ROUTER_H
#define MINERVA_BORDER_ROUTER_H

#include <net/gnrc/netif.h>

#ifdef __cplusplus
extern "C" {
#endif

int set_ips(gnrc_netif_t *outer, gnrc_netif_t *inner);

#ifdef __cplusplus
}
#endif

#endif /* MINERVA_BORDER_ROUTER_H */