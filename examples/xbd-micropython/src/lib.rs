#![no_std]

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

//



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
