use mcu_if::c_types::c_void;
use mcu_if::utils::{u8_slice_from, u8_slice_mut_from};

use core::{str::from_utf8, pin::Pin, task::{Context, Poll}};
use futures_util::{task::AtomicWaker, Stream};
use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;

use super::stream::XbdStream;
use super::gcoap::{ReqInner, COAP_METHOD_GET};

#[no_mangle]
pub extern fn xbd_blockwise_addr_ptr(idx: usize) -> *const c_void {
    BlockwiseData::state(&idx).unwrap().addr.as_ptr() as _
}

#[no_mangle]
pub extern fn xbd_blockwise_addr_update(addr: *const c_void, addr_len: usize, idx: usize) {
    let buf = &mut BlockwiseData::state_mut(&idx).unwrap().addr;
    blockwise_metadata_update(
        u8_slice_from(addr as *const u8, addr_len), buf, buf.len());
}

#[no_mangle]
pub extern fn xbd_blockwise_uri_ptr(idx: usize) -> *const c_void {
    BlockwiseData::state(&idx).unwrap().uri.as_ptr() as _
}

#[no_mangle]
pub extern fn xbd_blockwise_uri_update(uri: *const c_void, uri_len: usize, idx: usize) {
    let buf = &mut BlockwiseData::state_mut(&idx).unwrap().uri;
    blockwise_metadata_update(
        u8_slice_from(uri as *const u8, uri_len), buf, buf.len());
}

#[no_mangle]
pub extern fn xbd_blockwise_hdr_update(hdr: *const c_void, hdr_len: usize, idx: usize) {
    let BlockwiseState { hdr: buf, hdr_len: buf_len, .. } =
        BlockwiseData::state_mut(&idx).unwrap();

    *buf_len = blockwise_metadata_update(
        u8_slice_from(hdr as *const u8, hdr_len), buf, buf.len());
}

#[no_mangle]
pub extern fn xbd_blockwise_hdr_copy(buf: *mut u8, buf_sz: usize, idx: usize) -> usize {
    let BlockwiseState { hdr, hdr_len, .. } =
        BlockwiseData::state(&idx).unwrap();
    let len = *hdr_len;

    if len > 0 {
        u8_slice_mut_from(buf, buf_sz)[..len]
            .copy_from_slice(&hdr[..len]);
    }

    len
}

#[no_mangle]
pub extern fn xbd_blockwise_async_gcoap_req(
    last_addr: *const c_void, last_addr_len: usize,
    last_uri: *const c_void, last_uri_len: usize,
    idx: usize)
{
    let addr = from_utf8(u8_slice_from(last_addr as *const u8, last_addr_len)).unwrap();
    let uri = from_utf8(u8_slice_from(last_uri as *const u8, last_uri_len)).unwrap();
    add_blockwise_req_generic(Some((addr, uri)), Some(idx));
}

#[no_mangle]
pub extern fn xbd_blockwise_async_gcoap_complete(idx: usize) {
    add_blockwise_req_generic(None, Some(idx));
}

//

const LAST_BLOCKWISE_ADDR_MAX: usize = 64;
const LAST_BLOCKWISE_URI_MAX: usize = 64;
const LAST_BLOCKWISE_HDR_MAX: usize = 64;

type GridAddr = [[u8; LAST_BLOCKWISE_ADDR_MAX]; BLOCKWISE_STATES_MAX];
static mut GRID_ADDR: &'static mut GridAddr = &mut [[0; LAST_BLOCKWISE_ADDR_MAX]; BLOCKWISE_STATES_MAX];

type GridUri = [[u8; LAST_BLOCKWISE_URI_MAX]; BLOCKWISE_STATES_MAX];
static mut GRID_URI: &'static mut GridUri = &mut [[0; LAST_BLOCKWISE_URI_MAX]; BLOCKWISE_STATES_MAX];

type GridHdr = [[u8; LAST_BLOCKWISE_HDR_MAX]; BLOCKWISE_STATES_MAX];
static mut GRID_HDR: &'static mut GridHdr = &mut [[0; LAST_BLOCKWISE_HDR_MAX]; BLOCKWISE_STATES_MAX];

static mut ARRAY_HDR_LEN: &'static mut [usize] = &mut [0; BLOCKWISE_STATES_MAX];

