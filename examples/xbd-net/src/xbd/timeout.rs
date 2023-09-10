use core::{future::Future, pin::Pin, task::{Context, Poll}};
use futures_util::task::AtomicWaker;
use mcu_if::{println, alloc::{boxed::Box, vec::Vec}};

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
        println!("@@ Timeout::poll(): ^^");

        if let Some(_waker) = self._waker.take() {
            _waker.register(&cx.waker());

            super::Xbd::set_timeout(self.msec, move |_| {
                println!("@@ !! timeout, calling `_waker.wake()`");
                _waker.wake();
            });

            println!("@@ !! returning `Poll::Pending`");

            Poll::Pending
        } else {
            println!("@@ !! returning `Poll::Ready(())`");
            self.cb.take().map(|cb| cb());

            Poll::Ready(())
        }
    }
}

//

use mcu_if::alloc::rc::Rc; // !!
use core::cell::RefCell; // !!
use mcu_if::alloc::string::{String, ToString}; // !!
pub struct Timeout00 {
//    msec: u32,
//    cb: Option<Box<dyn FnOnce() + 'static>>,
    addr: String, uri: String, // !!
//    payload: Option<Vec<u8>>, // !!
    payload: Rc<RefCell<Option<Vec<u8>>>>, // !!
    _waker: Option<AtomicWaker>,
}

impl Timeout00 {
    pub fn new(addr: &str, uri: &str/* msec: u32, cb: Option<Box<dyn FnOnce() + 'static>> */) -> Self {
        Timeout00 { //msec, cb,
            addr: addr.to_string(), uri: uri.to_string(), // !!
//            payload: None, // !!
            payload: Rc::new(RefCell::new(None)), // !!
            _waker: Some(AtomicWaker::new()),
        }
    }
}

impl Future for Timeout00 {
    //type Output = ();
    type Output = Vec<u8>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<<Self as Future>::Output> {
        println!("@@ Timeout00::poll(): ^^");

        if let Some(_waker) = self._waker.take() {
            _waker.register(&cx.waker());

            // super::Xbd::set_timeout(self.msec, move || {
            //     println!("@@ !! timeout00, calling `_waker.wake()`");
            //     _waker.wake();
            // });
            //====
            let plc = self.payload.clone();
            //========== WIP !!!!!!
            super::Xbd::gcoap_get(&self.addr, &self.uri, move |payload| {
                crate::println!("!!!!00 payload: {:?}", payload);

                plc.borrow_mut().replace(payload); // !!
                _waker.wake();
            });
            //========== !!!! mock call test ^^
            // let cb = move |payload| { // !!
            //     crate::println!("!!!!00 payload: {:?}", payload);
            //
            //     plc.borrow_mut().replace(payload); // !!
            //     _waker.wake();
            // };
            // cb([0, 0, 0].to_vec());
            //========== !!!! mock call test $$

            println!("@@ !!00 returning `Poll::Pending`");

            Poll::Pending
        } else {
            println!("@@ !!00 returning `Poll::Ready(())`");
            //self.cb.take().map(|cb| cb());

            //Poll::Ready(())
            Poll::Ready(self.payload.take().unwrap())
        }
    }
}
