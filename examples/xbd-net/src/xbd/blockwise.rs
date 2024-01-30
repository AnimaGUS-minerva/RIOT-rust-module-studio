use mcu_if::c_types::c_void;

#[no_mangle]
pub extern fn xbd_kludge_update_blockwise_hdr(buf: *mut u8, buf_sz: usize) -> usize {
    use mcu_if::utils::u8_slice_mut_from;

    let len = blockwise_hdr_len();
    if len > 0 {
        blockwise_hdr_copy(u8_slice_mut_from(buf, buf_sz));
        blockwise_hdr_clear();
    }

    len
}

#[no_mangle]
pub extern fn xbd_kludge_async_gcoap_get_blockwise(hdr: *const c_void, hdr_len: usize) {
    use crate::xbd::gcoap::{ReqInner, COAP_METHOD_GET};
    use mcu_if::utils::u8_slice_from;

    blockwise_hdr_update(u8_slice_from(hdr as *const u8, hdr_len));

    ReqInner::add_blockwise(
        COAP_METHOD_GET, "[::1]:5683", "/const/song.txt", None); // !!! 1111 cf. _last_req_path
}

//

// TODO extension for multiple blockwise msg (ID) contexts
const LAST_BLOCKWISE_HDR_MAX: usize = 64;
static mut LAST_BLOCKWISE_HDR: &'static mut [u8] = &mut [0; LAST_BLOCKWISE_HDR_MAX];
static mut LAST_BLOCKWISE_LEN: usize = 0;

fn blockwise_hdr_update(hdr_slice: &[u8]) {
    let hdr_len = hdr_slice.len();
    assert!(hdr_len < LAST_BLOCKWISE_HDR_MAX);

    unsafe {
        LAST_BLOCKWISE_HDR.fill(0u8);
        LAST_BLOCKWISE_HDR[..hdr_len].copy_from_slice(hdr_slice);
        LAST_BLOCKWISE_LEN = hdr_len;
    }
}

fn blockwise_hdr_len() -> usize {
    unsafe { LAST_BLOCKWISE_LEN }
}

fn blockwise_hdr_copy(buf_slice: &mut [u8]) {
    unsafe {
        let len = LAST_BLOCKWISE_LEN;
        buf_slice[..len].
            copy_from_slice(&LAST_BLOCKWISE_HDR[..len]);
    }
}

fn blockwise_hdr_clear() {
    unsafe {
        LAST_BLOCKWISE_HDR.fill(0u8);
        LAST_BLOCKWISE_LEN = 0;
    }
}