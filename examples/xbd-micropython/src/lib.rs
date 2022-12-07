#![no_std]
#![feature(alloc_error_handler)]

#[cfg(all(not(feature = "std"), not(test)))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! { mcu_if::panic(info) }

#[cfg(all(not(feature = "std"), not(test)))]
#[alloc_error_handler]
fn alloc_error(layout: mcu_if::alloc::alloc::Layout) -> ! { mcu_if::alloc_error(layout) }

//

#[cfg(feature = "std")]
#[macro_use]
extern crate std;

#[cfg(feature = "std")]
use std::{println, vec::Vec, vec, boxed::Box, os::raw::*};
#[cfg(not(feature = "std"))]
use mcu_if::{println, alloc::{vec::Vec, vec, boxed::Box}, c_types::*};

#[allow(non_camel_case_types,dead_code)]
type size_t = c_uint;

//

#[cfg(test)]
mod tests;

//

fn init_psa_crypto() {
    minerva_voucher::init_psa_crypto();
}

#[no_mangle]
pub extern fn vi_init_psa_crypto() {
    init_psa_crypto();
}

//

use minerva_voucher::{Voucher, Sign, Validate, SignatureAlgorithm};
use minerva_voucher::{vrq, attr::{self, *}};
use core::{convert::TryFrom, mem::ManuallyDrop};

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
pub extern fn vi_get_voucher_jada(pp: *mut *const u8) -> usize {
    set_bytes_static(VOUCHER_JADA, pp)
}

#[no_mangle]
pub extern fn vi_get_voucher_F2_00_02(pp: *mut *const u8) -> usize {
    set_bytes_static(VOUCHER_F2_00_02, pp)
}

#[no_mangle]
pub extern fn vi_get_masa_pem_F2_00_02(pp: *mut *const u8) -> usize {
    set_bytes_static(MASA_PEM_F2_00_02, pp)
}

#[no_mangle]
pub extern fn vi_get_key_pem_F2_00_02(pp: *mut *const u8) -> usize {
    set_bytes_static(KEY_PEM_F2_00_02, pp)
}

#[no_mangle]
pub extern fn vi_get_device_crt_F2_00_02(pp: *mut *const u8) -> usize {
    set_bytes_static(DEVICE_CRT_F2_00_02, pp)
}

fn set_bytes_static(bytes: &[u8], pp: *mut *const u8) -> usize {
    let sz = bytes.len();
    unsafe { *pp = bytes.as_ptr(); }

    sz
}

fn set_bytes_heap(bytes: Vec<u8>, pp: *mut *const u8) -> usize {
    let sz = bytes.len();
    if sz > 0 {
        unsafe { *pp = bytes.as_ptr(); }
        core::mem::forget(bytes);
    } else {
        unsafe { *pp = core::ptr::null(); }
    }

    sz
}

//

#[no_mangle]
pub extern fn vi_dump(ptr: *const u8, sz: usize) {
    let raw_voucher = u8_slice_from(ptr, sz);

    Voucher::try_from(raw_voucher).unwrap().dump()
}

//

#[no_mangle]
pub extern fn vi_square(input: i32) -> i32 {
    input * input
}

//

#[no_mangle]
pub extern fn vi_validate(ptr: *const u8, sz: usize) -> bool {
    let raw_voucher = u8_slice_from(ptr, sz);
    println!("@@ validating raw_voucher: [len={}]", raw_voucher.len());

    Voucher::try_from(raw_voucher).unwrap().validate(None).is_ok()
}

#[no_mangle]
pub extern fn vi_validate_with_pem(ptr: *const u8, sz: usize, ptr_pem: *const u8, sz_pem: usize) -> bool {
    let raw_voucher = u8_slice_from(ptr, sz);
    let pem = u8_slice_from(ptr_pem, sz_pem);
    println!("@@ validating raw_voucher with pem: [len={}] [len={}]", raw_voucher.len(), pem.len());

    Voucher::try_from(raw_voucher).unwrap().validate(Some(pem)).is_ok()
}

//

#[no_mangle]
pub extern fn vi_get_vrq_F2_00_02(pp: *mut *const u8) -> usize {
    set_bytes_static(VOUCHER_REQUEST_F2_00_02, pp)
}

