use conquer_once::spin::OnceCell;
use core::{pin::Pin, task::{Context, Poll}};
use crossbeam_queue::ArrayQueue;
use futures_util::{stream::{Stream, StreamExt}, task::AtomicWaker};
use mcu_if::{alloc::boxed::Box, c_types::c_void};
use super::gcoap::{GcoapMemoState, GcoapServeResource};

extern "C" {
    fn free(ptr: *mut c_void);
}

type CVoidPtr = *const c_void;
type PtrSend = u32; // support RIOT 32bit MCUs only

enum XbdCallback {
    Timeout(PtrSend),
    _GcoapPing(PtrSend),
    GcoapGet(PtrSend),
    ServeRiotBoard(PtrSend),
    //ServeStats(PtrSend),
}

pub async fn process_xbd_callbacks() {
    let mut callbacks = CallbackStream::new();

    while let Some(xbd_callback) = callbacks.next().await {
        match xbd_callback {
            XbdCallback::Timeout(arg_ptr) => {
                let (cb_ptr, timeout_pp): (CVoidPtr, *mut CVoidPtr) = arg_from(arg_ptr);

                let timeout_ptr = unsafe { *Box::from_raw(timeout_pp) };
                assert_ne!(timeout_ptr, core::ptr::null());
                unsafe { free(timeout_ptr as *mut _); }

                call(cb_ptr, ());
            },
            XbdCallback::_GcoapPing(_) => todo!(),
            XbdCallback::GcoapGet(arg_ptr) => {
                let (cb_ptr, out) = arg_from::<GcoapMemoState>(arg_ptr);
                call(cb_ptr, out);
            },
            XbdCallback::ServeRiotBoard(arg_ptr) => { // !! todo --> XbdCallback::SockUdpEvt(arg_ptr)
                let (cb_ptr, evt_args) = arg_from::<(*const c_void, usize, *const c_void)>(arg_ptr);
                crate::println!("!!!!11 evt_args: {:?}", evt_args); //panic!("wp 11");

                //====
                //let pdu_args = (); // !! (pdu, buf, len, ctx) = xx(evt_args);
                //call(cb_ptr, pdu_args);
                //====
                let res = crate::xbd::gcoap::GcoapServe::new("param", "param").await; // !!
                //panic!("!!!!11 res: {:?}", res);
                //====
            },
        }
    }
}

pub fn into_raw<F, T>(cb: F) -> CVoidPtr where F: FnOnce(T) + 'static {
    let cb: Box<Box<dyn FnOnce(T) + 'static>> = Box::new(Box::new(cb));

    Box::into_raw(cb) as *const _
}

fn arg_from<T>(arg_ptr: PtrSend) -> (CVoidPtr, T) {
    unsafe { *Box::from_raw(arg_ptr as *mut _) }
}

fn cb_from<T>(cb_ptr: CVoidPtr) -> Box<Box<dyn FnOnce(T) + 'static>> {
    unsafe { Box::from_raw(cb_ptr as *mut _) }
}

fn call<T>(cb_ptr: CVoidPtr, out: T) {
    assert_ne!(cb_ptr, core::ptr::null());
    (*(cb_from(cb_ptr)))(out); // call, move, drop
}

//

const CALLBACK_QUEUE_CAP_DEFAULT: usize = 100;

static CALLBACK_QUEUE: OnceCell<ArrayQueue<XbdCallback>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

pub fn add_xbd_timeout_callback(arg_ptr: CVoidPtr) {
    add_xbd_callback(XbdCallback::Timeout(arg_ptr as PtrSend));
}
pub fn add_xbd_gcoap_get_callback(arg_ptr: CVoidPtr) {
    add_xbd_callback(XbdCallback::GcoapGet(arg_ptr as PtrSend));
}
pub fn add_xbd_gcoap_serve_riot_board_callback(arg_ptr: CVoidPtr) {
    add_xbd_callback(XbdCallback::ServeRiotBoard(arg_ptr as PtrSend));
}

fn add_xbd_callback(xbd_callback: XbdCallback) { // must not block/alloc/dealloc
    if let Ok(queue) = CALLBACK_QUEUE.try_get() {
        if let Err(_) = queue.push(xbd_callback) {
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
    type Item = XbdCallback;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<XbdCallback>> {
        let queue = CALLBACK_QUEUE
            .try_get()
            .expect("callback queue not initialized");

        // fast path
        if let Some(arg_ptr) = queue.pop() {
            return Poll::Ready(Some(arg_ptr));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Some(arg_ptr) => {
                WAKER.take();
                Poll::Ready(Some(arg_ptr))
            }
            None => Poll::Pending,
        }
    }
}