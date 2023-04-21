#![no_std]
#![feature(alloc_error_handler)]

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! { mcu_if::panic(info) }

#[alloc_error_handler]
fn alloc_error(layout: mcu_if::alloc::alloc::Layout) -> ! { mcu_if::alloc_error(layout) }

use mcu_if::println;

#[no_mangle]
pub extern fn start() {
    println!("[src/lib.rs] start(): ^^");
    net_tests();
}

fn net_tests() {
    println!("net_tests(): ^^");
}