#[no_mangle]
pub extern fn vi_create_vrq_F2_00_02(pp: *mut *const u8) -> usize {
    let vrq = vrq![
        Attr::Assertion(Assertion::Proximity),
        Attr::CreatedOn(1599086034),
        Attr::Nonce(vec![48, 130, 1, 216, 48, 130, 1, 94, 160, 3, 2, 1, 2, 2, 1, 1, 48, 10, 6, 8, 42, 134, 72, 206, 61, 4, 3, 2, 48, 115, 49, 18, 48, 16, 6, 10, 9, 146, 38, 137, 147, 242, 44, 100, 1, 25, 22, 2, 99, 97, 49, 25, 48, 23, 6, 10, 9, 146, 38, 137, 147, 242, 44, 100, 1, 25, 22, 9, 115, 97, 110, 100, 101, 108, 109, 97, 110, 49, 66, 48, 64, 6, 3, 85, 4, 3, 12, 57, 35, 60, 83, 121, 115, 116, 101, 109, 86, 97, 114, 105, 97, 98, 108, 101, 58, 48, 120, 48, 48, 48, 48, 53, 53, 98, 56, 50, 53, 48, 99, 48, 100, 98, 56, 62, 32, 85, 110, 115, 116, 114, 117, 110, 103, 32, 70, 111, 117, 110, 116, 97, 105, 110, 32, 67, 65, 48, 30, 23, 13, 50, 48, 48, 56, 50, 57, 48, 52, 48, 48, 49, 54, 90, 23, 13, 50, 50, 48, 56, 50, 57, 48, 52, 48, 48, 49, 54, 90, 48, 70, 49, 18, 48, 16, 6, 10, 9, 146, 38, 137, 147, 242, 44, 100, 1, 25, 22, 2, 99, 97, 49, 25, 48, 23, 6, 10, 9, 146, 38, 137, 147, 242, 44, 100, 1, 25, 22, 9, 115, 97, 110, 100, 101, 108, 109, 97, 110, 49, 21, 48, 19, 6, 3, 85, 4, 3, 12, 12, 85, 110, 115, 116, 114, 117, 110, 103, 32, 74, 82, 67, 48, 89, 48, 19, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 8, 42, 134, 72, 206, 61, 3, 1, 7, 3, 66, 0, 4, 150, 101, 80, 114, 52, 186, 159, 229, 221, 230, 95, 246, 240, 129, 111, 233, 72, 158, 129, 12, 18, 7, 59, 70, 143, 151, 100, 43, 99, 0, 141, 2, 15, 87, 201, 124, 148, 127, 132, 140, 178, 14, 97, 214, 201, 136, 141, 21, 180, 66, 31, 215, 242, 106, 183, 228, 206, 5, 248, 167, 76, 211, 139, 58, 163, 16, 48, 14, 48, 12, 6, 3, 85, 29, 19, 1, 1, 255, 4, 2, 48, 0, 48, 10, 6, 8, 42, 134, 72, 206, 61, 4, 3, 2, 3, 104, 0, 48, 101, 2, 49, 0, 135, 158, 205, 227, 138, 5, 18, 46, 182, 247, 44, 178, 27, 195, 210, 92, 190, 230, 87, 55, 112, 86, 156, 236, 35, 12, 164, 140, 57, 241, 64, 77, 114, 212, 215, 85, 5, 155, 128, 130, 2, 14, 212, 29, 79, 17, 159, 231, 2, 48, 60, 20, 216, 138, 10, 252, 64, 71, 207, 31, 135, 184, 115, 193, 106, 40, 191, 184, 60, 15, 136, 67, 77, 157, 243, 247, 168, 110, 45, 198, 189, 136, 149, 68, 47, 32, 55, 237, 204, 228, 133, 91, 17, 218, 154, 25, 228, 232]),
        Attr::ProximityRegistrarCert(vec![102, 114, 118, 85, 105, 90, 104, 89, 56, 80, 110, 86, 108, 82, 75, 67, 73, 83, 51, 113, 77, 81]),
        Attr::SerialNumber(b"00-D0-E5-F2-00-02".to_vec())];

    set_bytes_heap(vrq.serialize().unwrap(), pp)
}

