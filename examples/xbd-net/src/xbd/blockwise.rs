use mcu_if::c_types::c_void;
use mcu_if::utils::{u8_slice_from, u8_slice_mut_from};

use core::{str::from_utf8, pin::Pin, task::{Context, Poll}};
use futures_util::{task::AtomicWaker, Stream};
use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;

use super::stream::XbdStream;
use super::gcoap::{ReqInner, COAP_METHOD_GET};

#[no_mangle]
#[allow(static_mut_refs)]
pub extern fn xbd_blockwise_state_index() -> usize {
    (unsafe { &BLOCKWISE_STATE_INDEX }).unwrap()
}

#[no_mangle]
#[allow(static_mut_refs)]
pub extern fn xbd_blockwise_state_index_update(idx: usize) {
    *(unsafe { &mut BLOCKWISE_STATE_INDEX }) = Some(idx);
}

#[no_mangle]
pub extern fn xbd_blockwise_addr_ptr(idx: usize) -> *const c_void {
    BlockwiseData::state(&idx).unwrap().addr.as_ptr() as _
}

#[no_mangle]
pub extern fn xbd_blockwise_addr_update(addr: *const c_void, addr_len: usize, idx: usize) {
    let buf = &mut BlockwiseData::state_mut(&idx).unwrap().addr;
    BlockwiseState::update_metadata(
        u8_slice_from(addr as *const u8, addr_len), buf, buf.len());
}

#[no_mangle]
pub extern fn xbd_blockwise_uri_ptr(idx: usize) -> *const c_void {
    BlockwiseData::state(&idx).unwrap().uri.as_ptr() as _
}

#[no_mangle]
pub extern fn xbd_blockwise_uri_update(uri: *const c_void, uri_len: usize, idx: usize) {
    let buf = &mut BlockwiseData::state_mut(&idx).unwrap().uri;
    BlockwiseState::update_metadata(
        u8_slice_from(uri as *const u8, uri_len), buf, buf.len());
}

