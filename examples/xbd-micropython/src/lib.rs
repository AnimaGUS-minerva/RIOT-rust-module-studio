#![no_std]
#![feature(alloc_error_handler)]

#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! { mcu_if::panic(info) }

#[cfg(not(feature = "std"))]
#[alloc_error_handler]
fn alloc_error(layout: mcu_if::alloc::alloc::Layout) -> ! { mcu_if::alloc_error(layout) }

//

#[cfg(feature = "std")]
#[macro_use]
extern crate std;

#[cfg(feature = "std")]
use std::{println, vec::Vec};
#[cfg(not(feature = "std"))]
use mcu_if::{println, alloc::vec::Vec};

//

#[cfg(test)]
mod tests;

//

#[cfg(feature = "v3")]
fn init_psa_crypto() {
    use minerva_mbedtls::psa_crypto;

    psa_crypto::init().unwrap();
    psa_crypto::initialized().unwrap();
}

//

use minerva_voucher::{Sign, Validate, SignatureAlgorithm};

#[cfg(not(any(feature = "x86", feature = "xtensa")))]
use minerva_voucher::Voucher; // for x86_64{,-lts}, {x86,xtensa}-lts
#[cfg(any(feature = "x86", feature = "xtensa"))]
use wip::Voucher;

#[cfg(any(feature = "x86", feature = "xtensa"))]
mod wip {
    use super::*;
    use minerva_voucher::Voucher as BaseVoucher;

    pub struct Voucher(BaseVoucher); // dummy `Voucher` without validation capability
    impl Voucher {
        pub fn from(raw: &[u8]) -> Self { Voucher(BaseVoucher::from(raw).unwrap()) }
    }
    impl core::ops::Deref for Voucher {
        type Target = BaseVoucher;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl Validate for Voucher {
        fn validate(&self, _masa_pem: Option<&[u8]>) -> bool {
            println!("⚠️ WIP -- `x86`, `xtensa` (v3 with PSA); validation fails for now!!");
            false
        }
    }
}

//

use mcu_if::utils::u8_slice_from;

static VOUCHER_JADA: &[u8] = core::include_bytes!(
    concat!(env!("CARGO_MANIFEST_DIR"), "/files/voucher_jada123456789.vch"));
static VOUCHER_F2_00_02: &[u8] = core::include_bytes!(
    concat!(env!("CARGO_MANIFEST_DIR"), "/files/voucher_00-D0-E5-F2-00-02.vch"));
static MASA_PEM_F2_00_02: &[u8] = core::include_bytes!(
    concat!(env!("CARGO_MANIFEST_DIR"), "/files/masa_00-D0-E5-F2-00-02.crt"));
static KEY_PEM_02_00_2E: &[u8] = core::include_bytes!(
    concat!(env!("CARGO_MANIFEST_DIR"), "/files/key_00-D0-E5-02-00-2E.pem"));
static DEVICE_CRT_02_00_2E: &[u8] = core::include_bytes!(
    concat!(env!("CARGO_MANIFEST_DIR"), "/files/device_00-D0-E5-02-00-2E.crt"));


#[no_mangle]
pub extern fn vch_get_voucher_jada(pp: *mut *const u8) -> usize {
    set_bytes_static(VOUCHER_JADA, pp)
}

#[no_mangle]
pub extern fn vch_get_voucher_F2_00_02(pp: *mut *const u8) -> usize {
    set_bytes_static(VOUCHER_F2_00_02, pp)
}

#[no_mangle]
pub extern fn vch_get_masa_pem_F2_00_02(pp: *mut *const u8) -> usize {
    set_bytes_static(MASA_PEM_F2_00_02, pp)
}

#[no_mangle]
pub extern fn vch_get_key_pem_02_00_2E(pp: *mut *const u8) -> usize {
    set_bytes_static(KEY_PEM_02_00_2E, pp)
}

#[no_mangle]
pub extern fn vch_get_device_crt_02_00_2E(pp: *mut *const u8) -> usize {
    set_bytes_static(DEVICE_CRT_02_00_2E, pp)
}

fn set_bytes_static(bytes: &[u8], pp: *mut *const u8) -> usize {
    let sz = bytes.len();
    unsafe { *pp = bytes.as_ptr(); }

    sz
}

fn set_bytes_heap(bytes: Vec<u8>, pp: *mut *const u8) -> usize {
    let sz = bytes.len();
    unsafe { *pp = bytes.as_ptr(); }

    core::mem::forget(bytes);

    sz
}

//

#[no_mangle]
pub extern fn vch_debug(ptr: *const u8, sz: usize) {
    let raw_voucher = u8_slice_from(ptr, sz);

    Voucher::from(raw_voucher).unwrap().dump()
}

//

#[no_mangle]
pub extern fn vch_validate(ptr: *const u8, sz: usize) -> bool {
    let raw_voucher = u8_slice_from(ptr, sz);
    println!("@@ validating raw_voucher: [len={}]", raw_voucher.len());

    Voucher::from(raw_voucher).unwrap().validate(None)
}

#[no_mangle]
pub extern fn vch_validate_with_pem(ptr: *const u8, sz: usize, ptr_pem: *const u8, sz_pem: usize) -> bool {
    let raw_voucher = u8_slice_from(ptr, sz);
    let pem = u8_slice_from(ptr_pem, sz_pem);
    println!("@@ validating raw_voucher with pem: [len={}] [len={}]", raw_voucher.len(), pem.len());

    Voucher::from(raw_voucher).unwrap().validate(Some(pem))
}

//

#[no_mangle]
pub extern fn vch_create_vrq_02_00_2E(pp: *mut *const u8) -> usize {
    let mut vch = Voucher::new();
    vch.set_content(&minerva_voucher::wip_vrhash_sidhash_content());

    set_bytes_heap(vch.serialize().unwrap(), pp)
}

#[no_mangle]
pub extern fn vch_sign(
    ptr_raw: *const u8, sz_raw: usize, ptr_key: *const u8, sz_key: usize,
    pp: *mut *const u8
) -> usize {
    let raw = u8_slice_from(ptr_raw, sz_raw);
    let key = u8_slice_from(ptr_key, sz_key);
    println!("@@ vch_sign(): [len_raw={}] [len_key={}]", raw.len(), key.len());

    let mut vch = Voucher::from(raw).unwrap();
    vch.sign(key, SignatureAlgorithm::ES256);

    set_bytes_heap(vch.serialize().unwrap(), pp)
}

//

#[no_mangle]
pub extern fn vch_square(input: i32) -> i32 {
    input * input
}