#[no_mangle]
pub extern fn vi_sign(
    ptr_raw: *const u8, sz_raw: usize, ptr_key: *const u8, sz_key: usize,
    pp: *mut *const u8, alg: u8
) -> usize {
    let raw = u8_slice_from(ptr_raw, sz_raw);
    let key = u8_slice_from(ptr_key, sz_key);
    println!("@@ vi_sign(): [len_raw={}] [len_key={}]", raw.len(), key.len());

    let mut vch = Voucher::try_from(raw).unwrap();
    vch.sign(key, resolve_alg(alg).unwrap()).unwrap();

    set_bytes_heap(vch.serialize().unwrap(), pp)
}

//

type ProviderPtr = *const c_void;

fn get_voucher_ref(ptr: ProviderPtr) -> &'static Voucher {
    assert_ne!(ptr, core::ptr::null());

    unsafe { & *(ptr as *const Voucher) }
}

fn get_voucher_mut(ptr: ProviderPtr) -> &'static mut Voucher {
    assert_ne!(ptr, core::ptr::null());

    unsafe { &mut *(ptr as *mut Voucher) }
}

fn provider_allocate(pp: *mut ProviderPtr, vou: Voucher) {
    let ptr = ManuallyDrop::new(Box::pin(vou)).as_ref().get_ref()
        as *const Voucher as ProviderPtr;

    assert_eq!(unsafe { *pp }, core::ptr::null());
    unsafe { *pp = ptr; }
}

#[no_mangle]
pub extern fn vi_provider_allocate(pp: *mut ProviderPtr, is_vrq: bool) {
    let vou = if is_vrq { Voucher::new_vrq() } else { Voucher::new_vch() };
    provider_allocate(pp, vou);
}

