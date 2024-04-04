use mcu_if::c_types::c_void;
use mcu_if::utils::{u8_slice_from, u8_slice_mut_from};

use core::{str::from_utf8, pin::Pin, task::{Context, Poll}};
use futures_util::{task::AtomicWaker, Stream};
use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;

use super::stream::XbdStream;
use super::gcoap::{ReqInner, COAP_METHOD_GET};

#[no_mangle]
pub extern fn xbd_blockwise_state_index() -> usize {
    BlockwiseData::get_state_last()
}

#[no_mangle]
pub extern fn xbd_blockwise_addr_ptr(idx: usize) -> *const c_void {
    BlockwiseData::state(&idx).unwrap().addr.as_ptr() as _
}

#[no_mangle]
pub extern fn xbd_blockwise_uri_ptr(idx: usize) -> *const c_void {
    BlockwiseData::state(&idx).unwrap().uri.as_ptr() as _
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
pub extern fn xbd_blockwise_async_gcoap_next(
    idx: usize,
    addr: *const c_void, addr_len: usize,
    uri: *const c_void, uri_len: usize,
    hdr: *const c_void, hdr_len: usize)
{
    let _ = BlockwiseData::send_blockwise_req(
        Some(idx),
        Some((from_utf8(u8_slice_from(addr as *const u8, addr_len)).unwrap(),
              from_utf8(u8_slice_from(uri as *const u8, uri_len)).unwrap())),
        Some(u8_slice_from(hdr as *const u8, hdr_len)));
}

#[no_mangle]
pub extern fn xbd_blockwise_async_gcoap_complete(idx: usize) {
    let _ = BlockwiseData::send_blockwise_req(Some(idx), None, None);
}

//

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BlockwiseError {
    StateNotAvailable,
}

//

pub const BLOCKWISE_STATES_MAX: usize = 4;
pub const BLOCKWISE_ADDR_MAX: usize = 64;
pub const BLOCKWISE_URI_MAX: usize = 64;
pub const BLOCKWISE_HDR_MAX: usize = 64;

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

pub fn blockwise_states_print() {
    crate::println!("[debug] blockwise_states_print(): states: {:?}", BlockwiseData::states());
}

pub fn blockwise_states_debug() -> heapless::Vec<Option<()>, BLOCKWISE_STATES_MAX> {
    BlockwiseData::states()
        .iter()
        .map(|s| if s.is_some() { Some(()) } else { None })
        .collect::<_>()
}

pub struct BlockwiseData();

impl BlockwiseData {
    fn states() -> &'static mut [Option<BlockwiseState>] {
        unsafe { BLOCKWISE_STATES }
    }

    #[cfg_attr(not(target_arch = "xtensa"), allow(static_mut_refs))]
    fn get_state_last() -> usize {
        (unsafe { &BLOCKWISE_STATE_INDEX }).unwrap()
    }

    #[cfg_attr(not(target_arch = "xtensa"), allow(static_mut_refs))]
    pub fn set_state_last(idx: Option<usize>) {
        *(unsafe { &mut BLOCKWISE_STATE_INDEX }) = idx;
    }

    pub fn update_state(idx: usize, addr: &[u8], uri: &[u8], hdr: Option<&[u8]>) {
        let state = Self::state_mut(&idx).unwrap();

        let buf = &mut state.addr;
        BlockwiseState::update_metadata(addr, buf, buf.len());

        let buf = &mut state.uri;
        BlockwiseState::update_metadata(uri, buf, buf.len());

        if let Some(hdr) = hdr {
            let BlockwiseState { hdr: buf, hdr_len: buf_len, .. } = state;
            *buf_len = BlockwiseState::update_metadata(hdr, buf, buf.len());
        }
    }

    pub fn clear_state(idx: usize) {
        Self::update_state(idx, &[], &[], Some(&[]));
    }

    fn invalidate_state(idx: usize) {
        *(&mut Self::states()[idx]) = None;
    }

    fn state(idx: &usize) -> Option<&BlockwiseState> {
        Self::states()[*idx].as_ref()
    }

    fn state_mut(idx: &usize) -> Option<&mut BlockwiseState> {
        Self::states()[*idx].as_mut()
    }

    pub fn state_is_valid(idx: usize) -> bool {
        Self::state(&idx).is_some()
    }

    fn find_state_available() -> Option<(usize, &'static mut Option<BlockwiseState>)> {
        Self::states().iter_mut().enumerate().find(|x| x.1.is_none())
    }

    pub fn send_blockwise_req(idx: Option<usize>, addr_uri: Option<(&str, &str)>, hdr: Option<&[u8]>) -> Result<BlockwiseStream, BlockwiseError> {
        if let Some(idx) = idx {
            let bs = Self::state(&idx).unwrap().get_stream();

            if let Some((addr, uri)) = addr_uri { // <blockwise NEXT>
                let hdr = heapless::Vec::from_slice(hdr.unwrap()).unwrap();

                bs.add(Some(ReqInner::new(
                    COAP_METHOD_GET, addr, uri, None, true, Some(idx), Some(hdr))));
            } else { // <blockwise COMPLETE>
                BlockwiseData::clear_state(idx);
                BlockwiseData::invalidate_state(idx);

                bs.add(None);
            }

            return Ok(bs);
        }

        // <blockwise NEW>

        if let Some((idx, slot)) = Self::find_state_available() {
            let state = BlockwiseState::get(idx);

            *slot = Some(state.clone());
            crate::println!("debug <blockwise NEW>, via idx={}/{}", idx, BLOCKWISE_STATES_MAX);
            //blockwise_states_print(); // debug

            let (addr, uri) = addr_uri.unwrap();
            let req = ReqInner::new(COAP_METHOD_GET, addr, uri, None, true, Some(idx), None);
            let bs = state.get_stream();
            bs.add(Some(req));

            Ok(bs)
        } else {
            Err(BlockwiseError::StateNotAvailable)
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
        BlockwiseStream::get(self.idx, self.queue, self.waker)
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
pub struct BlockwiseStream {
    idx: usize,
    xs: XbdStream<Option<ReqInner>>,
}

impl BlockwiseStream {
    fn get(idx: usize, queue: &'static OnceCell<ArrayQueue<Option<ReqInner>>>, waker: &'static AtomicWaker) -> Self {
        let xs = XbdStream::get(&queue, &waker)
            .unwrap_or_else(|| XbdStream::new_with_cap(&queue, &waker, 1));

        Self { idx, xs }
    }

    fn add(&self, req: Option<ReqInner>) {
        assert_eq!(self.xs.len(), 0);
        self.xs.add(req);
    }

    fn empty(&self) {
        self.xs.empty();
    }

    pub fn cancel(&self) {
        BlockwiseData::clear_state(self.idx);
        BlockwiseData::invalidate_state(self.idx);

        self.empty();
        self.add(None);
    }
}

impl Stream for BlockwiseStream {
    type Item = ReqInner;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        unsafe {
            match Pin::get_unchecked_mut(self) {
                Self { xs, .. } => {
                    if let Poll::Ready(Some(item)) = Pin::new_unchecked(xs).poll_next(cx) {
                        Poll::Ready(item)
                    } else {
                        Poll::Pending
                    }
                },
            }
        }
    }
}