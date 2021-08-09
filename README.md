# RIOT-rust-module-studio

## Repository map (planned)

```
/
  README.md
  crates/              .... currently supports mcu's specific to esp32 only
    mcu-emu            .... `qemu-system-xtensa` runner
    mcu-if             .... "semi_std" interface on top of bare `no_std`
  examples/
    esp32-no_std       .... `no_std` hello world from Rust & RIOT
    esp32-semi_std     .... `no_std` plus dynamic containers, smart pointers, etc.
    esp32-demo-gui     .... GUI app with u8g2/ssd1306 backend
    esp32-demo-gnrc    .... app that interacts with RIOT's networking
    esp32-demo-cose    .... ECDSA app that runs on mcu's
    native-std         .... dual stack (`no_std`/`std`) Rust crate development
    native-demo-gui    .... GUI app with lvgl/SDL2 backend
    native-demo-gnrc   .... app for dev/debug with RIOT's networking
    native-demo-cose   .... ECDSA app dev/debug on Linux
```
