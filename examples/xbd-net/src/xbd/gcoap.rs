use core::{future::Future, pin::Pin, task::{Context, Poll}, cell::RefCell};
use futures_util::task::AtomicWaker;
use mcu_if::{alloc::{vec::Vec, string::{String, ToString}, rc::Rc}};

pub struct _GcoapPing {
    // ...
    _waker: Option<AtomicWaker>,
}

pub struct GcoapGet {
    addr: String,
    uri: String,
    payload: Rc<RefCell<Option<(u8, Vec<u8>)>>>,
    _waker: Option<AtomicWaker>,
}

impl GcoapGet {
    pub fn new(addr: &str, uri: &str) -> Self {
        GcoapGet {
            addr: addr.to_string(),
            uri: uri.to_string(),
            payload: Rc::new(RefCell::new(None)),
            _waker: Some(AtomicWaker::new()),
        }
    }
}

impl Future for GcoapGet {
    type Output = (u8, Vec<u8>);

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<<Self as Future>::Output> {
        if let Some(_waker) = self._waker.take() {
            _waker.register(&cx.waker());

            let plc = self.payload.clone();
            super::Xbd::gcoap_get(&self.addr, &self.uri, move |memo_state, payload| {
                plc.borrow_mut().replace((memo_state, payload));
                _waker.wake();
            });

            Poll::Pending
        } else {
            Poll::Ready(self.payload.take().unwrap())
        }
    }
}