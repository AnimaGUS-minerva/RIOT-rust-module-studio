mod callback;
pub use callback::process_api_stream;
use callback::{
    add_xbd_timeout_callback,
//    add_xbd_gcoap_req_callback
};

mod server;
pub use server::process_gcoap_server_stream;

mod shell;
pub use shell::process_shell_stream;

mod stream;

mod blockwise;
use blockwise::{BlockwiseStream, BlockwiseData, BLOCKWISE_HDR_MAX};
pub use blockwise::{
    BlockwiseError, BLOCKWISE_STATES_MAX,
    blockwise_states_print, blockwise_states_debug};

mod timeout;
use timeout::Timeout;

mod gcoap;
use gcoap::{COAP_METHOD_GET, REQ_ADDR_MAX, REQ_URI_MAX, Progress};
pub use gcoap::GcoapMemoState;

use core::future::Future;
use conquer_once::spin::OnceCell;
use mcu_if::{
    alloc::{boxed::Box, vec::Vec, collections::BTreeMap, string::{String, ToString}},
    c_types::c_void, utils::u8_slice_from};
//use crate::println;

extern "C" {
    fn strlen(ptr: *const u8) -> usize;
    fn xbd_resp_handler(
        memo: *const c_void, pdu: *const c_void, remote: *const c_void,
        payload: *mut c_void, payload_len: *mut c_void, context: *mut c_void) -> u8;
}

static XBD_CELL: OnceCell<Xbd> = OnceCell::uninit();

pub type XbdFnsEnt = (*const i8, *const c_void);
fn xbd_fns_from(ptr: *const XbdFnsEnt, sz: usize) -> &'static [XbdFnsEnt] {
    unsafe { core::slice::from_raw_parts(ptr, sz) }
}

pub fn init_once(xbd_fns_ptr: *const XbdFnsEnt, xbd_fns_sz: usize) {
    let fns = BTreeMap::from_iter(
        xbd_fns_from(xbd_fns_ptr, xbd_fns_sz)
            .iter()
            .map(|&(name, ptr) | {
                let name = name as *const u8;
                let name = unsafe { core::str::from_utf8_unchecked(
                    core::slice::from_raw_parts(name, strlen(name))).to_string() };

                (name, ptr as Ptr32Send)
            })
            .collect::<Vec<(_, _)>>());

    XBD_CELL
        .try_init_once(|| Xbd(fns))
        .expect("init_once() should only be called once");
}

type Ptr32Send = u32;
pub struct Xbd(BTreeMap<String, Ptr32Send>);

macro_rules! get_xbd_fn {
    ($name:expr, $t:ty) => {
        core::mem::transmute::<_, $t>(Xbd::get_ptr($name))
    };
}

impl Xbd {
    fn get_ptr(name: &str) -> *const c_void {
        XBD_CELL.try_get().unwrap().0.get(name).copied().unwrap() as _
    }

    pub fn usleep(usec: u32) {
        type Ty = unsafe extern "C" fn(u32);
        unsafe { (get_xbd_fn!("xbd_usleep", Ty))(usec); }
    }

    pub fn msleep(msec: u32, debug: bool) {
        type Ty = unsafe extern "C" fn(u32, bool);
        unsafe { (get_xbd_fn!("xbd_ztimer_msleep", Ty))(msec, debug); }
    }

    pub fn set_timeout<F>(msec: u32, cb: F) where F: FnOnce(()) + 'static {
        let timeout_ptr = Box::new(core::ptr::null());
        let timeout_pp = Box::into_raw(timeout_ptr);
        let arg = Box::new((callback::into_raw(cb), timeout_pp));

