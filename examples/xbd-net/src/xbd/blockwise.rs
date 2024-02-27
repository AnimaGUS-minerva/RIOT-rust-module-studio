use mcu_if::c_types::c_void;
use mcu_if::utils::{u8_slice_from, u8_slice_mut_from};

use core::{str::from_utf8, pin::Pin, task::{Context, Poll}};
use futures_util::{task::AtomicWaker, Stream};
use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;

use super::stream::XbdStream;
use super::gcoap::{ReqInner, COAP_METHOD_GET};

#[no_mangle]
pub extern fn xbd_blockwise_addr_ptr(blockwise_state_index: usize) -> *const c_void {
    //unsafe { LAST_BLOCKWISE_ADDR.as_ptr() as _ }
    match blockwise_state_index { // !!!! wip
        1 => unsafe { LAST_BLOCKWISE_ADDR.as_ptr() as _ },
        2 => unsafe { LAST_BLOCKWISE_2_ADDR.as_ptr() as _ },
        _ => unreachable!(),
    }
}

#[no_mangle]
pub extern fn xbd_blockwise_uri_ptr(blockwise_state_index: usize) -> *const c_void {
    //unsafe { LAST_BLOCKWISE_URI.as_ptr() as _ }
    match blockwise_state_index { // !!!! wip
        1 => unsafe { LAST_BLOCKWISE_URI.as_ptr() as _ },
        2 => unsafe { LAST_BLOCKWISE_2_URI.as_ptr() as _ },
        _ => unreachable!(),
    }
}

#[no_mangle]
pub extern fn xbd_blockwise_addr_update(addr: *const c_void, addr_len: usize, blockwise_state_index: usize) {
    let addr = u8_slice_from(addr as *const u8, addr_len);
    //unsafe { blockwise_metadata_update(addr, LAST_BLOCKWISE_ADDR, LAST_BLOCKWISE_ADDR_MAX); }
    match blockwise_state_index { // !!!! wip
        1 => unsafe { blockwise_metadata_update(addr, LAST_BLOCKWISE_ADDR, LAST_BLOCKWISE_ADDR_MAX); },
        2 => unsafe { blockwise_metadata_update(addr, LAST_BLOCKWISE_2_ADDR, LAST_BLOCKWISE_2_ADDR_MAX); },
        _ => unreachable!(),
    }
}

#[no_mangle]
pub extern fn xbd_blockwise_uri_update(uri: *const c_void, uri_len: usize, blockwise_state_index: usize) {
    let uri = u8_slice_from(uri as *const u8, uri_len);
    //unsafe { blockwise_metadata_update(uri, LAST_BLOCKWISE_URI, LAST_BLOCKWISE_URI_MAX); }
    match blockwise_state_index { // !!!! wip
        1 => unsafe { blockwise_metadata_update(uri, LAST_BLOCKWISE_URI, LAST_BLOCKWISE_URI_MAX); },
        2 => unsafe { blockwise_metadata_update(uri, LAST_BLOCKWISE_2_URI, LAST_BLOCKWISE_2_URI_MAX); },
        _ => unreachable!(),
    }
}

#[no_mangle]
pub extern fn xbd_blockwise_hdr_copy(buf: *mut u8, buf_sz: usize, blockwise_state_index: usize) -> usize {
    // let len = blockwise_hdr_len();
    // if len > 0 {
    //     blockwise_hdr_copy(u8_slice_mut_from(buf, buf_sz));
    // }
    //
    // len
    //====
    match blockwise_state_index { // !!!! wip
        1 => {
            let len = blockwise_hdr_len();
            if len > 0 {
                blockwise_hdr_copy(u8_slice_mut_from(buf, buf_sz));
            }

            len
        },
        2 => {
            let len = blockwise_2_hdr_len();
            if len > 0 {
                blockwise_2_hdr_copy(u8_slice_mut_from(buf, buf_sz));
            }

            len
        },
        _ => unreachable!(),
    }
}

