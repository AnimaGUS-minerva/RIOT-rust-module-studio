use conquer_once::spin::OnceCell;
use core::{pin::Pin, task::{Context, Poll}};
use crossbeam_queue::ArrayQueue;
use futures_util::{stream::{Stream, StreamExt}, task::AtomicWaker};
use mcu_if::{println, alloc::boxed::Box, c_types::c_void};

type CVoidPtr = *const c_void;

//

extern "C" {
    fn free(ptr: *mut c_void);
}

pub async fn process_timeout_callbacks() {
    let mut callbacks = CallbackStream::new();

    while let Some(arg_ptr) = callbacks.next().await {
        let (cb_ptr, timeout_pp): (CVoidPtr, *mut CVoidPtr) =
            unsafe { *Box::from_raw(arg_ptr as *mut _) }; // consume `arg`

        let timeout_ptr = unsafe { *Box::from_raw(timeout_pp) };
        println!("@@ freeing timeout_ptr: {:?}", timeout_ptr);
        assert_ne!(timeout_ptr, core::ptr::null());
        unsafe { free(timeout_ptr as *mut _); }

        let cb = cb_from(cb_ptr as CVoidPtr);
        (*cb)(); // call, move, drop
    }
}

pub fn into_raw<F>(cb: F) -> CVoidPtr where F: FnOnce() + 'static {
    let cb: Box<Box<dyn FnOnce() + 'static>> = Box::new(Box::new(cb));

    Box::into_raw(cb) as *const _
}

fn cb_from(cb_ptr: CVoidPtr) -> Box<Box<dyn FnOnce() + 'static>> {
    unsafe { Box::from_raw(cb_ptr as *mut _) }
}

//

const CALLBACK_QUEUE_CAP_DEFAULT: usize = 100;

static CALLBACK_QUEUE: OnceCell<ArrayQueue<u32>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

pub fn add_timeout_callback(arg_ptr: CVoidPtr) { // must not block/alloc/dealloc
    if let Ok(queue) = CALLBACK_QUEUE.try_get() {
        if let Err(_) = queue.push(arg_ptr as u32) {
            panic!("callback queue full");
        } else {
            WAKER.wake();
        }
    } else {
        panic!("callback queue uninitialized");
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
        if let Some(arg_ptr) = queue.pop() {
            return Poll::Ready(Some(arg_ptr as CVoidPtr));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Some(arg_ptr) => {
                WAKER.take();
                Poll::Ready(Some(arg_ptr as CVoidPtr))
            }
            None => Poll::Pending,
        }
    }
}