#[no_mangle]
pub extern fn vi_provider_allocate_from_cbor(pp: *mut ProviderPtr, buf: *const u8, sz: usize) -> bool {
    let cbor = u8_slice_from(buf, sz);

    if let Ok(vou) = Voucher::try_from(cbor) {
        provider_allocate(pp, vou);
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern fn vi_provider_free(pp: *mut ProviderPtr) {
    let null = core::ptr::null();

    let ptr = unsafe { *pp };
    assert_ne!(ptr, null);

    drop(unsafe { Box::from_raw(ptr as *mut Voucher) });

    unsafe { *pp = null; }
}

#[no_mangle]
pub extern fn vi_provider_is_vrq(ptr: ProviderPtr) -> bool {
    get_voucher_ref(ptr).is_vrq()
}

#[no_mangle]
pub extern fn vi_provider_to_cbor(ptr: ProviderPtr, pp: *mut *const u8) -> usize {
    set_bytes_heap(get_voucher_ref(ptr).serialize().unwrap_or(vec![]), pp)
}

#[no_mangle]
pub extern fn vi_provider_dump(ptr: ProviderPtr) {
    get_voucher_ref(ptr).dump();
}

#[no_mangle]
pub extern fn vi_provider_len(ptr: ProviderPtr) -> usize {
    get_voucher_ref(ptr).len()
}

//

fn set_inner(ptr: ProviderPtr, attr: Option<Attr>) -> bool {
    if let Some(attr) = attr {
        get_voucher_mut(ptr).set(attr);
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern fn vi_provider_set_attr_int(ptr: ProviderPtr, attr_key: u8, attr_val: u64) -> bool {
    use Attr::*;
    println!("@@ vi_provider_set_attr_int(): attr_key: {} | attr_val: {}", attr_key, attr_val);

    set_inner(ptr, match attr_key {
        ATTR_ASSERTION => match attr_val {
            0 => Some(Assertion(attr::Assertion::Verified)),
            1 => Some(Assertion(attr::Assertion::Logged)),
            2 => Some(Assertion(attr::Assertion::Proximity)),
            _ => None,
        },
        ATTR_CREATED_ON => Some(CreatedOn(attr_val)),
        ATTR_EXPIRES_ON => Some(ExpiresOn(attr_val)),
        ATTR_LAST_RENEWAL_DATE => Some(LastRenewalDate(attr_val)),
        _ => None,
    })
}

#[no_mangle]
pub extern fn vi_provider_set_attr_bool(ptr: ProviderPtr, attr_key: u8, attr_val: bool) -> bool {
    use Attr::*;
    println!("@@ vi_provider_set_attr_bool(): attr_key: {} | attr_val: {}", attr_key, attr_val);

    set_inner(ptr, match attr_key {
        ATTR_DOMAIN_CERT_REVOCATION_CHECKS => Some(DomainCertRevocationChecks(attr_val)),
        _ => None,
    })
}

#[no_mangle]
pub extern fn vi_provider_set_attr_bytes(ptr: ProviderPtr, attr_key: u8, buf: *const u8, sz: usize) -> bool {
    use Attr::*;
    let bytes = u8_slice_from(buf, sz).to_vec();

    set_inner(ptr, match attr_key {
        ATTR_IDEVID_ISSUER => Some(IdevidIssuer(bytes)),
        ATTR_NONCE => Some(Nonce(bytes)),
        ATTR_PINNED_DOMAIN_CERT => Some(PinnedDomainCert(bytes)),
        ATTR_PINNED_DOMAIN_PUBK => Some(PinnedDomainPubk(bytes)),
        ATTR_PINNED_DOMAIN_PUBK_SHA256 => Some(PinnedDomainPubkSha256(bytes)),
        ATTR_PRIOR_SIGNED_VOUCHER_REQUEST => Some(PriorSignedVoucherRequest(bytes)),
        ATTR_PROXIMITY_REGISTRAR_CERT => Some(ProximityRegistrarCert(bytes)),
        ATTR_PROXIMITY_REGISTRAR_PUBK => Some(ProximityRegistrarPubk(bytes)),
        ATTR_PROXIMITY_REGISTRAR_PUBK_SHA256 => Some(ProximityRegistrarPubkSha256(bytes)),
        ATTR_SERIAL_NUMBER => Some(SerialNumber(bytes)),
        _ => None,
    })
}

//

#[no_mangle]
pub extern fn vi_provider_has_attr_int(ptr: ProviderPtr, attr_key: u8) -> bool {
    vi_provider_get_int(ptr, attr_key).is_some()
}

#[no_mangle]
pub extern fn vi_provider_has_attr_bool(ptr: ProviderPtr, attr_key: u8) -> bool {
    vi_provider_get_bool(ptr, attr_key).is_some()
}

#[no_mangle]
pub extern fn vi_provider_has_attr_bytes(ptr: ProviderPtr, attr_key: u8) -> bool {
    vi_provider_get_bytes(ptr, attr_key).is_some()
}

#[no_mangle]
pub extern fn vi_provider_get_attr_int_or_panic(ptr: ProviderPtr, attr_key: u8) -> u64 {
    vi_provider_get_int(ptr, attr_key).unwrap()
}

#[no_mangle]
pub extern fn vi_provider_get_attr_bool_or_panic(ptr: ProviderPtr, attr_key: u8) -> bool {
    vi_provider_get_bool(ptr, attr_key).unwrap()
}

#[no_mangle]
pub extern fn vi_provider_get_attr_bytes_or_panic(ptr: ProviderPtr, attr_key: u8, pp: *mut *const u8) -> usize {
    set_bytes_heap(vi_provider_get_bytes(ptr, attr_key).unwrap().to_vec(), pp)
}

fn vi_provider_get_int(ptr: ProviderPtr, attr_key: u8) -> Option<u64> {
    use Attr::*;

    match get_voucher_ref(ptr).get(attr_key) {
        Some(Assertion(attr::Assertion::Verified)) => Some(0),
        Some(Assertion(attr::Assertion::Logged)) => Some(1),
        Some(Assertion(attr::Assertion::Proximity)) => Some(2),
        Some(CreatedOn(val)) => Some(*val),
        Some(ExpiresOn(val)) => Some(*val),
        Some(LastRenewalDate(val)) => Some(*val),
        _ => None,
    }
}

fn vi_provider_get_bool(ptr: ProviderPtr, attr_key: u8) -> Option<bool> {
    use Attr::*;

    match get_voucher_ref(ptr).get(attr_key) {
        Some(DomainCertRevocationChecks(val)) => Some(*val),
        _ => None,
    }
}

fn vi_provider_get_bytes(ptr: ProviderPtr, attr_key: u8) -> Option<&'static [u8]> {
    use Attr::*;

    match get_voucher_ref(ptr).get(attr_key) {
        Some(IdevidIssuer(x)) => Some(x),
        Some(Nonce(x)) => Some(x),
        Some(PinnedDomainCert(x)) => Some(x),
        Some(PinnedDomainPubk(x)) => Some(x),
        Some(PinnedDomainPubkSha256(x)) => Some(x),
        Some(PriorSignedVoucherRequest(x)) => Some(x),
        Some(ProximityRegistrarCert(x)) => Some(x),
        Some(ProximityRegistrarPubk(x)) => Some(x),
        Some(ProximityRegistrarPubkSha256(x)) => Some(x),
        Some(SerialNumber(x)) => Some(x),
        _ => None,
    }
}

//

#[no_mangle]
pub extern fn vi_provider_remove_attr(ptr: ProviderPtr, attr_key: u8) -> bool {
    get_voucher_mut(ptr).remove(attr_key)
}

#[no_mangle]
pub extern fn vi_provider_attr_key_at(ptr: ProviderPtr, n: usize) -> u8 {
    get_voucher_ref(ptr).iter().nth(n).unwrap().disc()
}

//

#[no_mangle]
pub extern fn vi_provider_sign(ptr: ProviderPtr, ptr_key: *const u8, sz_key: usize, alg: u8) -> bool {
    let key = u8_slice_from(ptr_key, sz_key);
    println!("@@ vi_provider_sign(): [len_key={}]", key.len());

    if let Some(alg) = resolve_alg(alg) {
        get_voucher_mut(ptr).sign(key, alg).is_ok()
    } else {
        println!("@@ vi_provider_sign(): invalid `alg`: {}", alg);
        false
    }
}

fn resolve_alg(alg: u8) -> Option<SignatureAlgorithm> {
    match alg {
        0 => Some(SignatureAlgorithm::ES256),
        1 => Some(SignatureAlgorithm::ES384),
        2 => Some(SignatureAlgorithm::ES512),
        3 => Some(SignatureAlgorithm::PS256),
        _ => None,
    }
}

#[no_mangle]
pub extern fn vi_provider_validate(ptr: ProviderPtr) -> bool {
    get_voucher_ref(ptr).validate(None).is_ok()
}

#[no_mangle]
pub extern fn vi_provider_validate_with_pem(ptr: ProviderPtr, ptr_pem: *const u8, sz_pem: usize) -> bool {
    let pem = u8_slice_from(ptr_pem, sz_pem);
    get_voucher_ref(ptr).validate(Some(pem)).is_ok()
}

//

#[no_mangle]
pub extern fn vi_provider_get_signer_cert(ptr: ProviderPtr, pp: *mut *const u8) -> usize {
    let cert = get_voucher_ref(ptr)
        .get_signer_cert()
        .map(|x| x.to_vec())
        .unwrap_or(vec![]);

    set_bytes_heap(cert, pp)
}

#[no_mangle]
pub extern fn vi_provider_set_signer_cert(ptr: ProviderPtr, buf: *const u8, sz: usize) {
    get_voucher_mut(ptr).set_signer_cert(u8_slice_from(buf, sz));
}

#[no_mangle]
pub extern fn vi_provider_get_content(ptr: ProviderPtr, pp: *mut *const u8) -> usize {
    set_bytes_heap(get_voucher_ref(ptr).to_validate().2.to_vec(), pp)
}

#[no_mangle]
pub extern fn vi_provider_get_signature_bytes(ptr: ProviderPtr, pp: *mut *const u8) -> usize {
    let sig = get_voucher_ref(ptr).to_validate().1
        .map(|x| x.0.to_vec())
        .unwrap_or(vec![]);

    set_bytes_heap(sig, pp)
}

#[no_mangle]
pub extern fn vi_provider_get_signature_alg(ptr: ProviderPtr) -> u8 {
    match get_voucher_ref(ptr).to_validate().1.unwrap().1 {
        SignatureAlgorithm::ES256 => 0,
        SignatureAlgorithm::ES384 => 1,
        SignatureAlgorithm::ES512 => 2,
        SignatureAlgorithm::PS256 => 3,
    }
}

//