        type Ty = unsafe extern "C" fn(
            u32, *const c_void, *mut (*const c_void, *mut *const c_void), *mut *const c_void);
        unsafe {
            (get_xbd_fn!("xbd_ztimer_set", Ty))(
                msec,
                add_xbd_timeout_callback as *const _, // cb_handler
                Box::into_raw(arg), // arg_ptr
                timeout_pp); // timeout_pp
        }
    }

    // TODO move to 'gcoap.rs'
    fn gcoap_req_v2(addr: &str, uri: &str, method: gcoap::CoapMethod,
                   payload: Option<&[u8]>, blockwise: bool, blockwise_state_index: Option<usize>,
                   progress_ptr: *mut Progress) {
        let payload_ptr = payload.map_or(core::ptr::null(), |payload| payload.as_ptr());
        let payload_len = payload.map_or(0, |payload| payload.len());

        let mut addr_cstr = heapless::String::<{ REQ_ADDR_MAX + 1 }>::new();
        addr_cstr.push_str(addr).unwrap();
        addr_cstr.push('\0').unwrap();

        let mut uri_cstr = heapless::String::<{ REQ_URI_MAX + 1 }>::new();
        uri_cstr.push_str(uri).unwrap();
        uri_cstr.push('\0').unwrap();

        type Ty = unsafe extern "C" fn(
            *const u8, *const u8, u8,
            *const u8, usize, bool, usize, *const c_void, *const c_void);

        assert_eq!(blockwise, blockwise_state_index.is_some());
        unsafe {
            (get_xbd_fn!("xbd_gcoap_req_send", Ty))(
                addr_cstr.as_ptr(),
                uri_cstr.as_ptr(),
                method, payload_ptr, payload_len,
                blockwise, blockwise_state_index.unwrap_or(0 /* to be ignored */),
                progress_ptr as *const c_void, // context
                Self::gcoap_req_resp_handler_v2 as *const c_void);
        }
    }

    // TODO move to 'gcoap.rs'
    fn gcoap_req_resp_handler_v2(memo: *const c_void, pdu: *const c_void, remote: *const c_void) {
        let mut context: *const c_void = core::ptr::null_mut();
        let mut payload_ptr: *const u8 = core::ptr::null_mut();
        let mut payload_len: usize = 0;

        let memo_state = unsafe {
            xbd_resp_handler(
                memo, pdu, remote,
                (&mut payload_ptr) as *mut *const u8 as *mut c_void,
                (&mut payload_len) as *mut usize as *mut c_void,
                (&mut context) as *mut *const c_void as *mut c_void) };

        let payload = if payload_len > 0 {
            let hvec: gcoap::PayloadOut = heapless::Vec::from_slice(
                u8_slice_from(payload_ptr, payload_len)).unwrap();
            Some(hvec)
        } else {
            assert_eq!(payload_ptr, core::ptr::null_mut());
            None
        };

        let memo = GcoapMemoState::new(memo_state, payload);
        Progress::get_ref_mut(context as *mut _).finish(memo);
    }

    pub fn async_sleep(msec: u32) -> impl Future<Output = ()> + 'static {
        Timeout::new(msec, None)
    }

    pub fn async_set_timeout<F>(msec: u32, cb: F) -> impl Future<Output = ()> + 'static where F: FnOnce() + 'static {
        Timeout::new(msec, Some(Box::new(cb)))
    }

    pub fn async_gcoap_get(addr: &str, uri: &str) -> impl Future<Output = GcoapMemoState> + 'static {
        gcoap::Req::new(COAP_METHOD_GET, addr, uri, None)
    }

    pub fn async_gcoap_get_blockwise(addr: &str, uri: &str) -> Result<BlockwiseStream, BlockwiseError> {
        BlockwiseData::send_blockwise_req(None, Some((addr, uri)), None)
    }

    pub fn async_gcoap_post(addr: &str, uri: &str, payload: &[u8]) -> impl Future<Output = GcoapMemoState> + 'static {
        gcoap::Req::new(gcoap::COAP_METHOD_POST, addr, uri, Some(heapless::Vec::from_slice(payload).unwrap()))
    }

    pub fn async_gcoap_put(addr: &str, uri: &str, payload: &[u8]) -> impl Future<Output = GcoapMemoState> + 'static {
        gcoap::Req::new(gcoap::COAP_METHOD_PUT, addr, uri, Some(heapless::Vec::from_slice(payload).unwrap()))
    }
}