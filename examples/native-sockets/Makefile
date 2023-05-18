all: build

MAIN_ELF := ./main/bin/native/main.elf
build:
	cd ./main && BOARD=native RIOTBASE=$(CURDIR)/../../RIOT make
	ldd $(MAIN_ELF)
	file $(MAIN_ELF)

run: build
	$(MAIN_ELF)

test:
	make build  # test `build` only

# OBSOLETE
# linux-to-riot udp send [1] works, but riot-to-linux direction [2] does NOT work.
# Use the `shell_cmd_gnrc_udp` module [3] instread.
#
# [1] https://github.com/AnimaGUS-minerva/RIOT/blob/fc7de5ceca954d9b8ea8a2af0b8b23c41c3b1565/examples/gnrc_networking/README.md?plain=1#L61-L67
# [2] https://github.com/AnimaGUS-minerva/RIOT/blob/fc7de5ceca954d9b8ea8a2af0b8b23c41c3b1565/examples/gnrc_networking/README.md?plain=1#L89-L93
# [3] https://github.com/AnimaGUS-minerva/studio/blob/880fe4326fc146727d16135997a62a68e6570fe5/examples/xbd-net/main/Makefile#L29-L43
demo: build
	@echo "==== begin example"
	@echo "> help"
	@echo "> ifconfig"
	@echo "> udp server start 1337"
	@echo "> udp send fe80::2842:7fff:fe79:d3e5 1337 hello"
	@echo "==== end example"
	sudo $(MAIN_ELF) tap0

clean:
	rm -rf ./main/bin
