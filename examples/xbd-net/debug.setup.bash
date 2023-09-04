alias ee-minimal='make run-esp32-minimal'
alias nn-notap='make run-native-notap'
alias nnn='nn-notap'
alias ee-wifi='make run-esp32-wroom32'


alias ee='echo "assuming tap0/br0 is already set up" && make build-esp32 && RIOT_BOARD=esp32-ethernet-kit-v1_0  EMU_ESP32_NIC="tap,model=open_eth,ifname=tap0,script=no,downscript=no"  make esp32-run-riot'
alias nn='echo "assuming tap1 is already set up" && IPV6_AUTO=0 IPV6_ADDR=fe80::78ec:5fff:febd:add9  make build-native && EMU_NATIVE_TAP=tap1  make native-run-riot'
#$ ip a
#...
#3: enp0s8: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc fq_codel master br0 state UP group default qlen 1000
#    link/ether 08:00:27:fd:b6:f8 brd ff:ff:ff:ff:ff:ff
#22: tap1: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc fq_codel state DOWN group default qlen 1000
#    link/ether 0e:b7:bd:21:79:fc brd ff:ff:ff:ff:ff:ff
#    inet6 fe80::cb7:bdff:fe21:79fc/64 scope link
#       valid_lft forever preferred_lft forever
#23: br0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP group default qlen 1000
#    link/ether 08:00:27:fd:b6:f8 brd ff:ff:ff:ff:ff:ff
#    inet6 fe80::a00:27ff:fefd:b6f8/64 scope link
#       valid_lft forever preferred_lft forever
#24: tap0: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc fq_codel master br0 state DOWN group default qlen 1000
#    link/ether 52:32:c1:53:ab:4c brd ff:ff:ff:ff:ff:ff
#    inet6 fe80::5032:c1ff:fe53:ab4c/64 scope link
#       valid_lft forever preferred_lft forever
