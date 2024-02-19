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
pub extern fn xbd_blockwise_async_gcoap_req(
    last_addr: *const c_void, last_addr_len: usize,
    last_uri: *const c_void, last_uri_len: usize)
{
    let addr = from_utf8(u8_slice_from(last_addr as *const u8, last_addr_len)).unwrap();
    let uri = from_utf8(u8_slice_from(last_uri as *const u8, last_uri_len)).unwrap();
    let req = ReqInner::new(COAP_METHOD_GET, addr, uri, None, true);

    add_blockwise_req(Some(req)); // !! todo use add_blockwise_req_generic
}

#[no_mangle]
pub extern fn xbd_blockwise_async_gcoap_complete() {
    add_blockwise_req(None); // !! todo use add_blockwise_req_generic
}

//

const LAST_BLOCKWISE_ADDR_MAX: usize = 64;
const LAST_BLOCKWISE_URI_MAX: usize = 64;
static mut LAST_BLOCKWISE_ADDR: &'static mut [u8] = &mut [0; LAST_BLOCKWISE_ADDR_MAX];
static mut LAST_BLOCKWISE_URI: &'static mut [u8] = &mut [0; LAST_BLOCKWISE_URI_MAX];

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

//---- !!!! POC hardcoded ^^
#[no_mangle]
pub extern fn xbd_blockwise_2_addr_ptr() -> *const c_void {
    unsafe { LAST_BLOCKWISE_2_ADDR.as_ptr() as _ }
}

#[no_mangle]
pub extern fn xbd_blockwise_2_uri_ptr() -> *const c_void {
    unsafe { LAST_BLOCKWISE_2_URI.as_ptr() as _ }
}

#[no_mangle]
pub extern fn xbd_blockwise_2_addr_update(addr: *const c_void, addr_len: usize) {
    let addr = u8_slice_from(addr as *const u8, addr_len);
    unsafe { blockwise_metadata_update(addr, LAST_BLOCKWISE_2_ADDR, LAST_BLOCKWISE_2_ADDR_MAX); }
}

#[no_mangle]
pub extern fn xbd_blockwise_2_uri_update(uri: *const c_void, uri_len: usize) {
    let uri = u8_slice_from(uri as *const u8, uri_len);
    unsafe { blockwise_metadata_update(uri, LAST_BLOCKWISE_2_URI, LAST_BLOCKWISE_2_URI_MAX); }
}

#[no_mangle]
pub extern fn xbd_blockwise_2_hdr_copy(buf: *mut u8, buf_sz: usize) -> usize {
    let len = blockwise_2_hdr_len();
    if len > 0 {
        blockwise_2_hdr_copy(u8_slice_mut_from(buf, buf_sz));
    }

    len
}

#[no_mangle]
pub extern fn xbd_blockwise_2_hdr_update(hdr: *const c_void, hdr_len: usize) {
    blockwise_2_hdr_update(u8_slice_from(hdr as *const u8, hdr_len));
}

#[no_mangle]
pub extern fn xbd_blockwise_2_async_gcoap_req(
    last_addr: *const c_void, last_addr_len: usize,
    last_uri: *const c_void, last_uri_len: usize)
{
    let addr = from_utf8(u8_slice_from(last_addr as *const u8, last_addr_len)).unwrap();
    let uri = from_utf8(u8_slice_from(last_uri as *const u8, last_uri_len)).unwrap();
    let req = ReqInner::new_2(COAP_METHOD_GET, addr, uri, None, true);

    add_blockwise_2_req(Some(req)); // !! todo use add_blockwise_req_generic
}

#[no_mangle]
pub extern fn xbd_blockwise_2_async_gcoap_complete() {
    add_blockwise_2_req(None); // !! todo use add_blockwise_req_generic
}

const LAST_BLOCKWISE_2_ADDR_MAX: usize = 64;
const LAST_BLOCKWISE_2_URI_MAX: usize = 64;
static mut LAST_BLOCKWISE_2_ADDR: &'static mut [u8] = &mut [0; LAST_BLOCKWISE_2_ADDR_MAX];
static mut LAST_BLOCKWISE_2_URI: &'static mut [u8] = &mut [0; LAST_BLOCKWISE_2_URI_MAX];

const LAST_BLOCKWISE_2_HDR_MAX: usize = 64;
static mut LAST_BLOCKWISE_2_HDR: &'static mut [u8] = &mut [0; LAST_BLOCKWISE_2_HDR_MAX];
static mut LAST_BLOCKWISE_2_LEN: usize = 0;

