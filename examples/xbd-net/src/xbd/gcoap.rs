use core::{future::Future, pin::Pin, task::{Context, Poll}, cell::RefCell};
use futures_util::task::AtomicWaker;
use mcu_if::{alloc::{vec::Vec, rc::Rc}}; // !!!!
use super::{BlockwiseData, BLOCKWISE_HDR_MAX};

pub const REQ_ADDR_MAX: usize = 64;
pub const REQ_URI_MAX: usize = 64;
const REQ_PAYLOAD_MAX: usize = 48;

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
    Resp(Option<Vec<u8>>),
    Timeout,
    Err,
    RespTrunc(Option<Vec<u8>>),
}

impl GcoapMemoState {
    pub fn new(memo_state: u8, payload: Option<Vec<u8>>) -> Self {
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
               payload: Option<heapless::Vec<u8, REQ_PAYLOAD_MAX>>) -> Self {
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
pub struct ReqInner {
    method: CoapMethod,
    addr: heapless::String<{ REQ_ADDR_MAX }>,
    uri: heapless::String<{ REQ_URI_MAX }>,
    payload: Option<heapless::Vec<u8, REQ_PAYLOAD_MAX>>,
    blockwise: bool,
    blockwise_state_index: Option<usize>,
    blockwise_hdr: Option<heapless::Vec<u8, BLOCKWISE_HDR_MAX>>,
    out: Rc<RefCell<Option<GcoapMemoState>>>,
    _waker: Option<AtomicWaker>,
}

impl ReqInner {
    pub fn new(method: CoapMethod, addr: &str, uri: &str,
               payload: Option<heapless::Vec<u8, REQ_PAYLOAD_MAX>>,
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
            out: Rc::new(RefCell::new(None)),
            _waker: Some(AtomicWaker::new()),
        }
    }
}

impl Future for ReqInner {
    type Output = GcoapMemoState;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<<Self as Future>::Output> {
        if let Some(_waker) = self._waker.take() {
            _waker.register(&cx.waker());

            let outc = self.out.clone();
            let cb = move |out| {
                outc.borrow_mut().replace(out);
                _waker.wake();
            };
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

                            super::Xbd::gcoap_get_blockwise(&self.addr, &self.uri, idx, cb);
                        } else { // blockwise stream could be already closed
                            BlockwiseData::set_state_last(None);

                            return Poll::Ready(GcoapMemoState::Err)
                        }
                    } else {
                        super::Xbd::gcoap_get(&self.addr, &self.uri, cb);
                    }
                },
                COAP_METHOD_POST => super::Xbd::gcoap_post(
                    &self.addr, &self.uri, self.payload.as_ref().unwrap().as_slice(), cb),
                COAP_METHOD_PUT => super::Xbd::gcoap_put(
                    &self.addr, &self.uri, self.payload.as_ref().unwrap().as_slice(), cb),
                _ => todo!(),
            }

            Poll::Pending
        } else {
            Poll::Ready(self.out.take().unwrap())
        }
    }
}

unsafe impl Send for ReqInner {
    // !!!! !!!!
}