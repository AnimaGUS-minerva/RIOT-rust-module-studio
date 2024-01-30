use mcu_if::c_types::c_void;
use mcu_if::utils::{u8_slice_from, u8_slice_mut_from};

#[no_mangle]
pub extern fn xbd_kludge_update_blockwise_hdr(buf: *mut u8, buf_sz: usize) -> usize {
    let len = blockwise_hdr_len();
    if len > 0 {
        blockwise_hdr_copy(u8_slice_mut_from(buf, buf_sz));
        blockwise_hdr_clear();
    }

    len
}

#[no_mangle]
pub extern fn xbd_kludge_async_gcoap_get_blockwise(
    hdr: *const c_void, hdr_len: usize,
    last_uri: *const c_void, last_uri_len: usize) {
    use crate::xbd::gcoap::{ReqInner, COAP_METHOD_GET};

    blockwise_hdr_update(u8_slice_from(hdr as *const u8, hdr_len));

    ReqInner::add_blockwise(
        COAP_METHOD_GET, "[::1]:5683",
        "/const/song.txt",// !!!! fixme
//fixme        u8_slice_from(last_uri as *const u8, last_uri_len),
        None); // !!! 1111
}

//

#[no_mangle]
pub extern fn xbd_kludge_save_blockwise_uri(uri: *const c_void, uri_len: usize) {
    blockwise_uri_update(u8_slice_from(uri as *const u8, uri_len));
}

#[no_mangle]
pub extern fn xbd_kludge_get_blockwise_uri() -> *const c_void {
    unsafe { LAST_BLOCKWISE_URI.as_ptr() as _ }
}

/* Retain request path to re-request if response includes block. User must not
 * start a new request (with a new path) until any blockwise transfer
 * completes or times out. */
const LAST_BLOCKWISE_URI_MAX: usize = 64;
static mut LAST_BLOCKWISE_URI: &'static mut [u8] = &mut [0; LAST_BLOCKWISE_URI_MAX];

fn blockwise_uri_update(uri_slice: &[u8]) {// !!!! refactor
    let uri_len = uri_slice.len();
    assert!(uri_len < LAST_BLOCKWISE_URI_MAX);

    unsafe {
        LAST_BLOCKWISE_URI.fill(0u8);
        LAST_BLOCKWISE_URI[..uri_len].copy_from_slice(uri_slice);
    }
}

//

// TODO extension for multiple blockwise msg (ID) contexts
const LAST_BLOCKWISE_HDR_MAX: usize = 64;
static mut LAST_BLOCKWISE_HDR: &'static mut [u8] = &mut [0; LAST_BLOCKWISE_HDR_MAX];
static mut LAST_BLOCKWISE_LEN: usize = 0;

fn blockwise_hdr_update(hdr_slice: &[u8]) {// !!!! refactor
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