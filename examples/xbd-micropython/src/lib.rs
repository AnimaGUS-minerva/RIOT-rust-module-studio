#![no_std]

use core::slice;

static VOUCHER_JADA: &[u8] = core::include_bytes!(
    concat!(env!("CARGO_MANIFEST_DIR"), "/files/voucher_jada123456789.vch"));
static VOUCHER_F2_00_02: &[u8] = core::include_bytes!(
    concat!(env!("CARGO_MANIFEST_DIR"), "/files/voucher_00-D0-E5-F2-00-02.vch"));
static MASA_PEM_F2_00_02: &[u8] = core::include_bytes!(
    concat!(env!("CARGO_MANIFEST_DIR"), "/files/masa_00-D0-E5-F2-00-02.crt"));

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

    libc_print::libc_println!("raw_voucher: {:?}", raw_voucher);

    if 1 == 1 { // TODO TEMP: hardcoded !!
        assert_eq!(raw_voucher.len(), 328);
        assert_eq!(raw_voucher, VOUCHER_JADA);
    }

    false // TODO
}

#[no_mangle]
pub extern fn vch_validate_with_pem(ptr: *const u8, sz: usize, ptr_pem: *const u8, sz_pem: usize) -> bool {
    let raw_voucher = u8_slice_from(ptr, sz);
    let pem = u8_slice_from(ptr_pem, sz_pem);

    if 1 == 1 { // TODO TEMP: hardcoded !!
        assert_eq!(raw_voucher.len(), 771);
        assert_eq!(raw_voucher, VOUCHER_F2_00_02);
        assert_eq!(pem.len(), 684);
        assert_eq!(pem, MASA_PEM_F2_00_02);
    }

    false // TODO
}

fn u8_slice_from(ptr: *const u8, sz: usize) -> &'static [u8] {
    unsafe { slice::from_raw_parts(ptr, sz) }
}

//

#[no_mangle]
pub extern fn vch_square(input: i32) -> i32 {
    input * input
}

extern "C" {
    fn abort() -> !;
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { abort(); }
}
