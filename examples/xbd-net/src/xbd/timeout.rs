use core::{future::Future, pin::Pin, task::{Context, Poll}};
use futures_util::task::AtomicWaker;
use mcu_if::alloc::boxed::Box;

pub struct Timeout {
    msec: u32,
    cb: Option<Box<dyn FnOnce() + 'static>>,
    _waker: Option<AtomicWaker>,
}

impl Timeout {
    pub fn new(msec: u32, cb: Option<Box<dyn FnOnce() + 'static>>) -> Self {
        Timeout { msec, cb,
            _waker: Some(AtomicWaker::new()),
        }
    }
}

impl Future for Timeout {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<<Self as Future>::Output> {
        if let Some(_waker) = self._waker.take() {
            _waker.register(cx.waker());

            super::Xbd::set_timeout(self.msec, move |_| {
                _waker.wake();
            });

            Poll::Pending
        } else {
            self.cb.take().map(|cb| cb());

            Poll::Ready(())
        }
    }
}