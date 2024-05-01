use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use futures_util::{stream::StreamExt, task::AtomicWaker};
use mcu_if::{println, alloc::boxed::Box, c_types::c_void, null_terminate_str};
use super::stream::XbdStream;
use super::callback::{Ptr32Send, arg_from};

extern "C" {
    fn server_init() -> i8;
    fn _on_sock_udp_evt_minerva(sock: *const c_void, flags: usize, arg: *const c_void);
    fn _on_sock_dtls_evt_minerva(sock: *const c_void, flags: usize, arg: *const c_void);
    fn riot_stats_handler_minerva(
        pdu: *const c_void, buf: *const c_void, len: usize, ctx: *const c_void) -> isize;
    fn riot_board_handler_minerva(
        pdu: *const c_void, buf: *const c_void, len: usize, ctx: *const c_void,
        board: *const u8) -> isize;
    fn gcoap_fileserver_handler(
        pdu: *const c_void, buf: *const c_void, len: usize, ctx: *const c_void) -> isize;
}

enum ServerCallback {
    GcoapServerSockUdpMsgRecv(Ptr32Send),
    GcoapServerSockDtlsMsgRecv(Ptr32Send),
    //ServeRiotBoard(Ptr32Send),
    //ServeStats(Ptr32Send),
}

static SERVER_QUEUE: OnceCell<ArrayQueue<ServerCallback>> = OnceCell::uninit();
static SERVER_WAKER: AtomicWaker = AtomicWaker::new();

pub async fn process_gcoap_server_stream() -> Result<(), i8> {
    let ret = unsafe { server_init() };
    if ret != 0 { return Err(ret); }

    let mut stream = XbdStream::new(&SERVER_QUEUE, &SERVER_WAKER);
    let unpack = |arg_ptr| arg_from::<(*const c_void, usize, *const c_void)>(arg_ptr);

    while let Some(cb) = stream.next().await {
        if 0 == 1 { crate::Xbd::async_sleep(1_000).await; } // debug, ok

        match cb {
            ServerCallback::GcoapServerSockUdpMsgRecv(arg_ptr) => {
                let (cb_ptr, (sock, flags, arg)) = unpack(arg_ptr);
                assert_eq!(cb_ptr, core::ptr::null());

                // TODO rust impl process/send
                // e.g.
                // let pdu_args = (); // !! (pdu, buf, len, ctx) = xx(sock, flags, arg);
                // let pdu_len = unsafe { riot_board_handler_fill(pdu, buf, len, ctx, board.as_ptr()) };
                // .... send

                unsafe { _on_sock_udp_evt_minerva(sock, flags, arg) };
            },
            ServerCallback::GcoapServerSockDtlsMsgRecv(arg_ptr) => {
                let (cb_ptr, (sock, flags, arg)) = unpack(arg_ptr);
                assert_eq!(cb_ptr, core::ptr::null());

                unsafe { _on_sock_dtls_evt_minerva(sock, flags, arg) };
            },
        }
    }

    Ok(())
}

//

#[no_mangle]
pub extern fn xbd_on_sock_udp_evt(sock: *const c_void, flags: usize, arg: *const c_void) {
    on_sock_evt(false, sock, flags, arg);
}

#[no_mangle]
pub extern fn xbd_on_sock_dtls_evt(sock: *const c_void, flags: usize, arg: *const c_void) {
    on_sock_evt(true, sock, flags, arg);
}

// cf. RIOT/sys/include/net/sock/async/types.h
// const SOCK_ASYNC_CONN_RDY  : usize = 0x0001;  /**< Connection ready event */
// const SOCK_ASYNC_CONN_FIN  : usize = 0x0002;  /**< Connection finished event */
// const SOCK_ASYNC_CONN_RECV : usize = 0x0004;  /**< Listener received connection event */
const SOCK_ASYNC_MSG_RECV  : usize = 0x0010;  /**< Message received event */
// const SOCK_ASYNC_MSG_SENT  : usize = 0x0020;  /**< Message sent event */
// const SOCK_ASYNC_PATH_PROP : usize = 0x0040;  /**< Path property changed event */

fn on_sock_evt(is_dtls: bool, sock: *const c_void, flags: usize, arg: *const c_void) {
    //println!("@@ on_sock_evt(): is_dtls: {} sock: {:?} type: {:?} arg: {:?}", is_dtls, sock, flags, arg);
    // !!!!  check against "type: 48" stuff in ',log--get-blockwise-sync'

    let bypass_async_server = || if is_dtls {
        unsafe { _on_sock_dtls_evt_minerva(sock, flags, arg) };
    } else {
        unsafe { _on_sock_udp_evt_minerva(sock, flags, arg) };
    };

    if let Some(stream) = XbdStream::get(&SERVER_QUEUE, &SERVER_WAKER) {
        if flags & SOCK_ASYNC_MSG_RECV > 0 {
            let arg_ptr = into_arg_ptr(sock, flags, arg) as _;
            stream.add(if is_dtls {
                ServerCallback::GcoapServerSockDtlsMsgRecv(arg_ptr)
            } else {
                ServerCallback::GcoapServerSockUdpMsgRecv(arg_ptr)
            });
        } else { // avoid handshake (timing/ordering) issues, directly process `SOCK_ASYNC_CONN_*`
            bypass_async_server();
        }
    } else { // async server not available
        bypass_async_server();
    }
}

fn into_arg_ptr(sock: *const c_void, flags: usize, arg: *const c_void) -> *const c_void {
    let cb_ptr = core::ptr::null::<()>();
    let evt_args = (sock, flags, arg);

    Box::into_raw(Box::new((cb_ptr, evt_args))) as _
}

//

#[no_mangle]
pub extern fn xbd_riot_stats_handler(
    pdu: *const c_void, buf: *const c_void, len: usize, ctx: *const c_void) -> isize {

    let pdu_len = unsafe { riot_stats_handler_minerva(pdu, buf, len, ctx) };
    println!("@@ xbd_riot_stats_handler(): pdu_len: {:?}", pdu_len);

    pdu_len
}

#[no_mangle]
pub extern fn xbd_riot_board_handler(
    pdu: *const c_void, buf: *const c_void, len: usize, ctx: *const c_void) -> isize {
    let board = null_terminate_str!("minerva");

    let pdu_len = unsafe { riot_board_handler_minerva(pdu, buf, len, ctx, board.as_ptr()) };
    println!("@@ xbd_riot_board_handler(): pdu_len: {:?}", pdu_len);

    pdu_len
}

#[no_mangle]
pub extern fn xbd_riot_fileserver_handler(
    pdu: *const c_void, buf: *const c_void, len: usize, ctx: *const c_void) -> isize {

    let pdu_len = unsafe { gcoap_fileserver_handler(pdu, buf, len, ctx) };
    println!("@@ xbd_riot_fileserver_handler(): pdu_len: {:?}", pdu_len);

    pdu_len
}