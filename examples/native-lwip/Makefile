all: build

RIOT_TAG := 2021.10
RIOT_ELF := ./main/bin/native/main.elf
build:
	@if [ ! -e RIOT ]; then \
		git clone https://github.com/RIOT-OS/RIOT RIOT; \
		cd RIOT && git checkout -q $(RIOT_TAG); \
	fi
	cd ./main && BOARD=native RIOTBASE=$(CURDIR)/RIOT make
	ldd $(RIOT_ELF)
	file $(RIOT_ELF)

run: build
	$(RIOT_ELF)

test:
	make build  # test `build` only

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
	@echo "\$$ sudo ip link set tap0 up"
	@echo "\$$ ping6 fe80:0:0:0:4012:67ff:fefe:4130%tap0  # ok"
	@echo "\$$ nc -6uv fe80:0:0:0:4012:67ff:fefe:4130%tap0 1333  # ok"
	@echo "==== end new shell"
	sudo $(RIOT_ELF) tap0

clean:
	rm -rf ./main/bin
