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

pub struct GcoapGet {
    addr: String,
    uri: String,
    out: Rc<RefCell<Option<GcoapMemoState>>>,
    _waker: Option<AtomicWaker>,
}

impl GcoapGet {
    pub fn new(addr: &str, uri: &str) -> Self {
        GcoapGet {
            addr: addr.to_string(),
            uri: uri.to_string(),
            out: Rc::new(RefCell::new(None)),
            _waker: Some(AtomicWaker::new()),
        }
    }
}

impl Future for GcoapGet {
    type Output = GcoapMemoState;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<<Self as Future>::Output> {
        if let Some(_waker) = self._waker.take() {
            _waker.register(&cx.waker());

            let outc = self.out.clone();
            super::Xbd::gcoap_get(&self.addr, &self.uri, move |out| {
                outc.borrow_mut().replace(out);
                _waker.wake();
            });

            Poll::Pending
        } else {
            Poll::Ready(self.out.take().unwrap())
        }
    }
}