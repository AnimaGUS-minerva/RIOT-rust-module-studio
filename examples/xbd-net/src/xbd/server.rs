use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use futures_util::{stream::StreamExt, task::AtomicWaker};
use mcu_if::{println, alloc::boxed::Box, c_types::c_void, null_terminate_str};
use super::stream::XbdStream;
use super::callbacks::{PtrSend, arg_from};

extern "C" {
    fn server_init() -> i8;
    fn _on_sock_udp_evt_minerva(sock: *const c_void, flags: usize, arg: *const c_void);
    fn get_kludge_force_no_async() -> bool; // !!
    fn riot_stats_handler_minerva(
        pdu: *const c_void, buf: *const c_void, len: usize, ctx: *const c_void) -> isize;
    fn riot_board_handler_minerva(
        pdu: *const c_void, buf: *const c_void, len: usize, ctx: *const c_void,
        board: *const u8) -> isize;
    fn gcoap_fileserver_handler(
        pdu: *const c_void, buf: *const c_void, len: usize, ctx: *const c_void) -> isize;
}

enum ServerCallback {
    GcoapServerSockUdpEvt(PtrSend),
    GcoapServerSockDtlsEvt(PtrSend),
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

pub async fn process_gcoap_server_stream() {
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
                println!("@@ process_gcoap_server_stream(): calling _on_sock_udp_evt_minerva(sock, flags, arg)");
                // TODO check comp ,log--get-blockwise-sync (flags)
                unsafe { _on_sock_udp_evt_minerva(sock, flags, arg) };
                //====
                //let pdu_args = (); // !! (pdu, buf, len, ctx) = xx(evt_args);
                // let pdu_len = unsafe { riot_board_handler_fill(pdu, buf, len, ctx, board.as_ptr()) };
                // panic!("!!!!22 pdu_len: {:?}", pdu_len);

                // ........... send
                //====
            },
            ServerCallback::GcoapServerSockDtlsEvt(_) => todo!(),
        }
    }
}

//

#[no_mangle]
pub extern fn xbd_on_sock_udp_evt(sock: *const c_void, flags: usize, arg: *const c_void) {
    println!("@@ xbd_on_sock_udp_evt(): sock: {:?} type: {:?} arg: {:?}", sock, flags, arg);

    let cb_ptr = core::ptr::null::<()>();
    let evt_args = (sock, flags, arg);

    let flag = unsafe { get_kludge_force_no_async() }; // !!
    if flag { //==== Xbd::async_gcoap_get(): NG (FIXME), Xbd::gcoap_get(): ok
        unsafe { _on_sock_udp_evt_minerva(sock, flags, arg) };
    } else { //==== Xbd::async_gcoap_get(): ok, Xbd::gcoap_get(): NG (FIXME)
        add_xbd_gcoap_server_sock_udp_event_callback(
            Box::into_raw(Box::new((cb_ptr, evt_args))) as *const c_void); // arg_ptr
    }
}

#[no_mangle]
pub extern fn xbd_on_sock_dtls_evt(sock: *const c_void, flags: usize, arg: *const c_void) {
    println!("@@ xbd_on_sock_dtls_evt(): sock: {:?} type: {:?} arg: {:?}", sock, flags, arg);

    todo!();
}

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

//-------- !!!!
#[no_mangle]
pub extern fn xbd_kludge_async_gcoap_get_blockwise(hdr: *const c_void, hdr_len: usize) {
    use crate::xbd::gcoap::{ReqInner, COAP_METHOD_GET};
    use mcu_if::utils::u8_slice_from;
    assert!(hdr_len < LAST_BLOCKWISE_HDR_MAX);
    unsafe {
        LAST_BLOCKWISE_HDR.fill(0u8);
        LAST_BLOCKWISE_HDR[..hdr_len].
            copy_from_slice(u8_slice_from(hdr as *const u8, hdr_len));
        LAST_BLOCKWISE_LEN = hdr_len;
    }

    ReqInner::add_blockwise(
        COAP_METHOD_GET, "[::1]:5683", "/const/song.txt", None); // !!!
}

#[no_mangle]
pub extern fn xbd_kludge_update_blockwise_hdr(buf: *mut u8, buf_sz: usize) -> usize {
    use mcu_if::utils::u8_slice_mut_from;

    unsafe {
        let len = LAST_BLOCKWISE_LEN;
        if len > 0 {
            u8_slice_mut_from(buf, buf_sz)[..len].
                copy_from_slice(&LAST_BLOCKWISE_HDR[..len]);

            LAST_BLOCKWISE_HDR.fill(0u8);
            LAST_BLOCKWISE_LEN = 0;
        }

        len
    }
}
pub const LAST_BLOCKWISE_HDR_MAX: usize = 64;
pub static mut LAST_BLOCKWISE_HDR: &'static mut [u8] = &mut [0; LAST_BLOCKWISE_HDR_MAX];
pub static mut LAST_BLOCKWISE_LEN: usize = 0;
//-------- !!!!
