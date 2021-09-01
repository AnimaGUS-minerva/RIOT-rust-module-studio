#![no_std]

#[no_mangle]
pub extern fn square(input: i32) -> i32 {
    input * input
}

#[no_mangle]
pub extern fn init_logger() {
    //panic!(); // TODO
}

#[no_mangle]
pub extern fn test_logger() {
    // TODO
}

extern "C" {
    fn abort() -> !;
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { abort(); }
}

