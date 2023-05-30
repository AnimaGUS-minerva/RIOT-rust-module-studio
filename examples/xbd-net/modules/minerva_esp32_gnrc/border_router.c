#include <stdio.h>
#include <net/gnrc/ipv6/nib.h>
#include <net/gnrc/ipv6.h>
#include <net/gnrc/netapi.h>
#include <net/gnrc/netif.h>
#ifdef MODULE_GNRC_RPL
  #include <net/gnrc/rpl.h>
#endif

int set_ips(gnrc_netif_t *outer, gnrc_netif_t *inner) {

#if defined(BR_IPV6_ADDR) && defined(BR_IPV6_ADDR_LEN)
    /* Add configured outer address */
    ipv6_addr_t addr;
    ipv6_addr_from_str(&addr, BR_IPV6_ADDR);
    if (gnrc_netif_ipv6_addr_add(outer, &addr, BR_IPV6_ADDR_LEN, 0) < 0) {
        printf("Failed setting outer address.\n");
        return -1;
    }
#endif

#ifdef BR_IPV6_DEF_RT
    /* Add default route */
    ipv6_addr_t defroute = IPV6_ADDR_UNSPECIFIED;
    ipv6_addr_from_str(&addr, BR_IPV6_DEF_RT);
    if (gnrc_ipv6_nib_ft_add(&defroute, 0, &addr, outer->pid, 0) < 0) {
        printf("Failed setting default route.\n");
        return -1;
    }
#endif

    /* Turn off router advert on outer interface, it's really not our job. */
    gnrc_ipv6_nib_change_rtr_adv_iface(outer, false);

    /* Add inner address based on prefix and interface iid */
    if (inner) {
        eui64_t iid;
        ipv6_addr_t prefix;
        ipv6_addr_from_str(&prefix, BR_IPV6_PREFIX);
        if (gnrc_netapi_get(inner->pid, NETOPT_IPV6_IID, 0, &iid, sizeof(iid)) < 0) {
            printf("Failed getting wireless interface iid.\n");
            return -1;
        }
        ipv6_addr_set_aiid(&prefix, iid.uint8);
        if (gnrc_netif_ipv6_addr_add(inner, &prefix, 64, 0) < 0) {
            printf("Failed setting outer address.\n");
            return -1;
        }
    }

#if GNRC_IPV6_NIB_CONF_MULTIHOP_P6C
    /* Add as authoritative border router */
    if (gnrc_ipv6_nib_abr_add(&prefix) < 0) {
        printf("Failed adding prefix as authoritative border router.\n");
        return -1;
    }
#endif

#ifdef MODULE_GNRC_RPL
    /* Configure rpl */
    if (!inner || gnrc_rpl_init(inner->pid) < 0) {
        printf("Failed initializing RPL on inner wireless interface.\n");
        return -1;
    }
    gnrc_rpl_instance_t *inst = gnrc_rpl_instance_get(GNRC_RPL_DEFAULT_INSTANCE);
    if (inst) {
        gnrc_rpl_instance_remove(inst);
    }
    if (!gnrc_rpl_root_init(GNRC_RPL_DEFAULT_INSTANCE, &prefix, false, false)) {
        printf("Failed initializing RPL root.\n");
        return -1;
    }
#endif

    return 0;
}
