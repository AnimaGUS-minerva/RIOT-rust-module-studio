all: build

RIOT_ELF := ./main/bin/native/main.elf
build:
	cd ./main && BOARD=native RIOTBASE=$(CURDIR)/../../RIOT make
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
	@echo "> udp server start 1337"
	@echo "> udp send fe80::2842:7fff:fe79:d3e5 1337 hello"
	@echo "==== end example"
	sudo $(RIOT_ELF) tap0

clean:
	rm -rf ./main/bin
