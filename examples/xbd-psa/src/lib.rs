#![no_std]
#![feature(alloc_error_handler)]

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! { mcu_if::panic(info) }

#[alloc_error_handler]
fn alloc_error(layout: mcu_if::alloc::alloc::Layout) -> ! { mcu_if::alloc_error(layout) }

//

use mcu_if::println;
use mcu_if::alloc::{vec, vec::Vec};

#[no_mangle]
pub extern fn square(input: i32) -> i32 {
    println!("[src/lib.rs] square(): input: {}", input);

    demo_psa();

    input * input
}

//

//use psa_crypto::{self, ffi};
//==== !!!!switch
use minerva_mbedtls::{psa_crypto, psa_ifce};
use psa_crypto::ffi;

fn demo_psa() {
    // ok
    println!("enum value of `ffi::MD_SHA256`: {:?}", ffi::MD_SHA256); // 4
    println!("enum value of `ffi::MD_SHA384`: {:?}", ffi::MD_SHA384); // 5
    println!("enum value of `ffi::MD_SHA512`: {:?}", ffi::MD_SHA512); // 6

    //====
    psa_crypto::init().unwrap(); // FIXME xtensa - "called `Result::unwrap()` on an `Err` value: InsufficientEntropy"
    psa_crypto::initialized().unwrap(); // !!
    let _info = md_info::from_type(MD_SHA256); // !!
    //====

    //

    println!("Vec::from([0, 1, 2]): {:?}", Vec::from([0, 1, 2]));
    println!("vec![0, 1, 2]: {:?}", vec![0, 1, 2]);
}

//

pub use ffi::{md_type_t, MD_SHA256};
pub struct md_info(*const ffi::md_info_t);
impl md_info {
    pub fn from_type(ty: ffi::md_type_t) -> Self {
        Self(unsafe { ffi::md_info_from_type(ty) })
    }
}
