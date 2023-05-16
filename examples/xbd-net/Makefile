all: build-native

include ../native.mk
include ../esp32.mk

# native

build-native:
	##make native-build-module
	make native-build-riot

#====!!!!
run-native-first: build-native
	sudo ip tuntap add dev tap1 mode tap user $$(whoami)
	@echo "[hint] > ifconfig"  # e.g. fe80::58d6:7dff:fe0f:6923
	@echo "[hint] > udp server start 1333"  # TODO
	EMU_NATIVE_TAP=tap1  make native-run-riot
	sudo ip link del tap1
run-native-second:# in second shell
	sudo ip link set tap1 down  # (in case - Temporary failure in name resolution)"
	sudo ip link set tap1 up  # make sure `ip a` shows "state UP" for tap1
	@echo [hint] ping6 fe80::58d6:7dff:fe0f:6923%tap1
	@echo [hint] nc -6uv fe80::58d6:7dff:fe0f:6923%tap1 1333  # TODO
#====!!!!

test-native: build-native
	EMU_TIMEOUT=4000  make native-run-riot

# esp32

build-esp32:
	##make esp32-build-module
	RIOT_BOARD=esp32-ethernet-kit-v1_0  make esp32-build-riot

build-esp32-debug:
	##make esp32-build-module
	RIOT_BOARD=esp32-ethernet-kit-v1_0 USE_ADHOC_GNRC=0  make esp32-build-riot

run-esp32: build-esp32
	RIOT_BOARD=esp32-ethernet-kit-v1_0  make esp32-run-riot

run-esp32-debug: build-esp32-debug
	RIOT_BOARD=esp32-ethernet-kit-v1_0  make esp32-run-riot

test-esp32: build-esp32
	EMU_TIMEOUT=4000 RIOT_BOARD=esp32-ethernet-kit-v1_0  make esp32-run-riot

#--------@@ begin: bridged networking
#ETH_IF := enp0sN
ETH_IF := enp0s8

run-esp32-tap: build-esp32
	make tap-setup
	RIOT_BOARD=esp32-ethernet-kit-v1_0 \
 EMU_ESP32_NIC="tap,model=open_eth,ifname=tap0,script=no,downscript=no" \
 make esp32-run-riot
	make tap-cleanup

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

run: run-native

test:
	##make test-native
	make test-esp32

clean:
	rm -rf target ./main/bin