#[no_mangle]
pub extern fn xbd_blockwise_hdr_update(hdr: *const c_void, hdr_len: usize, idx: usize) {
    let BlockwiseState { hdr: buf, hdr_len: buf_len, .. } =
        BlockwiseData::state_mut(&idx).unwrap();

    *buf_len = BlockwiseState::update_metadata(
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
    addr: *const c_void, addr_len: usize,
    uri: *const c_void, uri_len: usize,
    idx: usize)
{
    let addr = from_utf8(u8_slice_from(addr as *const u8, addr_len)).unwrap();
    let uri = from_utf8(u8_slice_from(uri as *const u8, uri_len)).unwrap();
    BlockwiseData::send_blockwise_req(Some(idx), Some((addr, uri)));
}

#[no_mangle]
pub extern fn xbd_blockwise_async_gcoap_complete(idx: usize) {
    BlockwiseData::send_blockwise_req(Some(idx), None);
}

//

const BLOCKWISE_ADDR_MAX: usize = 64;
const BLOCKWISE_URI_MAX: usize = 64;
const BLOCKWISE_HDR_MAX: usize = 64;
const BLOCKWISE_STATES_MAX: usize = 4;

static mut BLOCKWISE_STATE_INDEX: Option<usize> = None;

type GridAddr = [[u8; BLOCKWISE_ADDR_MAX]; BLOCKWISE_STATES_MAX];
static mut GRID_ADDR: &'static mut GridAddr = &mut [[0; BLOCKWISE_ADDR_MAX]; BLOCKWISE_STATES_MAX];

type GridUri = [[u8; BLOCKWISE_URI_MAX]; BLOCKWISE_STATES_MAX];
static mut GRID_URI: &'static mut GridUri = &mut [[0; BLOCKWISE_URI_MAX]; BLOCKWISE_STATES_MAX];

type GridHdr = [(usize, [u8; BLOCKWISE_HDR_MAX]); BLOCKWISE_STATES_MAX];
static mut GRID_HDR: &'static mut GridHdr = &mut [(0, [0; BLOCKWISE_HDR_MAX]); BLOCKWISE_STATES_MAX];

type QW = (OnceCell<ArrayQueue<Option<ReqInner>>>, AtomicWaker);
const ARRAY_REPEAT_VALUE_QW: QW = (OnceCell::uninit(), AtomicWaker::new());
static mut BLOCKWISE_QW: &'static mut [QW; BLOCKWISE_STATES_MAX] = &mut [ARRAY_REPEAT_VALUE_QW; BLOCKWISE_STATES_MAX];

const ARRAY_REPEAT_VALUE_BS: Option<BlockwiseState> = None;
static mut BLOCKWISE_STATES: &'static mut [Option<BlockwiseState>] = &mut [ARRAY_REPEAT_VALUE_BS; BLOCKWISE_STATES_MAX];

//

pub struct BlockwiseData();

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

    fn find_state_available() -> Option<(usize, &'static mut Option<BlockwiseState>)> {
        Self::states().iter_mut().enumerate().find(|x| x.1.is_none())
    }

    pub fn send_blockwise_req(idx: Option<usize>, addr_uri: Option<(&str, &str)>) -> Option<BlockwiseStream> {
        if let Some(idx) = idx {
            let stat = Self::state(&idx).unwrap();

            if let Some((addr, uri)) = addr_uri { // <blockwise NEXT>
                stat.add_to_stream(Some(
                    ReqInner::new(COAP_METHOD_GET, addr, uri, None, true, Some(idx))));
            } else { // <blockwise COMPLETE>
                stat.add_to_stream(None);
            }

            return None;
        }

        // <blockwise NEW>

        if let Some((idx, slot)) = Self::find_state_available() {
            let state = BlockwiseState::get(idx);
            *slot = Some(state.clone());

            let bs = state.get_stream(); // makes sure stream is initialized before `.add_to_stream()`

            let states = Self::states();
            crate::println!("sending <blockwise NEW>, via idx={}/{}, where states: {:?}",
                            idx, states.len(), states);

            let (addr, uri) = addr_uri.unwrap();
            let req = ReqInner::new(COAP_METHOD_GET, addr, uri, None, true, Some(idx));
            state.add_to_stream(Some(req));

            Some(bs)
        } else { // no available BlockwiseState
            None
        }
    }
}

//

#[derive(Debug)]
struct BlockwiseState {
    idx: usize,
    queue: &'static OnceCell<ArrayQueue<Option<ReqInner>>>,
    waker: &'static AtomicWaker,
    addr: &'static mut [u8],
    uri: &'static mut [u8],
    hdr: &'static mut [u8],
    hdr_len: usize,
}

impl Clone for BlockwiseState {
    fn clone(&self) -> BlockwiseState {
        Self {
            idx: self.idx,
            queue: self.queue,
            waker: self.waker,
            addr: unsafe { &mut GRID_ADDR[self.idx] },
            uri: unsafe { &mut GRID_URI[self.idx] },
            hdr: unsafe { &mut GRID_HDR[self.idx].1 },
            hdr_len: unsafe { GRID_HDR[self.idx].0 },
        }
    }
}

impl BlockwiseState {
    fn get(idx: usize) -> Self {
        let (queue, waker) = unsafe { &BLOCKWISE_QW[idx] };

        Self { queue, waker, idx,
            addr: unsafe { &mut GRID_ADDR[idx] },
            uri: unsafe { &mut GRID_URI[idx] },
            hdr: unsafe { &mut GRID_HDR[idx].1 },
            hdr_len: unsafe { GRID_HDR[idx].0 },
        }
    }

    fn get_stream(&self) -> BlockwiseStream {
        BlockwiseStream::get(self.queue, self.waker)
    }

    fn add_to_stream(&self, req: Option<ReqInner>) {
        XbdStream::add(self.queue, self.waker, req);
    }

    fn update_metadata(data_in: &[u8], data: &mut [u8], data_max: usize) -> usize {
        let data_len = data_in.len();
        assert!(data_len < data_max);

        data.fill(0u8);
        data[..data_len].copy_from_slice(data_in);

        data_len
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