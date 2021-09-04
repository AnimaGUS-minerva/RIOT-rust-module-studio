#![no_std]

pub use libc_print::libc_println as println;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { abort(); }
}

extern "C" {
    fn abort() -> !;
}
