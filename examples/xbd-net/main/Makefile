APPLICATION = main
BOARD ?= esp32-ethernet-kit-v1_0
RIOTBASE ?= $(CURDIR)/../../../RIOT

INCLUDES += -I$(APPDIR)

INCLUDES += -I$(CURDIR)/../include# @@ rustmod
ifeq ($(BOARD),native)
ARCHIVES += $(CURDIR)/../target/i686-unknown-linux-gnu/release/librustmod.a
else
ARCHIVES += $(CURDIR)/../target/xtensa-esp32-none-elf/release/librustmod.a
endif

MINERVA_NET_MODULE_DIR := $(CURDIR)/../modules
EXTERNAL_MODULE_DIRS += $(MINERVA_NET_MODULE_DIR)

##

USEMODULE += minerva_gnrc
INCLUDES += -I$(MINERVA_NET_MODULE_DIR)/minerva_gnrc/include

USEMODULE += minerva_gcoap
INCLUDES += -I$(MINERVA_NET_MODULE_DIR)/minerva_gcoap/include

USEMODULE += minerva_xbd
INCLUDES += -I$(MINERVA_NET_MODULE_DIR)/minerva_xbd/include

DEBUG_ETH_MINIMAL ?= 0
ifeq ($(DEBUG_ETH_MINIMAL),0)
  USEMODULE += shell
  USEMODULE += shell_commands
  USEMODULE += ps
endif

ifeq ($(BOARD),native)
  CFLAGS += -DMINERVA_BOARD_NATIVE
else ifeq ($(BOARD),esp32-wroom-32)
  CFLAGS += -DMINERVA_BOARD_ESP32_WROOM32

  # cf. https://github.com/gschorcht/RIOT_ESP_NOW_WiFi_Border_Router
  CFLAGS += -DESP_WIFI_SSID=\"xxxx\"
  CFLAGS += -DESP_WIFI_PASS=\"yyyy\"

  # "use the same channel as it is used by the AP of the infrastructure-mode WiFi network"
  #CFLAGS += -DESP_NOW_CHANNEL=\"9\"
else ifeq ($(BOARD),esp32-ethernet-kit-v1_0)
  ifeq ($(DEBUG_ETH_MINIMAL),1)
    CFLAGS += -DMINERVA_DEBUG_ETH_MINIMAL

    USEMODULE += minerva_esp32_minimal
    INCLUDES += -I$(MINERVA_NET_MODULE_DIR)/minerva_esp32_minimal/include
  endif
else
  $(error BOARD set is not supported)
endif

##

USEMODULE += gnrc_icmpv6_echo
#==== native (using tap1), ok
# > ping fe80::2cca:caff:fe9f:13c5  # ping to linux tap1
#==== esp32 (using tap0/br0), ok
# > ping fe80::a00:27ff:fefd:b6f8  # ping to linux br0

USEMODULE += shell_cmd_gnrc_udp
#==== native (using tap1)
#---- linux-to-riot, ok
# > udp server start 8899
# $ nc -6uv fe80::2cca:caff:fe9f:13c6%tap1 8899  # send to riot udp server
#---- riot-to-linux, ok
# $ nc -6ul 8877  # linux udp server
# > udp send fe80::2cca:caff:fe9f:13c5 8877 hello  # send to linux tap1
#==== esp32 (using tap0/br0)
#---- linux-to-riot, ok
# > udp server start 8899
# $ nc -6uv fe80::200:ff:fe00:3%br0 8899  # send to riot udp server
#---- riot-to-linux, ok
# $ nc -6ul 8877  # linux udp server
# > udp send fe80::a00:27ff:fefd:b6f8 8877 hello  # send to linux br0

## minerva_gcoap (`coap-{client,server}` from libcoap2-bin)
#==== native (using tap1)
#---- linux-to-riot, ok
# > coap server  # init riot gcoap server
# $ coap-client -N -m get -p 5683 coap://[fe80::78ec:5fff:febd:add9%tap1]/.well-known/core  # send to riot gcoap server
# </cli/stats>;ct=0;rt="count";obs,</riot/board>
#---- riot-to-linux, ok
# $ coap-server  # linux coap server
# > coap get [fe80::20be:cdff:fe0e:44a1]:5683 /.well-known/core  # send to linux tap1
# gcoap: response Success, code 2.05, 105 bytes
# </>;title="General Info";ct=0,</time>;if="clock";rt="ticks";title="Internal Clock";ct=0;obs,</async>;ct=0
#==== esp32 (using tap0/br0)
#---- linux-to-riot, ok
# > coap server  # init riot gcoap server
# $ coap-client -N -m get -p 5683 coap://[fe80::200:ff:fe00:3%br0]/.well-known/core  # send to riot gcoap server
# </cli/stats>;ct=0;rt="count";obs,</riot/board>
#---- riot-to-linux, FIXME
# $ coap-server  # linux coap server
# > coap get [fe80::a00:27ff:fefd:b6f8]:5683 /.well-known/core  # send to linux br0
#[log:riot] gcoap_cli: sending msg ID 31232, 23 bytes
#[log:riot] gcoap: timeout for msg ID 31232
#[log:linux] CRIT coap_network_send: Network is unreachable
#[log:linux] WARN cannot send response for transaction 31232

##

# Comment this out to disable code in RIOT that does safety checking
# which is not needed in a production environment but helps in the
# development process:
DEVELHELP ?= 1

# Change this to 0 show compiler invocation lines by default:
QUIET ?= 1

include $(RIOTBASE)/Makefile.include
