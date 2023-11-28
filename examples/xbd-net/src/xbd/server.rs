use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use futures_util::{stream::StreamExt, task::AtomicWaker};
use mcu_if::{println, alloc::boxed::Box, c_types::c_void, null_terminate_str};
use super::stream::XbdStream;
use super::callbacks::{PtrSend, arg_from};

extern "C" {
    fn server_init() -> i8;
    fn _on_sock_udp_evt_minerva(sock: *const c_void, flags: usize, arg: *const c_void);
    fn riot_board_handler_minerva(
        pdu: *const c_void, buf: *const c_void, len: usize, ctx: *const c_void,
        board: *const u8) -> isize;
}

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

fn add_xbd_gcoap_server_sock_udp_event_callback(arg_ptr: *const c_void) {
    add_server_callback(ServerCallback::GcoapServerSockUdpEvt(arg_ptr as PtrSend));
}

pub fn start_gcoap_server() -> Result<(), i8> {
    let ret = unsafe { server_init() };

    if ret == 0 { Ok(()) } else { Err(ret) }
}

pub async fn process_gcoap_server_callbacks() {
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

/*
#[derive(Debug)]
pub enum GcoapServeResource {
    RiotBoard(Option<Vec<u8>>),
    Stats,
}

pub struct GcoapServe {
    addr: String,
    uri: String,
    out: Rc<RefCell<Option<GcoapServeResource>>>,
    _waker: Option<AtomicWaker>,
}

impl GcoapServe {
    pub fn new(addr: &str, uri: &str) -> Self {
        GcoapServe {
            addr: addr.to_string(),
            uri: uri.to_string(),
            out: Rc::new(RefCell::new(None)),
            _waker: Some(AtomicWaker::new()),
        }
    }
}

impl Future for GcoapServe {
    type Output = GcoapServeResource;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<<Self as Future>::Output> {
        if let Some(_waker) = self._waker.take() {
            _waker.register(&cx.waker());

            let outc = self.out.clone();
            // super::Xbd::gcoap_get(&self.addr, &self.uri, move |out| {
            //     outc.borrow_mut().replace(out);
            //     _waker.wake();
            // });

            //Poll::Pending
            Poll::Ready(GcoapServeResource::RiotBoard(None)) // !!!! debuggggg
        } else {
            Poll::Ready(self.out.take().unwrap())
        }
    }
}
*/

//

#[no_mangle]
pub extern fn xbd_on_sock_udp_evt(sock: *const c_void, flags: usize, arg: *const c_void) {
    println!("@@ xbd_on_sock_udp_evt(): sock: {:?} type: {:?} arg: {:?}", sock, flags, arg);

    let cb_ptr = core::ptr::null::<()>();
    let evt_args = (sock, flags, arg);

    add_xbd_gcoap_server_sock_udp_event_callback(
        Box::into_raw(Box::new((cb_ptr, evt_args))) as *const c_void); // arg_ptr
}

//#[no_mangle]// !!!! rust wrapper, get, put
//pub extern fn xbd_stats_handler(

#[no_mangle]// !!!! rust wrapper, get
pub extern fn xbd_riot_board_handler(
    pdu: *const c_void, buf: *const c_void, len: usize, ctx: *const c_void) -> isize {
    let board = null_terminate_str!("minerva");

    let pdu_len = unsafe { riot_board_handler_minerva(pdu, buf, len, ctx, board.as_ptr()) };
    println!("@@ xbd_riot_board_handler(): pdu_len: {:?}", pdu_len);
    return pdu_len;
}