use mcu_if::utils::u8_slice_mut_from;
use std::io;

#[no_mangle]
pub extern fn voucher_version_get_string_full(ptr: *mut u8, sz: usize) {
    let mut buf = u8_slice_mut_from(ptr, sz);

    let mut version: &[u8] = b"rust voucher x.x.x"; // WIP

    buf.fill(0u8);
    io::copy(&mut version, &mut buf).unwrap();
}