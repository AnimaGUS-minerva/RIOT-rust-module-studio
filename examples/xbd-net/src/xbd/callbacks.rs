use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use futures_util::{stream::StreamExt, task::AtomicWaker};
use mcu_if::{alloc::boxed::Box, c_types::c_void};
use super::gcoap::GcoapMemoState;
use super::stream::XbdStream;

extern "C" {
    fn free(ptr: *mut c_void);
    fn _on_sock_udp_evt_minerva(sock: *const c_void, flags: usize, arg: *const c_void);
}

type CVoidPtr = *const c_void;
type PtrSend = u32; // support RIOT 32bit MCUs only

//

enum ApiCallback {
    Timeout(PtrSend),
    _GcoapPing(PtrSend),
    GcoapGet(PtrSend),
}

static API_QUEUE: OnceCell<ArrayQueue<ApiCallback>> = OnceCell::uninit();
static API_WAKER: AtomicWaker = AtomicWaker::new();

fn add_api_callback(cb: ApiCallback) {
    XbdStream::add(&API_QUEUE, &API_WAKER, cb);
}

pub fn add_xbd_timeout_callback(arg_ptr: CVoidPtr) {
    add_api_callback(ApiCallback::Timeout(arg_ptr as PtrSend));
}
pub fn add_xbd_gcoap_get_callback(arg_ptr: CVoidPtr) {
    add_api_callback(ApiCallback::GcoapGet(arg_ptr as PtrSend));
}

pub async fn process_api_callbacks() {
    let mut stream = XbdStream::new(&API_QUEUE, &API_WAKER);

    while let Some(cb) = stream.next().await {
        match cb {
            ApiCallback::Timeout(arg_ptr) => {
                let (cb_ptr, timeout_pp): (CVoidPtr, *mut CVoidPtr) = arg_from(arg_ptr);

                let timeout_ptr = unsafe { *Box::from_raw(timeout_pp) };
                assert_ne!(timeout_ptr, core::ptr::null());
                unsafe { free(timeout_ptr as *mut _); }// !!!!

                call(cb_ptr, ());
            },
            ApiCallback::_GcoapPing(_) => todo!(),
            ApiCallback::GcoapGet(arg_ptr) => {
                let (cb_ptr, out) = arg_from::<GcoapMemoState>(arg_ptr);
                call(cb_ptr, out);
            },
        }
    }
}

//

enum ServerCallback {
    GcoapServerSockUdpEvt(PtrSend)
    //ServeRiotBoard(PtrSend),
    //ServeStats(PtrSend),
}

static SERVER_QUEUE: OnceCell<ArrayQueue<ServerCallback>> = OnceCell::uninit();
static SERVER_WAKER: AtomicWaker = AtomicWaker::new();

fn add_server_callback(cb: ServerCallback) {
    XbdStream::add(&SERVER_QUEUE, &SERVER_WAKER, cb);
}

pub fn add_xbd_gcoap_server_sock_udp_event_callback(arg_ptr: CVoidPtr) {
    add_server_callback(ServerCallback::GcoapServerSockUdpEvt(arg_ptr as PtrSend));
}

pub async fn process_server_callbacks() {
    let mut stream = XbdStream::new(&SERVER_QUEUE, &SERVER_WAKER);

    while let Some(cb) = stream.next().await {
        match cb {
            ServerCallback::GcoapServerSockUdpEvt(arg_ptr) => {
                let (cb_ptr, (sock, flags, arg) /* evt_args */) =
                    arg_from::<(*const c_void, usize, *const c_void)>(arg_ptr);
                assert_eq!(cb_ptr, core::ptr::null());

                //let _ = crate::xbd::gcoap::GcoapServe::new("param", "param").await; // ok
                if 0 == 1 { crate::Xbd::async_sleep(100).await; } // ok

                //====
                unsafe { _on_sock_udp_evt_minerva(sock, flags, arg) };
                //====
                //let pdu_args = (); // !! (pdu, buf, len, ctx) = xx(evt_args);
                // let pdu_len = unsafe { riot_board_handler_fill(pdu, buf, len, ctx, board.as_ptr()) };
                // panic!("!!!!22 pdu_len: {:?}", pdu_len);

                // ........... send
                //====
            },
        }
    }
}

//

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