fn blockwise_2_hdr_update(hdr: &[u8]) {
    unsafe {
        LAST_BLOCKWISE_2_LEN = blockwise_metadata_update(
            hdr, LAST_BLOCKWISE_2_HDR, LAST_BLOCKWISE_2_HDR_MAX);
    }
}

fn blockwise_2_hdr_len() -> usize {
    unsafe { LAST_BLOCKWISE_2_LEN }
}

fn blockwise_2_hdr_copy(buf: &mut [u8]) {
    unsafe {
        let len = LAST_BLOCKWISE_2_LEN;
        buf[..len].
            copy_from_slice(&LAST_BLOCKWISE_2_HDR[..len]);
    }
}
//---- !!!! POC hardcoded $$

//

pub static BLOCKWISE_QUEUE: OnceCell<ArrayQueue<Option<ReqInner>>> = OnceCell::uninit();
pub static BLOCKWISE_WAKER: AtomicWaker = AtomicWaker::new();

pub fn add_blockwise_req(req: Option<ReqInner>) { // !! todo use add_blockwise_req_generic
    XbdStream::add(&BLOCKWISE_QUEUE, &BLOCKWISE_WAKER, req);
}
//---- !!!! POC hardcoded ^^
pub static BLOCKWISE_2_QUEUE: OnceCell<ArrayQueue<Option<ReqInner>>> = OnceCell::uninit();
pub static BLOCKWISE_2_WAKER: AtomicWaker = AtomicWaker::new();

pub fn add_blockwise_2_req(req: Option<ReqInner>) {
    XbdStream::add(&BLOCKWISE_2_QUEUE, &BLOCKWISE_2_WAKER, req);
}
//---- !!!! POC hardcoded $$
const BLOCKWISE_STATES_MAX: usize = 4;
//pub static BLOCKWISE_STATES: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
// !!!! (idx: u8) -> (blockwise_nested_id: Option<isize>, metadata: _)

static mut STATES00: &'static mut [(Option<usize>, u8)] = &mut [(None, 42); BLOCKWISE_STATES_MAX];

pub fn add_blockwise_req_generic(addr: &str, uri: &str) -> Option<BlockwiseStream> {
/*
    if BLOCKWISE_STATES.get().is_none() {
        BLOCKWISE_STATES
            .try_init_once(|| ArrayQueue::new(BLOCKWISE_STATES_MAX))
            .unwrap();
    }
    let stats = BLOCKWISE_STATES.get().unwrap();
*/

    (unsafe { &mut STATES00 })[0] = (Some(0), 99); // !!!! KLUDGE placeholder

    if let Some((idx, stat)) = unsafe { &mut STATES00 }.iter_mut().enumerate().find(|x| x.1.0.is_none()) {
        *stat = (Some(idx), 43);
        crate::println!("sending, where STATES00: {:?}", unsafe { &STATES00 });
        //panic!("!!");

        match idx { // !!!! KLUDGE hardcoded
            0 => todo!(),
            1 => {
                let bs = BlockwiseStream::get(&BLOCKWISE_QUEUE, &BLOCKWISE_WAKER);
                XbdStream::add(&BLOCKWISE_QUEUE, &BLOCKWISE_WAKER,
                               Some(ReqInner::new(COAP_METHOD_GET, addr, uri, None, true)));

                Some(bs)
            },
            2 => {
                let bs = BlockwiseStream::get(&BLOCKWISE_2_QUEUE, &BLOCKWISE_2_WAKER);
                XbdStream::add(&BLOCKWISE_2_QUEUE, &BLOCKWISE_2_WAKER,
                               Some(ReqInner::new_2(COAP_METHOD_GET, addr, uri, None, true)));

                Some(bs)
            },
            _ => unreachable!(),
        }
    } else {
        return None;
    }
}
//---- !!!! POC generic $$

#[derive(Debug)]
pub struct BlockwiseStream(XbdStream<Option<ReqInner>>);

impl BlockwiseStream {
    pub fn get(queue: &'static OnceCell<ArrayQueue<Option<ReqInner>>>, waker: &'static AtomicWaker) -> Self {
        XbdStream::get(&queue, &waker).map_or_else(
            || Self(XbdStream::new_with_cap(&queue, &waker, 1)), |xs| Self(xs))
    }
}

impl Stream for BlockwiseStream {
    type Item = Option<ReqInner>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        unsafe {
            match Pin::get_unchecked_mut(self) {
                Self(inner) => Pin::new_unchecked(inner).poll_next(cx),
            }
        }
    }
}