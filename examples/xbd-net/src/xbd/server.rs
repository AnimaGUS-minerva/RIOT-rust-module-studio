use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use futures_util::{stream::StreamExt, task::AtomicWaker};
use mcu_if::c_types::c_void;
use super::stream::XbdStream;
use super::callbacks::{PtrSend, arg_from};

extern "C" {
    fn _on_sock_udp_evt_minerva(sock: *const c_void, flags: usize, arg: *const c_void);
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

pub fn add_xbd_gcoap_server_sock_udp_event_callback(arg_ptr: *const c_void) {
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