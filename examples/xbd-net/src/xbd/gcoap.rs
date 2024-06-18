use core::{future::Future, pin::Pin, task::{Context, Poll, Waker}};
use core::ffi::c_void;
use futures_util::task::AtomicWaker;
use super::{BlockwiseData, BLOCKWISE_HDR_MAX};
use mcu_if::utils::u8_slice_from;

pub const REQ_ADDR_MAX: usize = 64;
pub const REQ_URI_MAX: usize = 64;

const PAYLOAD_REQ_MAX: usize = 48;
const PAYLOAD_OUT_MAX: usize = 128;

type PayloadReq = heapless::Vec<u8, PAYLOAD_REQ_MAX>;
type PayloadOut = heapless::Vec<u8, PAYLOAD_OUT_MAX>;

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

type CoapMethod = u8;
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

#[derive(Debug)]
pub struct Progress<T>(Option<AtomicWaker>, pub Option<AtomicWaker>, pub Option<T>);

#[derive(Debug)]
pub enum ProgressV2<T> {
    New,
    Registered,
    Resolved(T),
}

impl<T> Progress<T> {
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

    pub fn resolve(&mut self, ret: T) {
        assert!(self.0.is_none() && self.1.is_some() && self.2.is_none()); // registered

        self.2.replace(ret);
        self.1.take().unwrap().wake();
    }

    pub fn take(&mut self) -> T {
        assert!(self.0.is_none() && self.1.is_none() && self.2.is_some()); // resolved

        self.2.take().unwrap()
    }

    pub fn as_mut_ptr(&self) -> *mut Self {
        self as *const _ as *mut _
    }

    pub fn get_mut_ref(ptr: *mut Self) -> &'static mut Self {
        unsafe { &mut *ptr }
    }
}

//

#[derive(Debug)]
pub struct ReqInner {
    method: CoapMethod,
    addr: heapless::String<{ REQ_ADDR_MAX }>,
    uri: heapless::String<{ REQ_URI_MAX }>,
    payload: Option<PayloadReq>,
    blockwise: bool,
    blockwise_state_index: Option<usize>,
    blockwise_hdr: Option<heapless::Vec<u8, BLOCKWISE_HDR_MAX>>,
    progress: Progress<GcoapMemoState>,
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

impl Future for ReqInner {
    type Output = GcoapMemoState;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<<Self as Future>::Output> {
        if self.progress.is_new() {
            self.progress.register(cx.waker());
            let progress_ptr = self.progress.as_mut_ptr();

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

                            gcoap_get_blockwise(&self.addr, &self.uri, idx, progress_ptr);
                        } else { // blockwise stream could be already closed
                            BlockwiseData::set_state_last(None);

                            return Poll::Ready(GcoapMemoState::Err)
                        }
                    } else {
                        gcoap_get(&self.addr, &self.uri, progress_ptr);
                    }
                },
                COAP_METHOD_POST => gcoap_post(
                    &self.addr, &self.uri, self.payload.as_ref().unwrap().as_slice(), progress_ptr),
                COAP_METHOD_PUT => gcoap_put(
                    &self.addr, &self.uri, self.payload.as_ref().unwrap().as_slice(), progress_ptr),
                _ => todo!(),
            }

            Poll::Pending
        } else {
            Poll::Ready(self.progress.take())
        }
    }
}

unsafe impl Send for ReqInner {
    // !!!! !!!!
}

//

fn gcoap_get(addr: &str, uri: &str, progress_ptr: *mut Progress<GcoapMemoState>) {
    gcoap_req(addr, uri, COAP_METHOD_GET, None, false, None, progress_ptr);
}

fn gcoap_get_blockwise(addr: &str, uri: &str, blockwise_state_index: usize, progress_ptr: *mut Progress<GcoapMemoState>) {
    gcoap_req(addr, uri, COAP_METHOD_GET, None, true, Some(blockwise_state_index), progress_ptr);
}

fn gcoap_post(addr: &str, uri: &str, payload: &[u8], progress_ptr: *mut Progress<GcoapMemoState>) {
    gcoap_req(addr, uri, COAP_METHOD_POST, Some(payload), false, None, progress_ptr);
}

fn gcoap_put(addr: &str, uri: &str, payload: &[u8], progress_ptr: *mut Progress<GcoapMemoState>) {
    gcoap_req(addr, uri, COAP_METHOD_PUT, Some(payload), false, None, progress_ptr);
}

fn gcoap_req(addr: &str, uri: &str, method: CoapMethod,
    payload: Option<&[u8]>, blockwise: bool, blockwise_state_index: Option<usize>,
    progress_ptr: *mut Progress<GcoapMemoState>) {
    let payload_ptr = payload.map_or(core::ptr::null(), |payload| payload.as_ptr());
    let payload_len = payload.map_or(0, |payload| payload.len());

    let mut addr_cstr = heapless::String::<{ REQ_ADDR_MAX + 1 }>::new();
    addr_cstr.push_str(addr).unwrap();
    addr_cstr.push('\0').unwrap();

    let mut uri_cstr = heapless::String::<{ REQ_URI_MAX + 1 }>::new();
    uri_cstr.push_str(uri).unwrap();
    uri_cstr.push('\0').unwrap();

    type Ty = unsafe extern "C" fn(
        *const u8, *const u8, u8,
        *const u8, usize, bool, usize, *const c_void, *const c_void);

    assert_eq!(blockwise, blockwise_state_index.is_some());
    unsafe {
        (crate::get_xbd_fn!("xbd_gcoap_req_send", Ty))(
            addr_cstr.as_ptr(),
            uri_cstr.as_ptr(),
            method, payload_ptr, payload_len,
            blockwise, blockwise_state_index.unwrap_or(0 /* to be ignored */),
            progress_ptr as *const c_void, // context
            gcoap_req_resp_handler as *const c_void);
    }
}

fn gcoap_req_resp_handler(memo: *const c_void, pdu: *const c_void, remote: *const c_void) {
    extern "C" {
        fn xbd_resp_handler(
            memo: *const c_void, pdu: *const c_void, remote: *const c_void,
            payload: *mut c_void, payload_len: *mut c_void, context: *mut c_void) -> u8;
    }

    let mut context: *const c_void = core::ptr::null_mut();
    let mut payload_ptr: *const u8 = core::ptr::null_mut();
    let mut payload_len: usize = 0;

    let memo_state = unsafe {
        xbd_resp_handler(
            memo, pdu, remote,
            (&mut payload_ptr) as *mut *const u8 as *mut c_void,
            (&mut payload_len) as *mut usize as *mut c_void,
            (&mut context) as *mut *const c_void as *mut c_void) };

    let payload = if payload_len > 0 {
        let hvec: PayloadOut = heapless::Vec::from_slice(
            u8_slice_from(payload_ptr, payload_len)).unwrap();
        Some(hvec)
    } else {
        assert_eq!(payload_ptr, core::ptr::null_mut());
        None
    };

    let memo = GcoapMemoState::new(memo_state, payload);
    Progress::get_mut_ref(context as *mut _).resolve(memo);
}