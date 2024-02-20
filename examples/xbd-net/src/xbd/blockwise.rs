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
    last_uri: *const c_void, last_uri_len: usize,
    blockwise_state_index: usize)
{
    let addr = from_utf8(u8_slice_from(last_addr as *const u8, last_addr_len)).unwrap();
    let uri = from_utf8(u8_slice_from(last_uri as *const u8, last_uri_len)).unwrap();
    add_blockwise_req_generic(Some((addr, uri)), Some(blockwise_state_index));
}

#[no_mangle]
pub extern fn xbd_blockwise_async_gcoap_complete(blockwise_state_index: usize) {
    add_blockwise_req_generic(None, Some(blockwise_state_index));
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
//---- !!!! POC hardcoded ^^
pub static BLOCKWISE_2_QUEUE: OnceCell<ArrayQueue<Option<ReqInner>>> = OnceCell::uninit();
pub static BLOCKWISE_2_WAKER: AtomicWaker = AtomicWaker::new();
//---- !!!! POC hardcoded $$
const BLOCKWISE_STATES_MAX: usize = 4;
//pub static BLOCKWISE_STATES: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

type Stat = u8; // !! todo
static mut STATES00: &'static mut [Option<Stat>] = &mut [None; BLOCKWISE_STATES_MAX];

pub fn add_blockwise_req_generic(
    addr_uri: Option<(&str, &str)>, blockwise_state_index: Option<usize>) -> Option<BlockwiseStream> {

    if let Some(idx) = blockwise_state_index {
        let (queue, waker) = match idx { // KLUDGE !! todo
            0 => todo!(),
            1 => (&BLOCKWISE_QUEUE, &BLOCKWISE_WAKER),
            2 => (&BLOCKWISE_2_QUEUE, &BLOCKWISE_2_WAKER),
            _ => unreachable!(),
        };

        if let Some((addr, uri)) = addr_uri { // blockwise NEXT
            let req = ReqInner::new(COAP_METHOD_GET, addr, uri, None, true, Some(idx));
            XbdStream::add(queue, waker, Some(req));
        } else { // blockwise COMPLETE
            XbdStream::add(queue, waker, None);
        }

        return None;
    }

    //
    // blockwise NEW
    //

/*
    if BLOCKWISE_STATES.get().is_none() {
        BLOCKWISE_STATES
            .try_init_once(|| ArrayQueue::new(BLOCKWISE_STATES_MAX))
            .unwrap();
    }
    let stats = BLOCKWISE_STATES.get().unwrap();
*/

    (unsafe { &mut STATES00 })[0] = Some(99); // !!!! KLUDGE placeholder

    if let Some((idx, stat)) = unsafe { &mut STATES00 }.iter_mut().enumerate().find(|x| x.1.is_none()) {
        *stat = Some(99);
        crate::println!("sending, where STATES00: {:?}", unsafe { &STATES00 });
        //panic!("!!");

        let (addr, uri) = addr_uri.unwrap();
        match idx { // !!!! KLUDGE hardcoded
            0 => todo!(),
            1 => {
                let bs = BlockwiseStream::get(&BLOCKWISE_QUEUE, &BLOCKWISE_WAKER);
                XbdStream::add(&BLOCKWISE_QUEUE, &BLOCKWISE_WAKER,
                               Some(ReqInner::new(COAP_METHOD_GET, addr, uri, None, true, Some(idx))));

                Some(bs)
            },
            2 => {
                let bs = BlockwiseStream::get(&BLOCKWISE_2_QUEUE, &BLOCKWISE_2_WAKER);
                XbdStream::add(&BLOCKWISE_2_QUEUE, &BLOCKWISE_2_WAKER,
                               Some(ReqInner::new(COAP_METHOD_GET, addr, uri, None, true, Some(idx))));

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