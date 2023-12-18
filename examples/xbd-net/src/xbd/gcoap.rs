use core::{future::Future, pin::Pin, task::{Context, Poll}, cell::RefCell};
use futures_util::task::AtomicWaker;
use mcu_if::{alloc::{vec::Vec, string::{String, ToString}, rc::Rc}};

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
// const GCOAP_MEMO_ERR: u8 =         0x05;
// const GCOAP_MEMO_RESP_TRUNC: u8 =  0x06;

#[derive(Debug)]
pub enum GcoapMemoState {
    Resp(Option<Vec<u8>>),
    Timeout,
}

impl GcoapMemoState {
    pub fn new(memo_state: u8, payload: Option<Vec<u8>>) -> Self {
        match memo_state {
            // ...
            GCOAP_MEMO_RESP => Self::Resp(payload),
            GCOAP_MEMO_TIMEOUT => Self::Timeout,
            // ...
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
    pub fn new(method: CoapMethod, addr: &str, uri: &str, payload: Option<Vec<u8>>) -> Self {
        let inner = ReqInner::new(method, addr, uri, payload);

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
    addr: String,
    uri: String,
    payload: Option<Vec<u8>>,
    out: Rc<RefCell<Option<GcoapMemoState>>>,
    _waker: Option<AtomicWaker>,
}

impl ReqInner {
    pub fn new(method: CoapMethod, addr: &str, uri: &str, payload: Option<Vec<u8>>) -> Self {
        ReqInner {
            method,
            addr: addr.to_string(),
            uri: uri.to_string(),
            payload,
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
                COAP_METHOD_GET => super::Xbd::gcoap_get(
                    &self.addr, &self.uri, cb),
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