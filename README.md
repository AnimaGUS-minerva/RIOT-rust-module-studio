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

### examples/xbd-base


### examples/xbd-micropython
