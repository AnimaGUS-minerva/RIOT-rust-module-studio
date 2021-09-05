#![no_std]

//

#[cfg(feature = "std")]
#[macro_use]
extern crate std;

#[cfg(feature = "std")]
use std::println;
#[cfg(not(feature = "std"))]
use mcu_if::println;

//

#[cfg(test)]
mod tests;

//

mod voucher;
use crate::voucher::{Voucher, VOUCHER_JADA, VOUCHER_F2_00_02, MASA_PEM_F2_00_02};

use core::slice;

#[no_mangle]
pub extern fn vch_get_voucher_jada(ptr: *mut *const u8) -> usize {
    set_bytes(VOUCHER_JADA, ptr)
}

#[no_mangle]
pub extern fn vch_get_voucher_F2_00_02(ptr: *mut *const u8) -> usize {
    set_bytes(VOUCHER_F2_00_02, ptr)
}

#[no_mangle]
pub extern fn vch_get_masa_pem_F2_00_02(ptr: *mut *const u8) -> usize {
    set_bytes(MASA_PEM_F2_00_02, ptr)
}

fn set_bytes(bytes: &[u8], ptr: *mut *const u8) -> usize {
    unsafe { *ptr = bytes.as_ptr(); }

    bytes.len()
}

#[no_mangle]
pub extern fn vch_validate(ptr: *const u8, sz: usize) -> bool {
    let raw_voucher = u8_slice_from(ptr, sz);
    println!("@@ validating raw_voucher: [len={}] {:?}", raw_voucher.len(), raw_voucher);

    let vch = Voucher::from(raw_voucher);

    // debug
    let (sig, alg) = vch.get_signature();
    println!("sig len: {}", sig.len());
    println!("alg: {:?}", *alg);
    println!("signer cert len: {}", vch.get_signer_cert().unwrap().len());

    vch.validate(None)
}

#[no_mangle]
pub extern fn vch_validate_with_pem(ptr: *const u8, sz: usize, ptr_pem: *const u8, sz_pem: usize) -> bool {
    let raw_voucher = u8_slice_from(ptr, sz);
    let pem = u8_slice_from(ptr_pem, sz_pem);

    println!("@@ validating raw_voucher with pem: [len={}] [len={}]", raw_voucher.len(), pem.len());

    Voucher::from(raw_voucher).validate(Some(pem))
}

fn u8_slice_from(ptr: *const u8, sz: usize) -> &'static [u8] {
    unsafe { slice::from_raw_parts(ptr, sz) }
}

#[no_mangle]
pub extern fn vch_square(input: i32) -> i32 {
    input * input
}

//

#[macro_export]
macro_rules! null_terminate_bytes {
    ($bytes:expr) => ({
        let mut v = ($bytes).to_vec();
        v.push(0x00);
        v
    });
}

#[macro_export]
macro_rules! null_terminate_str { ($str:expr) => (crate::null_terminate_bytes!(($str).as_bytes())); }

#[macro_export]
macro_rules! cstr_from { ($str:expr) => (crate::null_terminate_str!($str).as_ptr() as *const i8); }
