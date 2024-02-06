use mcu_if::c_types::c_void;
use mcu_if::utils::{u8_slice_from, u8_slice_mut_from};

use core::{str::from_utf8, pin::Pin, task::{Context, Poll}};
use futures_util::{task::AtomicWaker, Stream};
use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;

use super::stream::XbdStream;
use super::gcoap::{ReqInner, COAP_METHOD_GET};

#[no_mangle]
pub extern fn xbd_blockwise_addr_ptr() -> *const c_void {
    unsafe { LAST_BLOCKWISE_ADDR.as_ptr() as _ }
}

#[no_mangle]
pub extern fn xbd_blockwise_uri_ptr() -> *const c_void {
    unsafe { LAST_BLOCKWISE_URI.as_ptr() as _ }
}

#[no_mangle]
pub extern fn xbd_blockwise_addr_update(addr: *const c_void, addr_len: usize) {
    let addr = u8_slice_from(addr as *const u8, addr_len);
    unsafe { blockwise_metadata_update(addr, LAST_BLOCKWISE_ADDR, LAST_BLOCKWISE_ADDR_MAX); }
}

#[no_mangle]
pub extern fn xbd_blockwise_uri_update(uri: *const c_void, uri_len: usize) {
    let uri = u8_slice_from(uri as *const u8, uri_len);
    unsafe { blockwise_metadata_update(uri, LAST_BLOCKWISE_URI, LAST_BLOCKWISE_URI_MAX); }
}

#[no_mangle]
pub extern fn xbd_blockwise_hdr_copy(buf: *mut u8, buf_sz: usize) -> usize {
    let len = blockwise_hdr_len();
    if len > 0 {
        blockwise_hdr_copy(u8_slice_mut_from(buf, buf_sz));
    }

    len
}

#[no_mangle]
pub extern fn xbd_blockwise_hdr_update(hdr: *const c_void, hdr_len: usize) {
    blockwise_hdr_update(u8_slice_from(hdr as *const u8, hdr_len));
}

#[no_mangle]
pub extern fn xbd_blockwise_async_gcoap_get(
    last_addr: *const c_void, last_addr_len: usize,
    last_uri: *const c_void, last_uri_len: usize)
{
    let addr = from_utf8(u8_slice_from(last_addr as *const u8, last_addr_len)).unwrap();
    let uri = from_utf8(u8_slice_from(last_uri as *const u8, last_uri_len)).unwrap();
    ReqInner::add_blockwise(COAP_METHOD_GET, addr, uri, None);
}

//

// FIXME
/* Retain request path to re-request if response includes block. User must not
 * start a new request (with a new path) until any blockwise transfer
 * completes or times out. */
const LAST_BLOCKWISE_ADDR_MAX: usize = 64;
const LAST_BLOCKWISE_URI_MAX: usize = 64;
static mut LAST_BLOCKWISE_ADDR: &'static mut [u8] = &mut [0; LAST_BLOCKWISE_ADDR_MAX];
static mut LAST_BLOCKWISE_URI: &'static mut [u8] = &mut [0; LAST_BLOCKWISE_URI_MAX];

// TODO extension for multiple blockwise msg (ID) contexts
const LAST_BLOCKWISE_HDR_MAX: usize = 64;
static mut LAST_BLOCKWISE_HDR: &'static mut [u8] = &mut [0; LAST_BLOCKWISE_HDR_MAX];
static mut LAST_BLOCKWISE_LEN: usize = 0;

fn blockwise_metadata_update(data_in: &[u8], data: &'static mut [u8], data_max: usize) -> usize {
    let data_len = data_in.len();
    assert!(data_len < data_max);

    data.fill(0u8);
    data[..data_len].copy_from_slice(data_in);

    data_len
}

fn blockwise_hdr_update(hdr: &[u8]) {
    unsafe {
        LAST_BLOCKWISE_LEN = blockwise_metadata_update(
            hdr, LAST_BLOCKWISE_HDR, LAST_BLOCKWISE_HDR_MAX);
    }
}

fn blockwise_hdr_len() -> usize {
    unsafe { LAST_BLOCKWISE_LEN }
}

fn blockwise_hdr_copy(buf: &mut [u8]) {
    unsafe {
        let len = LAST_BLOCKWISE_LEN;
        buf[..len].
            copy_from_slice(&LAST_BLOCKWISE_HDR[..len]);
    }
}

//

pub static BLOCKWISE_QUEUE: OnceCell<ArrayQueue<ReqInner>> = OnceCell::uninit();
pub static BLOCKWISE_WAKER: AtomicWaker = AtomicWaker::new();

pub fn add_blockwise_req(req: ReqInner) {
    XbdStream::add(&BLOCKWISE_QUEUE, &BLOCKWISE_WAKER, req);
}

pub struct BlockwiseStream(XbdStream<ReqInner>);

impl BlockwiseStream {
    pub fn new() -> Self {
        Self(XbdStream::new(&BLOCKWISE_QUEUE, &BLOCKWISE_WAKER))
    }
}

impl Stream for BlockwiseStream {
    type Item = ReqInner;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        unsafe {
            match Pin::get_unchecked_mut(self) {
                Self(inner) => Pin::new_unchecked(inner).poll_next(cx),
            }
        }
    }
}