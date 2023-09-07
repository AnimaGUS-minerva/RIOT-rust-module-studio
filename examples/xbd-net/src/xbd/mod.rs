mod callbacks;
pub use callbacks::process_timeout_callbacks;

mod timeout;
use timeout::Timeout;

use core::future::Future;
use conquer_once::spin::OnceCell;
use mcu_if::{alloc::{boxed::Box, vec::Vec}, c_types::c_void, null_terminate_str, utils::u8_slice_from};

pub type SleepFnPtr = unsafe extern "C" fn(u32);
pub type SetTimeoutFnPtr = unsafe extern "C" fn(u32, *const c_void, *mut (*const c_void, *mut *const c_void), *mut *const c_void);
pub type GcoapReqSendFnPtr = unsafe extern "C" fn(*const u8, *const u8 /* WIP */);

extern "C" {
    fn _xbd_resp_handler(memo: *const c_void, pdu: *const c_void, remote: *const c_void,
                         payload: *mut c_void, payload_len: *mut c_void);
}

static XBD_CELL: OnceCell<Xbd> = OnceCell::uninit();

pub fn init_once(
    xbd_usleep: SleepFnPtr,
    xbd_ztimer_msleep: SleepFnPtr,
    xbd_ztimer_set: SetTimeoutFnPtr,
    xbd_gcoap_req_send: GcoapReqSendFnPtr
) {
    XBD_CELL
        .try_init_once(|| Xbd::_new(xbd_usleep, xbd_ztimer_msleep, xbd_ztimer_set, xbd_gcoap_req_send))
        .expect("init_once() should only be called once");
}

pub struct Xbd {
    _usleep: SleepFnPtr,
    _ztimer_msleep: SleepFnPtr,
    _ztimer_set: SetTimeoutFnPtr,
    _gcoap_req_send: GcoapReqSendFnPtr,
}

impl Xbd {
    fn _new(
        xbd_usleep: SleepFnPtr,
        xbd_ztimer_msleep: SleepFnPtr,
        xbd_ztimer_set: SetTimeoutFnPtr,
        xbd_gcoap_req_send: GcoapReqSendFnPtr
    ) -> Self {
        Self {
            _usleep: xbd_usleep,
            _ztimer_msleep: xbd_ztimer_msleep,
            _ztimer_set: xbd_ztimer_set,
            _gcoap_req_send: xbd_gcoap_req_send,
        }
    }

    fn get_ref() -> &'static Self { XBD_CELL.try_get().unwrap() }

    //

    pub fn usleep(usec: u32) {
        unsafe { (Self::get_ref()._usleep)(usec); }
    }

    pub fn msleep(msec: u32) {
        unsafe { (Self::get_ref()._ztimer_msleep)(msec); }
    }

    pub fn set_timeout<F>(msec: u32, cb: F) where F: FnOnce() + 'static {
        let timeout_ptr = Box::new(core::ptr::null());
        let timeout_pp = Box::into_raw(timeout_ptr);
        let arg = Box::new((callbacks::into_raw(cb), timeout_pp));

        unsafe {
            (Self::get_ref()._ztimer_set)(
                msec,
                callbacks::add_timeout_callback as *const _, // cb_handler
                Box::into_raw(arg), // arg_ptr
                timeout_pp); // timeout_pp
        }
    }

    // !!!! WIP
    pub fn gcoap_get<F>(addr: &str, uri: &str, cb: F) where F: FnOnce(&[u8]) + 'static {
        // let timeout_ptr = Box::new(core::ptr::null());
        // let timeout_pp = Box::into_raw(timeout_ptr);
        // let arg = Box::new((callbacks::into_raw(cb), timeout_pp));

        unsafe {
            //(Self::get_ref()._gcoap_client_send)(
                // msec,
                // callbacks::add_timeout_callback as *const _, // cb_handler
                // Box::into_raw(arg), // arg_ptr
                // timeout_pp); // timeout_pp
                //==== !!!! no provably
                // uint8_t *buf, size_t len, char *addr_str
                //callbacks::add_timeout_callback as *const _, // cb_handler !!!! can re-use??
            //==== !!!! WIP, more directly
            (Self::get_ref()._gcoap_req_send)(
                //buf, len, remote, _resp_handler_xx, NULL // !!!! WIP
                null_terminate_str!(addr).as_ptr(),
                null_terminate_str!(uri).as_ptr(),
            )
        }
    }

    pub fn async_gcoap_get(addr: &str, uri: &str) -> impl Future<Output = Vec<u8>> + 'static {
        nn() // !!!!2222  cb
        //Timeout::new(msec, Some(Box::new(cb)))
    }

    //

    pub fn async_sleep(msec: u32) -> impl Future<Output = ()> + 'static {
        Timeout::new(msec, None)
    }

    pub fn async_set_timeout<F>(msec: u32, cb: F) -> impl Future<Output = ()> + 'static where F: FnOnce() + 'static {
        Timeout::new(msec, Some(Box::new(cb)))
    }
}

//

async fn nn() -> Vec<u8> { [99].to_vec() }

#[no_mangle]
pub extern fn xbd_resp_handler(memo: *const c_void, pdu: *const c_void, remote: *const c_void) {

    let mut payload_ptr: *const u8 = core::ptr::null_mut();
    let mut payload_len: usize = 0;
    unsafe {
        _xbd_resp_handler(
            memo, pdu, remote,
            (&mut payload_ptr) as *mut *const u8 as *mut c_void,
            (&mut payload_len) as *mut usize as *mut c_void);
    }

    crate::println!("!!!! payload_ptr: {:?}", &payload_ptr); // ok
    crate::println!("!!!! payload_len: {}", payload_len); // ok

    let payload = u8_slice_from(payload_ptr, payload_len).to_vec(); // ok
    crate::println!("!!!! payload: {:?}", payload);
}