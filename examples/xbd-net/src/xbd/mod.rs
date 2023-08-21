mod callbacks;
pub use callbacks::process_timeout_callbacks;

use mcu_if::{println, alloc::{rc::Rc, boxed::Box}, c_types::c_void};

pub type SleepFnPtr = unsafe extern "C" fn(u32);
pub type SetTimeoutFnPtr = unsafe extern "C" fn(u32, *const c_void, *mut (*const c_void, *mut *const c_void), *mut *const c_void);

pub struct Xbd {
    _usleep: SleepFnPtr,
    _ztimer_msleep: SleepFnPtr,
    _ztimer_set: SetTimeoutFnPtr,
}

impl Xbd {
    pub fn new(
        xbd_usleep: SleepFnPtr,
        xbd_ztimer_msleep: SleepFnPtr,
        xbd_ztimer_set: SetTimeoutFnPtr
    ) -> Self {
        Self {
            _usleep: xbd_usleep,
            _ztimer_msleep: xbd_ztimer_msleep,
            _ztimer_set: xbd_ztimer_set,
        }
    }

    pub fn usleep(&self, usec: u32) {
        unsafe { (self._usleep)(usec); }
    }

    pub fn msleep(&self, msec: u32) {
        unsafe { (self._ztimer_msleep)(msec); }
    }

    pub fn set_timeout<F>(&self, msec: u32, cb: F) where F: FnOnce() + 'static {
        let timeout_ptr = Box::new(core::ptr::null());
        let timeout_pp = Box::into_raw(timeout_ptr);
        let arg = Box::new((callbacks::into_raw(cb), timeout_pp));

        unsafe {
            (self._ztimer_set)(
                msec,
                callbacks::add_timeout_callback as *const _, // cb_handler
                Box::into_raw(arg), // arg_ptr
                timeout_pp); // timeout_pp
        }
    }

    //

    pub fn async_set_timeout<F>(
        xbd: Rc<Xbd>, msec: u32, cb: Option<F>
    ) -> impl Future<Output = ()> + 'static where F: FnOnce() + 'static {
        println!("@@ async_set_timeout(): ^^");

        Timeout::new(xbd, msec)
    }
}

//

use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use futures_util::task::AtomicWaker;

pub struct Timeout {
    xbd: Rc<Xbd>,
    msec: u32,
    //cb: Box<dyn FnOnce() + 'static>,
    _waker: Option<AtomicWaker>,
}

impl Timeout {
    pub fn new(xbd: Rc<Xbd>, msec: u32) -> Self {
        Timeout {
            xbd,
            msec,
            //cb,
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

            self.xbd.set_timeout(self.msec, move || {
                println!("@@ !! timeout, calling `_waker.wake()`");
                _waker.wake();
            });

            println!("@@ !! returning `Poll::Pending`");
            Poll::Pending
        } else {
            println!("@@ !! returning `Poll::Ready(())`");
            Poll::Ready(())
        }
    }
}