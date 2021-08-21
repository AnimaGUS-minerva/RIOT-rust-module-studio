# RIOT-rust-module-studio

[![MIT licensed][mit-badge]][mit-url]
[![CI][actions-badge]][actions-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/AnimaGUS-minerva/RIOT-rust-module-studio/blob/main/LICENSE
[actions-badge]: https://github.com/AnimaGUS-minerva/RIOT-rust-module-studio/workflows/CI/badge.svg
[actions-url]: https://github.com/AnimaGUS-minerva/RIOT-rust-module-studio/actions

Robust IoT development with Rust and RIOT-OS

## Getting started

```
$ make init  # set up toolchain
$ make test  # perform all tests for apps under examples/* 
```

## Environments

Ubuntu 20.04 is being used for CI.

## Repository map (planned)

```
/
  README.md
  crates/              .... 💡 currently supports mcu's specific to esp32 only
    mcu-emu            .... 🚧 `qemu-system-xtensa` runner
    mcu-if             .... 🚧 "semi_std" interface on top of bare `no_std`
  examples/
    esp32-no_std       .... ✅ `no_std` hello world from Rust & RIOT
    esp32-semi_std     .... 🚧 `no_std` plus dynamic containers, smart pointers, etc.
    esp32-demo-gui     .... GUI demo with u8g2/ssd1306 backend
    esp32-demo-gnrc    .... interacting with RIOT's networking
    esp32-demo-cose    .... ECDSA demo running on mcu's
    native-std         .... dual stack (`no_std`/`std`) Rust crate development
    native-demo-gui    .... GUI demo with lvgl/SDL2 backend
    native-demo-gnrc   .... app for dev/debug with RIOT's networking
    native-demo-cose   .... ECDSA app dev/debug on Linux
```
