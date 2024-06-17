use core::{future::Future, pin::Pin, task::{Context, Poll, Waker}};
use futures_util::task::AtomicWaker;
use super::{BlockwiseData, BLOCKWISE_HDR_MAX};

pub const REQ_ADDR_MAX: usize = 64;
pub const REQ_URI_MAX: usize = 64;

const PAYLOAD_REQ_MAX: usize = 48;
const PAYLOAD_OUT_MAX: usize = 128;

type PayloadReq = heapless::Vec<u8, PAYLOAD_REQ_MAX>;
pub type PayloadOut = heapless::Vec<u8, PAYLOAD_OUT_MAX>;

//
// gcoap client
//

pub struct _GcoapPing {
    // ...
    _waker: Option<AtomicWaker>,
}

// const GCOAP_MEMO_UNUSED: u8 =      0x00;
// const GCOAP_MEMO_RETRANSMIT: u8 =  0x01;
// const GCOAP_MEMO_WAIT: u8 =        0x02;
const GCOAP_MEMO_RESP: u8 =        0x03;
const GCOAP_MEMO_TIMEOUT: u8 =     0x04;
const GCOAP_MEMO_ERR: u8 =         0x05;
const GCOAP_MEMO_RESP_TRUNC: u8 =  0x06;

#[derive(Debug, PartialEq)]
pub enum GcoapMemoState {
    Resp(Option<PayloadOut>),
    Timeout,
    Err,
    RespTrunc(Option<PayloadOut>),
}

impl GcoapMemoState {
    pub fn new(memo_state: u8, payload: Option<PayloadOut>) -> Self {
        match memo_state {
            // ...
            GCOAP_MEMO_RESP => Self::Resp(payload),
            GCOAP_MEMO_TIMEOUT => Self::Timeout,
            GCOAP_MEMO_ERR => Self::Err,
            GCOAP_MEMO_RESP_TRUNC => Self::RespTrunc(payload),
            _ => unreachable!(),
        }
    }
}

//

/* RIOT/sys/include/net/coap.h
#define COAP_CLASS_REQ          (0)
#define COAP_METHOD_GET         (1)
#define COAP_METHOD_POST        (2)
#define COAP_METHOD_PUT         (3)
#define COAP_METHOD_DELETE      (4)
#define COAP_METHOD_FETCH       (5)
#define COAP_METHOD_PATCH       (6)
#define COAP_METHOD_IPATCH      (7)
*/

pub type CoapMethod = u8;
pub const COAP_METHOD_GET      : CoapMethod = 0x01;
pub const COAP_METHOD_POST     : CoapMethod = 0x02;
pub const COAP_METHOD_PUT      : CoapMethod = 0x03;
// pub const COAP_METHOD_DELETE   : CoapMethod = 0x04;
// pub const COAP_METHOD_FETCH    : CoapMethod = 0x05;
// pub const COAP_METHOD_PATCH    : CoapMethod = 0x06;
// pub const COAP_METHOD_IPATCH   : CoapMethod = 0x07;

//

#[repr(u8)]
#[derive(Debug)]
pub enum Req {
    Get(ReqInner) = COAP_METHOD_GET,
    Post(ReqInner) = COAP_METHOD_POST,
    Put(ReqInner) = COAP_METHOD_PUT,
}

impl Req {
    pub fn new(method: CoapMethod, addr: &str, uri: &str,
               payload: Option<PayloadReq>) -> Self {
        let inner = ReqInner::new(method, addr, uri, payload, false, None, None);

        match method {
            COAP_METHOD_GET => Self::Get(inner),
            COAP_METHOD_POST => Self::Post(inner),
            COAP_METHOD_PUT => Self::Put(inner),
            _ => todo!(),
        }
    }
}

impl Future for Req {
    type Output = GcoapMemoState;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<<Self as Future>::Output> {
        // https://internals.rust-lang.org/t/idea-enhance-match-ergonomics-to-match-on-pinned-enums-without-unsafe/9317
        unsafe {
            match Pin::get_unchecked_mut(self) {
                Req::Get(req) | Req::Post(req) | Req::Put(req) => Pin::new_unchecked(req).poll(cx),
            }
        }
    }
}

//

#[derive(Debug)]// !!!! V1
pub struct Progress(Option<AtomicWaker>, pub Option<AtomicWaker>, pub Option<GcoapMemoState>);

impl Progress {
    pub fn new() -> Self {
        Self(Some(AtomicWaker::new()), None, None)
    }

    pub fn is_new(&self) -> bool {
        self.0.is_some() && self.1.is_none() && self.2.is_none()
    }

    pub fn register(&mut self, cx_waker: &Waker) {
        assert!(self.is_new());

        let waker = self.0.take().unwrap();
        waker.register(cx_waker);
        self.1.replace(waker);
    }

