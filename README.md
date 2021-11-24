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
  crates/              .... 💡 currently supports mcu's specific to esp32 (and Linux native) only
    mcu-emu            .... emulator runner (`qemu-system-xtensa` or RIOT native board binary)
  examples/
    esp32-no_std       .... bare `no_std` firmware with a Rust module
    xbd-base           .... cross-`BOARD` (esp32/native) firmware with minimal 'librustmod.a'
    xbd-micropython    .... cross-`BOARD` firmware featuring MicroPython with 'libvoucher.a'
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

```
$ make run-native

[test] voucher.test_ffi : ✅
[test] voucher.get_voucher_jada : ✅
[test] voucher.get_voucher_F2_00_02 : ✅
[test] voucher.get_masa_pem_F2_00_02 : ✅
[test] no MemoryError for simple ops : ✅
@@ validating raw_voucher: [len=328]
⚠️ missing `signature_type`; ES256 is assumed
[test] voucher.validate - jada : ✅
@@ validating raw_voucher with pem: [len=771] [len=684]
[test] voucher.validate - F2_00_02 : ✅
[test] voucher.get_key_pem_02_00_2E : ✅
[test] voucher.get_device_crt_02_00_2E : ✅
bs_vrq [210, 132, 65, 160, 160, 88, 68, 161, 26, 0, 15, 70, 194, 164, 1, 105, 112, 114, 111, 120, 105, 109, 105, 116, 121, 2, 193, 26, 97, 119, 115, 164, 10, 81, 48, 48, 45, 68, 48, 45, 69, 53, 45, 48, 50, 45, 48, 48, 45, 50, 69, 7, 118, 114, 72, 103, 99, 66, 86, 78, 86, 97, 70, 109, 66, 87, 98, 84, 77, 109, 101, 79, 75, 117, 103, 64]
@@ validating raw_voucher with pem: [len=76] [len=761]
⚠️ missing `signature_type`; ES256 is assumed
[test] voucher.{sign,validate} - validating an unsigned voucher should fail : ✅
@@ vch_sign(): [len_raw=76] [len_key=227]
⚠️ missing `signature_type`; ES256 is assumed
bs_vrq_signed [210, 132, 65, 160, 160, 88, 68, 161, 26, 0, 15, 70, 194, 164, 1, 105, 112, 114, 111, 120, 105, 109, 105, 116, 121, 2, 193, 26, 97, 119, 115, 164, 10, 81, 48, 48, 45, 68, 48, 45, 69, 53, 45, 48, 50, 45, 48, 48, 45, 50, 69, 7, 118, 114, 72, 103, 99, 66, 86, 78, 86, 97, 70, 109, 66, 87, 98, 84, 77, 109, 101, 79, 75, 117, 103, 88, 72, 48, 70, 2, 33, 0, 226, 133, 204, 212, 146, 54, 173, 224, 191, 137, 104, 146, 5, 43, 216, 61, 167, 219, 192, 125, 138, 167, 160, 145, 26, 197, 52, 17, 94, 97, 210, 115, 2, 33, 0, 149, 230, 42, 127, 120, 31, 10, 28, 154, 2, 82, 16, 154, 165, 201, 129, 133, 192, 49, 15, 44, 159, 165, 129, 124, 210, 216, 67, 144, 174, 77, 107]
@@ validating raw_voucher with pem: [len=149] [len=761]
⚠️ missing `signature_type`; ES256 is assumed
[test] voucher.{sign,validate} - 02_00_2E via pubkey : ✅
@@ validating raw_voucher with pem: [len=149] [len=227]
⚠️ missing `signature_type`; ES256 is assumed
[test] voucher.{sign,validate} - 02_00_2E via privkey : ✅
-- boot.py exited. Starting REPL..
MicroPython v1.16-38-g5885fb219 on 2021-11-24; riot-native with native
Type "help()" for more information.
>>> Quiting native...
```

```
$ make run-esp32

--snip--

>>> Quiting qemu...
```
