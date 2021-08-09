#![no_std]

#[no_mangle]
pub extern fn double_input(input: i32) -> i32 {
    input * 2
}

extern "C" {
    fn abort() -> !;
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { abort(); }
}