    pub fn finish(&mut self, memo: GcoapMemoState) {
        assert!(self.0.is_none() && self.1.is_some() && self.2.is_none()); // registered

        self.2.replace(memo);
        self.1.take().unwrap().wake();
    }

    pub fn as_mut_ptr(&self) -> *mut Self {
        self as *const _ as *mut _
    }

    pub fn get_ref_mut(ptr: *mut Self) -> &'static mut Self {
        unsafe { &mut *ptr }
    }
}

#[derive(Debug)]
pub struct ReqInner {
    method: CoapMethod,
    addr: heapless::String<{ REQ_ADDR_MAX }>,
    uri: heapless::String<{ REQ_URI_MAX }>,
    payload: Option<PayloadReq>,
    blockwise: bool,
    blockwise_state_index: Option<usize>,
    blockwise_hdr: Option<heapless::Vec<u8, BLOCKWISE_HDR_MAX>>,
    progress: Progress,
}

impl ReqInner {
    pub fn new(method: CoapMethod, addr: &str, uri: &str,
               payload: Option<PayloadReq>,
               blockwise: bool,
               blockwise_state_index: Option<usize>,
               blockwise_hdr: Option<heapless::Vec<u8, BLOCKWISE_HDR_MAX>>) -> Self {
        ReqInner {
            method,
            addr: heapless::String::try_from(addr).unwrap(),
            uri: heapless::String::try_from(uri).unwrap(),
            payload,
            blockwise,
            blockwise_state_index,
            blockwise_hdr,
            progress: Progress::new(),
        }
    }
}

fn gcoap_get(addr: &str, uri: &str, progress_ptr: *mut Progress) {
    super::Xbd::gcoap_req_v2(addr, uri, COAP_METHOD_GET, None, false,
                             None, progress_ptr);
}

fn gcoap_get_blockwise(addr: &str, uri: &str, blockwise_state_index: usize, progress_ptr: *mut Progress) {
    super::Xbd::gcoap_req_v2(addr, uri, COAP_METHOD_GET, None, true,
                             Some(blockwise_state_index), progress_ptr);
}

fn gcoap_post<F>(addr: &str, uri: &str, payload: &[u8], cb: F) where F: FnOnce(GcoapMemoState) + 'static {
    //Self::gcoap_req(addr, uri, COAP_METHOD_POST, Some(payload), false, None, cb);
    // 11TODO -> v2; plus test via [shell.rs] `test_heapless_req().await` now at ????
//    super::Xbd::gcoap_req_v2(addr, uri, xxxx);
}

fn gcoap_put<F>(addr: &str, uri: &str, payload: &[u8], cb: F) where F: FnOnce(GcoapMemoState) + 'static {
    //Self::gcoap_req(addr, uri, COAP_METHOD_PUT, Some(payload), false, None, cb);
    // 11TODO -> v2; plus test via [shell.rs] `test_heapless_req().await` now at ????
//    super::Xbd::gcoap_req_v2(addr, uri, xxxx);
}

impl Future for ReqInner {
    type Output = GcoapMemoState;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<<Self as Future>::Output> {
        if self.progress.is_new() {
            self.progress.register(cx.waker());

            let cb = |_| panic!("BUILD SHIM"); // !!!! !!!!
            match self.method {
                COAP_METHOD_GET => {
                    if self.blockwise {
                        let idx = self.blockwise_state_index.unwrap();

                        if BlockwiseData::state_is_valid(idx) {
                            BlockwiseData::set_state_last(Some(idx));
                            BlockwiseData::update_state(idx,
                                self.addr.as_bytes(),
                                self.uri.as_bytes(),
                                self.blockwise_hdr.as_deref());

                            gcoap_get_blockwise(&self.addr, &self.uri, idx,
                                                self.progress.as_mut_ptr());
                        } else { // blockwise stream could be already closed
                            BlockwiseData::set_state_last(None);

                            return Poll::Ready(GcoapMemoState::Err)
                        }
                    } else {
                        gcoap_get(&self.addr, &self.uri, self.progress.as_mut_ptr());
                    }
                },
                COAP_METHOD_POST => gcoap_post(// 11TODO -> v2
                    &self.addr, &self.uri, self.payload.as_ref().unwrap().as_slice(), cb),
                COAP_METHOD_PUT => gcoap_put(// 11TODO -> v2
                    &self.addr, &self.uri, self.payload.as_ref().unwrap().as_slice(), cb),
                _ => todo!(),
            }

            Poll::Pending
        } else {
            let out = self.progress.2.take().unwrap();
            Poll::Ready(out)
        }
    }
}

unsafe impl Send for ReqInner {
    // !!!! !!!!
}