#![no_std]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

pub use libc_print::libc_println as println;

extern "C" {
    fn abort() -> !;
    fn malloc(sz: u32) -> *mut core::ffi::c_void;
    fn free(ptr: *mut core::ffi::c_void);
}

//

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("🦀 panic(): ----");

    if let Some(message) = info.message() {
        println!("  message: \"{}\"", message);
    }

    if let Some(location) = info.location() {
        println!("  line: {}", location.line());
        println!("  file: '{}'", location.file());
    } else {
        println!("panic occurred but can't get location information...");
    }

    unsafe { abort(); }
}

//

extern crate alloc;

pub use alloc::{*, vec::Vec};

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("alloc_error_handler(): layout: {:?}", layout)
}

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

use alloc::alloc::{GlobalAlloc, Layout};

struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size() as u32) as *mut u8
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr as *mut core::ffi::c_void);
    }
}