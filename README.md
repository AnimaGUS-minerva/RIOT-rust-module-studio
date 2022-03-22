# iot-rust-module-studio

[![MIT licensed][mit-badge]][mit-url]
[![CI][actions-badge]][actions-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/AnimaGUS-minerva/RIOT-rust-module-studio/blob/main/LICENSE
[actions-badge]: https://github.com/AnimaGUS-minerva/RIOT-rust-module-studio/workflows/CI/badge.svg
[actions-url]: https://github.com/AnimaGUS-minerva/RIOT-rust-module-studio/actions

Robust IoT development with Rust and RIOT-OS.

### Repository map

```
/
  Makefile
  crates/              .... 💡 currently supported MCU arches: ESP32 `xtensa` and Linux native `x86`
    mcu-emu            .... emulator runner (`qemu-system-xtensa` or RIOT native board binary)
  examples/
    esp32-no_std       .... bare `no_std` firmware with a minimal Rust module 'librustmod.a'
    xbd-base           .... cross-`BOARD` (esp32/native) firmware with a demo module 'librustmod.a'
    xbd-micropython    .... cross-`BOARD` firmware featuring MicroPython with a RFC8995 module 'libvoucher.a'
    ...
```

### Environments

Ubuntu 20.04 is supported and also being used for CI.

## Getting started

After cloning the repo, first, set up the pre-configured RIOT/ESP32 toolchain:

```
$ make init
```

## examples/[esp32-no_std](https://github.com/AnimaGUS-minerva/iot-rust-module-studio/tree/main/examples/esp32-no_std)

A bare `no_std` [ESP32 firmware](https://github.com/AnimaGUS-minerva/iot-rust-module-studio/blob/main/examples/esp32-no_std/riot/main.c) with [a minimal Rust module](https://github.com/AnimaGUS-minerva/iot-rust-module-studio/blob/main/examples/esp32-no_std/src/lib.rs).  Use `make run` to (build and) run the firmware. To exit the `qemu-system-xtensa` based runner, type `Ctrl-a x`.

```
$ make run
```

Upon running the firmware, a binary file called 'riot.esp32.bin' is generated.  For test on the real ESP32 device, you can flash this image onto the device by following [the Espressif's manual](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/).

## examples/[xbd-base](https://github.com/AnimaGUS-minerva/iot-rust-module-studio/tree/main/examples/xbd-base)

A [cross-`BOARD` firmware](https://github.com/AnimaGUS-minerva/iot-rust-module-studio/blob/main/examples/xbd-base/riot/main.c) with [a demo Rust module](https://github.com/AnimaGUS-minerva/iot-rust-module-studio/blob/main/examples/xbd-base/src/lib.rs) featuring the "`semi_std`" support (`println!()`, `vec::*`, `Box`, `core2::io::*`, etc.). This example would be a convenient template for you to start developing a new RIOT-OS firmware in Rust.

Use `make run-native` (or `make run` as default) to run it as RIOT `native` firmware; or use `make run-esp32` for ESP32.

```
$ make run-native
```

```
$ make run-esp32
```

## examples/[xbd-micropython](https://github.com/AnimaGUS-minerva/iot-rust-module-studio/tree/main/examples/xbd-micropython)

This example demonstrates a cross-`BOARD` firmware running the [MicroPython](https://github.com/micropython/micropython) interpretor.  After bootstraping, a Rust module called 'libvoucher.a' (which [implements the RFC8995 Constrained Voucher](https://github.com/AnimaGUS-minerva/voucher)) is loaded as Python module, and then its APIs are unit-tested.

```
$ make run-native

[test] voucher.test_ffi : ✅
[test] voucher.get_voucher_jada : ✅
[test] voucher.get_voucher_F2_00_02 : ✅
[test] voucher.get_masa_pem_F2_00_02 : ✅
[test] no MemoryError for simple ops : ✅
@@ validating raw_voucher: [len=328]
[test] voucher.validate - jada : ✅
@@ validating raw_voucher with pem: [len=771] [len=684]
[test] voucher.validate - F2_00_02 : ✅
[test] voucher.get_key_pem_F2_00_02 : ✅
[test] voucher.get_device_crt_F2_00_02 : ✅
[test] bs_vrq_sample (with bare signature) : ✅
@@ validating raw_voucher with pem: [len=622] [len=644]
[test] voucher.{sign,validate} - vrq sample F2_00_02 via pubkey : ✅
@@ validating raw_voucher with pem: [len=622] [len=227]
[test] voucher.{sign,validate} - vrq sample F2_00_02 via privkey : ✅
[test] bs_vrq : ✅
@@ validating raw_voucher with pem: [len=555] [len=644]
[test] voucher.{sign,validate} - validating an unsigned voucher should fail : ✅
@@ vch_sign(): [len_raw=555] [len_key=227]
rnd_std_rand(): ^^
rnd_std_rand(): ^^
rnd_std_rand(): ^^
[test] bs_vrq_signed (with asn1 signature) : ✅
@@ validating raw_voucher with pem: [len=630] [len=644]
[test] voucher.{sign,validate} - vrq F2_00_02 via pubkey : ✅
@@ validating raw_voucher with pem: [len=630] [len=227]
[test] voucher.{sign,validate} - vrq F2_00_02 via privkey : ✅
-- boot.py exited. Starting REPL..
MicroPython v1.16-39-g503c0d317 on 2022-02-03; riot-native with native
Type "help()" for more information.
>>> ^C
native: exiting
```

```
$ make run-esp32

--snip--

>>> Quiting qemu...
```