fn blockwise_metadata_update(data_in: &[u8], data: &mut [u8], data_max: usize) -> usize {
    let data_len = data_in.len();
    assert!(data_len < data_max);

    data.fill(0u8);
    data[..data_len].copy_from_slice(data_in);

    data_len
}

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
const ARRAY_REPEAT_VALUE: Option<BlockwiseState> = None;
static mut BLOCKWISE_STATES: &'static mut [Option<BlockwiseState>] = &mut [ARRAY_REPEAT_VALUE; BLOCKWISE_STATES_MAX];

struct BlockwiseData();
impl BlockwiseData {
    fn states() -> &'static mut [Option<BlockwiseState>] {
        unsafe { BLOCKWISE_STATES }
    }

    fn state(idx: &usize) -> Option<&BlockwiseState> {
        Self::states()[*idx].as_ref()
    }

    fn state_mut(idx: &usize) -> Option<&mut BlockwiseState> {
        Self::states()[*idx].as_mut()
    }
}

//

#[derive(Debug)]
struct BlockwiseState {
    queue: &'static OnceCell<ArrayQueue<Option<ReqInner>>>,
    waker: &'static AtomicWaker,
    idx: usize,
    addr: &'static mut [u8],
    uri: &'static mut [u8],
    hdr: &'static mut [u8],
    hdr_len: usize,
}

impl Clone for BlockwiseState {
    fn clone(&self) -> BlockwiseState {
        Self {
            queue: self.queue,
            waker: self.waker,
            idx: self.idx,
            addr: unsafe { &mut GRID_ADDR[self.idx] },
            uri: unsafe { &mut GRID_URI[self.idx] },
            hdr: unsafe { &mut GRID_HDR[self.idx] },
            hdr_len: self.hdr_len,
        }
    }
}

impl BlockwiseState {
    fn get(idx: usize) -> Self {
        let (queue, waker) = match idx { // KLUDGE !! to be rectified
            0 => (&BLOCKWISE_QUEUE, &BLOCKWISE_WAKER),
            1 => (&BLOCKWISE_1_QUEUE, &BLOCKWISE_1_WAKER),
            2 => (&BLOCKWISE_2_QUEUE, &BLOCKWISE_2_WAKER),
            _ => unreachable!(),
        };

        Self { queue, waker, idx,
            addr: unsafe { &mut GRID_ADDR[idx] },
            uri: unsafe { &mut GRID_URI[idx] },
            hdr: unsafe { &mut GRID_HDR[idx] },
            hdr_len: unsafe { ARRAY_HDR_LEN[idx] },
        }
    }

    fn get_stream(&self) -> BlockwiseStream {
        BlockwiseStream::get(self.queue, self.waker)
    }

    fn add_to_stream(&self, req: Option<ReqInner>) {
        XbdStream::add(self.queue, self.waker, req);
    }
}

pub fn add_blockwise_req_generic(
    addr_uri: Option<(&str, &str)>, blockwise_state_index: Option<usize>) -> Option<BlockwiseStream> {

    if let Some(idx) = blockwise_state_index {
        let stat = BlockwiseData::state(&idx).unwrap();

        if let Some((addr, uri)) = addr_uri { // <blockwise NEXT>
            stat.add_to_stream(Some(
                ReqInner::new(COAP_METHOD_GET, addr, uri, None, true, Some(idx))));
        } else { // <blockwise COMPLETE>
            stat.add_to_stream(None);
        }

        return None;
    }

    //
    // <blockwise NEW>
    //

    let states = BlockwiseData::states();
    if let Some((idx, slot)) = states.iter_mut().enumerate().find(|x| x.1.is_none()) {
        let stat = BlockwiseState::get(idx);
        *slot = Some(stat.clone());

        let bs = stat.get_stream(); // makes sure stream is initialized before `.add_to_stream()`

        crate::println!("sending <blockwise NEW>, via idx={}/{}, where states: {:?}",
                        idx, states.len(), states);
        let (addr, uri) = addr_uri.unwrap();
        let req = ReqInner::new(COAP_METHOD_GET, addr, uri, None, true, Some(idx));
        stat.add_to_stream(Some(req));

        Some(bs)
    } else { // no available BlockwiseState
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