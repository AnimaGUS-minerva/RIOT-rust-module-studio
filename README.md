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
  README.md
  crates/              .... 💡 currently supports mcu's specific to esp32 (and Linux native) only
    mcu-emu            .... emulator runner (`qemu-system-xtensa` or RIOT native board binary)
    mcu-if             .... "semi_std" interface on top of bare `no_std`
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

A bare `no_std` [ESP32 firmware](https://github.com/AnimaGUS-minerva/iot-rust-module-studio/blob/main/examples/esp32-no_std/riot/main.c) with [a Rust module](https://github.com/AnimaGUS-minerva/iot-rust-module-studio/blob/main/examples/esp32-no_std/src/lib.rs).  Use `make run` to (build and) run the firmware. To exit from the qemu-xtensa based runner, type `Ctrl-a x`.

```
$ make run
```

## examples/[xbd-base](https://github.com/AnimaGUS-minerva/iot-rust-module-studio/tree/main/examples/xbd-base)

A [cross-`BOARD` (esp32/native) firmware](https://github.com/AnimaGUS-minerva/iot-rust-module-studio/blob/main/examples/xbd-base/riot/main.c) with a [demo Rust module with "`semi_std`" support](https://github.com/AnimaGUS-minerva/iot-rust-module-studio/blob/main/examples/xbd-base/src/lib.rs) (`println!()`, `vec::*`, `Box`, `core2::io::*`, etc.). This would be a convenient template to start developing your new RIOT-OS firmware in Rust.

Use `make run-native` (or `make run` as default) to (build and) run it as RIOT `native` firmware; or use `make run-esp32` for ESP32.

```
$ make run-native
```

```
$ make run-esp32
```

## examples/[xbd-micropython](https://github.com/AnimaGUS-minerva/iot-rust-module-studio/tree/main/examples/xbd-micropython)

```
$ make run-native
```

```
$ make run-esp32
```
