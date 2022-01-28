all: build

RIOT_TAG := 2021.10
clone-RIOT:
	@if [ ! -e RIOT ]; then \
		git clone https://github.com/RIOT-OS/RIOT RIOT; \
		cd RIOT && git checkout -q $(RIOT_TAG); \
	fi

#

MP_REPO := https://github.com/AnimaGUS-minerva/micropython
MP_REV := 4bf34ac9b  # v1.17-riot-lwip
MP_ELF := ./micropython/ports/riot/main/bin/native/main.elf
clone-mp: clone-RIOT
	@if [ ! -e micropython ]; then \
		git clone $(MP_REPO) micropython; \
		(cd micropython; git checkout -q $(MP_REV); ln -s ../../../RIOT ports/riot/); \
	fi

build-mp: clone-mp
	make -C micropython/ports/riot
	ldd $(MAIN_ELF) && file $(MAIN_ELF)
run-mp: build-mp
	$(MP_ELF)

#

MAIN_ELF := ./main/bin/native/main.elf
build: clone-RIOT
	cd ./main && BOARD=native RIOTBASE=$(CURDIR)/RIOT make
	ldd $(MAIN_ELF) && file $(MAIN_ELF)

run: build
	$(MAIN_ELF)

demo: build
	@echo "==== begin example"
	@echo "> help"
	@echo "> ifconfig"
	@echo "> udp server start 1333"
	@echo "> udp send fe80:0:0:0:4012:67ff:fefe:4130:1333 00  # FIXME: Error: unable to parse destination address"
	@echo "==== end example"
	@echo "==== begin new shell"
	@echo "\$$ # https://github.com/RIOT-OS/RIOT/blob/2021.10/examples/gnrc_networking/README.md"
	@echo "\$$ sudo ip tuntap add tap0 mode tap user \$${USER}"
	@echo "\$$ sudo ip link set tap0 down  # (in case - Temporary failure in name resolution)"
	@echo "\$$ sudo ip link set tap0 up"
	@echo "\$$ ping6 fe80:0:0:0:4012:67ff:fefe:4130%tap0  # ok"
	@echo "\$$ nc -6uv fe80:0:0:0:4012:67ff:fefe:4130%tap0 1333  # ok"
	@echo "==== end new shell"
	sudo $(MAIN_ELF) tap0

clean:
	rm -rf ./main/bin

test: clone-RIOT
	cd ./RIOT/tests/lwip_sock_ip && make && make test
	cd ./RIOT/tests/lwip_sock_udp && make && make test
	cd ./RIOT/tests/lwip_sock_tcp && make && make test
	make build     # test build only
	make build-mp  # test build only
