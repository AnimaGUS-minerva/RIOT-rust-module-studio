all: build-native

include ../native.mk
include ../esp32.mk

#submodules

get-crate-embassy:
	[ -e "crates/embassy/.git" ] || \
		(cd crates && git submodule init embassy && git submodule update)
sync-crate-embassy:
	git submodule update --remote crates/embassy && \
		cd crates/embassy && git checkout minerva && git pull

# native

build-native: get-crate-embassy libcoap-riot-pkg
	make native-build-module
	make native-build-riot

run:
	make native-run-riot

#====!!!!temp
run-native-tap-first: build-native
	sudo ip tuntap add dev tap1 mode tap user $$(whoami)  # (after `modprobe tun` on archlinux)
	EMU_NATIVE_TAP=tap1  make native-run-riot
	sudo ip link del tap1
run-native-tap-second:# in second shell
	sudo ip link set tap1 down  # (in case - Temporary failure in name resolution)"
	sudo ip link set tap1 up  # make sure `ip a` shows "state UP" for tap1
#====!!!!

# esp32

build-esp32: get-crate-embassy libcoap-riot-pkg
	make esp32-build-module
	RIOT_BOARD=esp32-ethernet-kit-v1_0  make esp32-build-riot
build-esp32-minimal:
	make esp32-build-module
	RIOT_BOARD=esp32-ethernet-kit-v1_0 DEBUG_ETH_MINIMAL=1 ETH_MANUAL=1 make esp32-build-riot
build-esp32-wifi:
	make esp32-build-module
	RIOT_BOARD=esp32-wroom-32  make esp32-build-riot

run-esp32: build-esp32
	RIOT_BOARD=esp32-ethernet-kit-v1_0  make esp32-run-riot
run-esp32-minimal: build-esp32-minimal
	RIOT_BOARD=esp32-ethernet-kit-v1_0  make esp32-run-riot
run-esp32-wifi: build-esp32-wifi
	RIOT_BOARD=esp32-wroom-32  make esp32-run-riot

# no tap
test-esp32:
	EMU_TIMEOUT=4000  make run-esp32

# ethernet debug only
test-esp32-minimal:
	EMU_TIMEOUT=4000  make run-esp32-minimal

#--------@@ begin: bridged networking
#ETH_IF := enp0sN
ETH_IF := enp0s8

tap-setup:
	@echo "tap-setup..."
	sudo ip link add br0 type bridge
	sudo ip addr flush dev $(ETH_IF)
	sudo ip link set $(ETH_IF) master br0
	sudo ip tuntap add dev tap0 mode tap user $$(whoami)
	sudo ip link set tap0 master br0
	sudo ip link set dev br0 up
	sudo ip link set dev tap0 up
	ip a && brctl show

tap-cleanup:
	@echo "tap-cleanup..."
	sudo ip link del tap0
	sudo ip link del br0
	##sudo dhclient $(ETH_IF)
	ip a && brctl show
#--------@@ end: bridged networking


test-native-notap: build-native
	EMU_TIMEOUT=4000  make native-run-riot

COAP_CLIENT := LD_LIBRARY_PATH=./libcoap/local/lib libcoap-minimal/client
_COAP_SERVER := libcoap-minimal/server
COAP_SERVER := LD_LIBRARY_PATH=./libcoap/local/lib $(_COAP_SERVER)

# assume tap1 is already set up (via `ci-fixture-net` in 'studio/Makefile')
test-native-tap1:
	killall $(_COAP_SERVER) || true
	$(COAP_SERVER) 5683 fe80::20be:cdff:fe0e:44a1%tap1 &
	##IPV6_AUTO=0 IPV6_ADDR=fe80::78ec:5fff:febd:add9 GCOAP_ENABLE_DTLS=1  make build-native
	IPV6_AUTO=0 IPV6_ADDR=fe80::78ec:5fff:febd:add9  make build-native  # with pkg libcoap
	EMU_TIMEOUT=32000 EMU_NATIVE_TAP=tap1  make native-run-riot &
	sleep 2 && $(COAP_CLIENT) 5683 fe80::78ec:5fff:febd:add9%tap1 .well-known/core &

# assume tap0 is already set up (via `ci-fixture-net` in 'studio/Makefile')
test-esp32-tap0: build-esp32
	killall $(_COAP_SERVER) || true
	$(COAP_SERVER) 5683 fe80::a00:27ff:fefd:b6f8%br0 &
	EMU_TIMEOUT=32000 RIOT_BOARD=esp32-ethernet-kit-v1_0 \
 EMU_ESP32_NIC="tap,model=open_eth,ifname=tap0,script=no,downscript=no" \
 make esp32-run-riot &
	sleep 2 && $(COAP_CLIENT) 5683 fe80::200:ff:fe00:3%br0 .well-known/core &

test: build-libcoap-minimal libcoap-riot-examples
	make test-native-notap
	rm -rf ./main/bin && make test-native-tap1
	make test-esp32-minimal
	make test-esp32-tap0

#

build-libcoap-minimal: get-libcoap
	PKG_CONFIG_PATH=../libcoap/local/lib/pkgconfig  make -C libcoap-minimal

get-libcoap:
	[ -e "libcoap/.git" ] || \
 (git submodule init libcoap && git submodule update)
	[ -e "libcoap/local" ] || \
 (cd libcoap && ./autogen.sh && ./configure --prefix `pwd`/local --disable-doxygen --disable-manpages && make && make install)

sync-libcoap:
	git submodule update --remote libcoap && \
 cd libcoap && git checkout develop-minerva && git pull

libcoap-riot-pkg: get-libcoap
	[ -e "libcoap/examples/riot/RIOT" ] || \
 (cd libcoap/examples/riot && ln -s ../../../../../RIOT .)
	[ -e "../../RIOT/pkg/libcoap" ] || make pkg -C libcoap/examples/riot

libcoap-riot-examples: libcoap-riot-pkg
	make client -C libcoap/examples/riot
	##FIXME##make server -C libcoap/examples/riot

#

clean:
	rm -rf target ./main/bin
