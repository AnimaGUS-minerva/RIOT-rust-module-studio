#![no_std]

#[no_mangle]
pub extern fn square(input: i32) -> i32 {
    input * input
}

extern "C" {
    fn abort() -> !;
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { abort(); }
}
