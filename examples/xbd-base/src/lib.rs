#![no_std]

use mcu_if::println;

#[no_mangle]
pub extern fn square(input: i32) -> i32 {
    println!("[src/lib.rs] square(): input: {}", input);

    input * input
}