#[no_mangle]
pub extern fn xbd_blockwise_hdr_update(hdr: *const c_void, hdr_len: usize, blockwise_state_index: usize) {
    //blockwise_hdr_update(u8_slice_from(hdr as *const u8, hdr_len));
    //====
    match blockwise_state_index { // !!!! wip
        1 => { blockwise_hdr_update(u8_slice_from(hdr as *const u8, hdr_len)); },
        2 => { blockwise_2_hdr_update(u8_slice_from(hdr as *const u8, hdr_len)); },
        _ => unreachable!(),
    }
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
pub static BLOCKWISE_1_QUEUE: OnceCell<ArrayQueue<Option<ReqInner>>> = OnceCell::uninit();
pub static BLOCKWISE_1_WAKER: AtomicWaker = AtomicWaker::new();
pub static BLOCKWISE_2_QUEUE: OnceCell<ArrayQueue<Option<ReqInner>>> = OnceCell::uninit();
pub static BLOCKWISE_2_WAKER: AtomicWaker = AtomicWaker::new();
//---- !!!! POC hardcoded $$
const BLOCKWISE_STATES_MAX: usize = 4;

//pub static BLOCKWISE_STATES: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static mut STATES00: &'static mut [Option<BlockwiseState>] = &mut [None; BLOCKWISE_STATES_MAX];

#[derive(Copy, Clone, Debug)]
struct BlockwiseState {
    queue: &'static OnceCell<ArrayQueue<Option<ReqInner>>>,
    waker: &'static AtomicWaker,
    // WIP - fuse metadata stuff
    //
}

impl BlockwiseState {
    fn resolve(idx: usize) -> Self {
        let (queue, waker) = match idx { // KLUDGE !! to be rectified
            0 => (&BLOCKWISE_QUEUE, &BLOCKWISE_WAKER),
            1 => (&BLOCKWISE_1_QUEUE, &BLOCKWISE_1_WAKER),
            2 => (&BLOCKWISE_2_QUEUE, &BLOCKWISE_2_WAKER),
            _ => unreachable!(),
        };

        Self { queue, waker }
    }

    fn get_stream(self) -> BlockwiseStream {
        BlockwiseStream::get(self.queue, self.waker)
    }

    fn add_to_stream(self, req: Option<ReqInner>) {
        XbdStream::add(self.queue, self.waker, req);
    }
}

pub fn add_blockwise_req_generic(
    addr_uri: Option<(&str, &str)>, blockwise_state_index: Option<usize>) -> Option<BlockwiseStream> {

    if let Some(idx) = blockwise_state_index {
        let stat = unsafe { STATES00[idx] }.unwrap();

        if let Some((addr, uri)) = addr_uri { // blockwise NEXT
            let req = ReqInner::new(COAP_METHOD_GET, addr, uri, None, true, Some(idx));
            stat.add_to_stream(Some(req));

        } else { // blockwise COMPLETE
            stat.add_to_stream(None);
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

    { // !!!! KLUDGE placeholder
        (unsafe { &mut STATES00 })[0] = Some(BlockwiseState::resolve(0));
    }

    if let Some((idx, slot)) = unsafe { &mut STATES00 }.iter_mut().enumerate().find(|x| x.1.is_none()) {
        let stat = BlockwiseState::resolve(idx);
        *slot = Some(stat);

        let bs = stat.get_stream(); // makes sure stream is initialized before `.add_to_stream()`

        crate::println!("sending NEW, via idx={}/{}, where STATES00: {:?}",
                        idx, BLOCKWISE_STATES_MAX, unsafe { &STATES00 });
        let (addr, uri) = addr_uri.unwrap();
        let req = ReqInner::new(COAP_METHOD_GET, addr, uri, None, true, Some(idx));
        stat.add_to_stream(Some(req));

        Some(bs)
    } else { // STATES00 is full
        None
    }
}

//

#[derive(Debug)]
pub struct BlockwiseStream(XbdStream<Option<ReqInner>>);

impl BlockwiseStream {
    pub fn get(queue: &'static OnceCell<ArrayQueue<Option<ReqInner>>>, waker: &'static AtomicWaker) -> Self {
        Self(XbdStream::get(&queue, &waker)
            .unwrap_or(XbdStream::new_with_cap(&queue, &waker, 1)))
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