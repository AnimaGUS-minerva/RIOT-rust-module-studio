use mcu_if::{println, alloc::boxed::Box, c_types::c_void};

type CVoidPtr = *const c_void;
pub type SleepFnPtr = unsafe extern "C" fn(u32);
pub type SetTimeoutFnPtr = unsafe extern "C" fn(u32, CVoidPtr, CVoidPtr);

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
        let cb: Box<Box<dyn FnOnce() + 'static>> = Box::new(Box::new(cb));
        let cb_ptr = Box::into_raw(cb) as *const _;
        let cb_handler = add_timeout_callback as *const _;

        unsafe { (self._ztimer_set)(msec, cb_handler, cb_ptr); }
    }

    pub fn cb_from(cb_ptr: CVoidPtr) -> Box<Box<dyn FnOnce() + 'static>> {
        unsafe { Box::from_raw(cb_ptr as *mut _) }
    }
}

pub async fn process_timeout_callbacks() {
    let mut callbacks = CallbackStream::new();

    while let Some(cb_ptr) = callbacks.next().await {
        let cb = Xbd::cb_from(cb_ptr as CVoidPtr);
        (*cb)(); // call, move, drop
    }
}

//

use conquer_once::spin::OnceCell;
use core::{pin::Pin, task::{Context, Poll}};
use crossbeam_queue::ArrayQueue;
use futures_util::{stream::{Stream, StreamExt}, task::AtomicWaker};

const CALLBACK_QUEUE_CAP_DEFAULT: usize = 100;

static CALLBACK_QUEUE: OnceCell<ArrayQueue<u32>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

fn add_timeout_callback(cb_ptr: CVoidPtr) {
    if let Ok(queue) = CALLBACK_QUEUE.try_get() {
        if let Err(_) = queue.push(cb_ptr as u32) {
            println!("WARNING: callback queue full; dropping callbacks");
            drop(Xbd::cb_from(cb_ptr));
        } else {
            WAKER.wake();
        }
    } else {
        println!("WARNING: callback queue uninitialized");
        drop(Xbd::cb_from(cb_ptr));
    }
}

struct CallbackStream {
    _private: (),
}

impl CallbackStream {
    pub fn new() -> Self {
        CALLBACK_QUEUE
            .try_init_once(|| ArrayQueue::new(CALLBACK_QUEUE_CAP_DEFAULT))
            .expect("CallbackStream::new should only be called once");
        CallbackStream { _private: () }
    }
}

impl Stream for CallbackStream {
    type Item = CVoidPtr;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<CVoidPtr>> {
        let queue = CALLBACK_QUEUE
            .try_get()
            .expect("callback queue not initialized");

        // fast path
        if let Some(cb_ptr) = queue.pop() {
            return Poll::Ready(Some(cb_ptr as CVoidPtr));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Some(cb_ptr) => {
                WAKER.take();
                Poll::Ready(Some(cb_ptr as CVoidPtr))
            }
            None => Poll::Pending,
        }
    }
}