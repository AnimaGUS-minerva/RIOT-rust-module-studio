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
use std::{println, vec::Vec, vec};
#[cfg(not(feature = "std"))]
use mcu_if::{println, alloc::{vec::Vec, vec}};

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
use minerva_voucher::{vrq, attr::*};
use core::convert::TryFrom;

#[cfg(not(any(feature = "x86", feature = "xtensa")))]
use minerva_voucher::Voucher; // for x86_64{,-lts}, {x86,xtensa}-lts
#[cfg(any(feature = "x86", feature = "xtensa"))]
use wip::Voucher;

#[cfg(any(feature = "x86", feature = "xtensa"))]
mod wip { // TODO adapt to voucher 0.6.0 -- `CARGO_FEATURES="--features xtensa"  make esp32-build-module`
    use super::*;
    use minerva_voucher::Voucher as BaseVoucher;

    pub struct Voucher(BaseVoucher); // dummy `Voucher` without validation capability
    impl Voucher {
        pub fn from(raw: &[u8]) -> Self { Voucher(BaseVoucher::try_from(raw).unwrap()) }
    }
    impl core::ops::Deref for Voucher {
        type Target = BaseVoucher;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl Validate for Voucher {
        fn validate(&self, _masa_pem: Option<&[u8]>) -> Result<&Self, VoucherError> {
            println!("⚠️ WIP -- `x86`, `xtensa` (v3 with PSA); validation fails for now!!");
            Err(VoucherError::ValidationFailed)
        }
    }
}

//

use mcu_if::utils::u8_slice_from;

static VOUCHER_JADA: &[u8] = core::include_bytes!(
    concat!(env!("CARGO_MANIFEST_DIR"), "/data/jada/voucher_jada123456789.vch"));
static VOUCHER_F2_00_02: &[u8] = core::include_bytes!(
    concat!(env!("CARGO_MANIFEST_DIR"), "/data/00-D0-E5-F2-00-02/voucher_00-D0-E5-F2-00-02.vch"));
static VOUCHER_REQUEST_F2_00_02: &[u8] = core::include_bytes!(
    concat!(env!("CARGO_MANIFEST_DIR"), "/data/00-D0-E5-F2-00-02/vr_00-D0-E5-F2-00-02.vrq"));
static MASA_PEM_F2_00_02: &[u8] = core::include_bytes!(
    concat!(env!("CARGO_MANIFEST_DIR"), "/data/00-D0-E5-F2-00-02/masa.crt"));
static KEY_PEM_F2_00_02: &[u8] = core::include_bytes!(
    concat!(env!("CARGO_MANIFEST_DIR"), "/data/00-D0-E5-F2-00-02/key.pem"));
static DEVICE_CRT_F2_00_02: &[u8] = core::include_bytes!(
    concat!(env!("CARGO_MANIFEST_DIR"), "/data/00-D0-E5-F2-00-02/device.crt"));


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
pub extern fn vch_get_key_pem_F2_00_02(pp: *mut *const u8) -> usize {
    set_bytes_static(KEY_PEM_F2_00_02, pp)
}

#[no_mangle]
pub extern fn vch_get_device_crt_F2_00_02(pp: *mut *const u8) -> usize {
    set_bytes_static(DEVICE_CRT_F2_00_02, pp)
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

    Voucher::try_from(raw_voucher).unwrap().dump()
}

//

#[no_mangle]
pub extern fn vch_validate(ptr: *const u8, sz: usize) -> bool {
    let raw_voucher = u8_slice_from(ptr, sz);
    println!("@@ validating raw_voucher: [len={}]", raw_voucher.len());

    Voucher::try_from(raw_voucher).unwrap().validate(None).is_ok()
}

#[no_mangle]
pub extern fn vch_validate_with_pem(ptr: *const u8, sz: usize, ptr_pem: *const u8, sz_pem: usize) -> bool {
    let raw_voucher = u8_slice_from(ptr, sz);
    let pem = u8_slice_from(ptr_pem, sz_pem);
    println!("@@ validating raw_voucher with pem: [len={}] [len={}]", raw_voucher.len(), pem.len());

    Voucher::try_from(raw_voucher).unwrap().validate(Some(pem)).is_ok()
}

//

#[no_mangle]
pub extern fn vch_get_vrq_F2_00_02(pp: *mut *const u8) -> usize {
    set_bytes_static(VOUCHER_REQUEST_F2_00_02, pp)
}

#[no_mangle]
pub extern fn vch_create_vrq_F2_00_02(pp: *mut *const u8) -> usize {
    let vrq = vrq![
        Attr::Assertion(Assertion::Proximity),
        Attr::CreatedOn(1599086034),
        Attr::Nonce(vec![48, 130, 1, 216, 48, 130, 1, 94, 160, 3, 2, 1, 2, 2, 1, 1, 48, 10, 6, 8, 42, 134, 72, 206, 61, 4, 3, 2, 48, 115, 49, 18, 48, 16, 6, 10, 9, 146, 38, 137, 147, 242, 44, 100, 1, 25, 22, 2, 99, 97, 49, 25, 48, 23, 6, 10, 9, 146, 38, 137, 147, 242, 44, 100, 1, 25, 22, 9, 115, 97, 110, 100, 101, 108, 109, 97, 110, 49, 66, 48, 64, 6, 3, 85, 4, 3, 12, 57, 35, 60, 83, 121, 115, 116, 101, 109, 86, 97, 114, 105, 97, 98, 108, 101, 58, 48, 120, 48, 48, 48, 48, 53, 53, 98, 56, 50, 53, 48, 99, 48, 100, 98, 56, 62, 32, 85, 110, 115, 116, 114, 117, 110, 103, 32, 70, 111, 117, 110, 116, 97, 105, 110, 32, 67, 65, 48, 30, 23, 13, 50, 48, 48, 56, 50, 57, 48, 52, 48, 48, 49, 54, 90, 23, 13, 50, 50, 48, 56, 50, 57, 48, 52, 48, 48, 49, 54, 90, 48, 70, 49, 18, 48, 16, 6, 10, 9, 146, 38, 137, 147, 242, 44, 100, 1, 25, 22, 2, 99, 97, 49, 25, 48, 23, 6, 10, 9, 146, 38, 137, 147, 242, 44, 100, 1, 25, 22, 9, 115, 97, 110, 100, 101, 108, 109, 97, 110, 49, 21, 48, 19, 6, 3, 85, 4, 3, 12, 12, 85, 110, 115, 116, 114, 117, 110, 103, 32, 74, 82, 67, 48, 89, 48, 19, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 8, 42, 134, 72, 206, 61, 3, 1, 7, 3, 66, 0, 4, 150, 101, 80, 114, 52, 186, 159, 229, 221, 230, 95, 246, 240, 129, 111, 233, 72, 158, 129, 12, 18, 7, 59, 70, 143, 151, 100, 43, 99, 0, 141, 2, 15, 87, 201, 124, 148, 127, 132, 140, 178, 14, 97, 214, 201, 136, 141, 21, 180, 66, 31, 215, 242, 106, 183, 228, 206, 5, 248, 167, 76, 211, 139, 58, 163, 16, 48, 14, 48, 12, 6, 3, 85, 29, 19, 1, 1, 255, 4, 2, 48, 0, 48, 10, 6, 8, 42, 134, 72, 206, 61, 4, 3, 2, 3, 104, 0, 48, 101, 2, 49, 0, 135, 158, 205, 227, 138, 5, 18, 46, 182, 247, 44, 178, 27, 195, 210, 92, 190, 230, 87, 55, 112, 86, 156, 236, 35, 12, 164, 140, 57, 241, 64, 77, 114, 212, 215, 85, 5, 155, 128, 130, 2, 14, 212, 29, 79, 17, 159, 231, 2, 48, 60, 20, 216, 138, 10, 252, 64, 71, 207, 31, 135, 184, 115, 193, 106, 40, 191, 184, 60, 15, 136, 67, 77, 157, 243, 247, 168, 110, 45, 198, 189, 136, 149, 68, 47, 32, 55, 237, 204, 228, 133, 91, 17, 218, 154, 25, 228, 232]),
        Attr::ProximityRegistrarCert(vec![102, 114, 118, 85, 105, 90, 104, 89, 56, 80, 110, 86, 108, 82, 75, 67, 73, 83, 51, 113, 77, 81]),
        Attr::SerialNumber(b"00-D0-E5-F2-00-02".to_vec())];

    set_bytes_heap(vrq.serialize().unwrap(), pp)
}

#[no_mangle]
pub extern fn vch_sign(
    ptr_raw: *const u8, sz_raw: usize, ptr_key: *const u8, sz_key: usize,
    pp: *mut *const u8
) -> usize {
    let raw = u8_slice_from(ptr_raw, sz_raw);
    let key = u8_slice_from(ptr_key, sz_key);
    println!("@@ vch_sign(): [len_raw={}] [len_key={}]", raw.len(), key.len());

    let mut vch = Voucher::try_from(raw).unwrap();
    vch.sign(key, SignatureAlgorithm::ES256).unwrap();

    set_bytes_heap(vch.serialize().unwrap(), pp)
}

//

#[no_mangle]
pub extern fn vch_square(input: i32) -> i32 {
    input * input
}
