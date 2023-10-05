mod callbacks;
pub use callbacks::process_xbd_callbacks;
use callbacks::{add_xbd_timeout_callback, add_xbd_gcoap_get_callback};

mod timeout;
use timeout::Timeout;

mod gcoap;
use gcoap::GcoapGet;

use core::future::Future;
use conquer_once::spin::OnceCell;
use mcu_if::{
    alloc::{boxed::Box, vec::Vec, vec, collections::BTreeMap, string::{String, ToString}},
    println, c_types::c_void, null_terminate_str, utils::u8_slice_from};

extern "C" {
    fn _xbd_resp_handler(
        memo: *const c_void, pdu: *const c_void, remote: *const c_void,
        payload: *mut c_void, payload_len: *mut c_void, context: *mut c_void);
}

static XBD_CELL: OnceCell<Xbd> = OnceCell::uninit();

pub type XbdFnsEnt = (*const i8, *const c_void);
fn xbd_fns_from(ptr: *const XbdFnsEnt, sz: usize) -> &'static [XbdFnsEnt] {
    unsafe { core::slice::from_raw_parts(ptr, sz) }
}

pub fn init_once(xbd_fns_ptr: *const XbdFnsEnt, xbd_fns_sz: usize) {
    let xbd_fns = xbd_fns_from(xbd_fns_ptr, xbd_fns_sz);
    println!("!!!! xbd_fns: {:?}", xbd_fns);

    let fnmap = BTreeMap::from_iter(xbd_fns
        .iter()
        .map(|&(name, ptr) | {
            let name = (unsafe { core::ffi::CStr::from_ptr(name) }).to_str().unwrap().to_string();
            (name, ptr as PtrSend)
        })
        .collect::<Vec<(_, _)>>());
    println!("!!!! fnmap: {:?}", fnmap);

    XBD_CELL
        .try_init_once(|| Xbd(fnmap))
        .expect("init_once() should only be called once");

    /* !!!! test call
    xbd_fns.iter().for_each(|ent| {
        let name = unsafe { core::ffi::CStr::from_ptr(ent.0) };
        let name = name.to_str().unwrap();
        println!("!!!! name: {:?}", name);

        match name {
            "xbd_usleep" => { // ok
                // https://stackoverflow.com/questions/46134477/how-can-i-call-a-raw-address-from-rust
                let code: unsafe extern "C" fn(u32) = unsafe { core::mem::transmute(ent.1) };

                unsafe { code(1_000_000) };
                unsafe { code(2_000_000) };
            },
            "xbd_ztimer_msleep" => {},
            "xbd_ztimer_set" => {},
            "xbd_gcoap_req_send" => {},
            _ => panic!("unknown `xbd_` function"),
        }
    });
    */
}

type PtrSend = u32; // support RIOT 32bit MCUs only
pub struct Xbd(BTreeMap<String, PtrSend>);

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
        let arg = Box::new((callbacks::into_raw(cb), timeout_pp));

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

    pub fn gcoap_get<F>(addr: &str, uri: &str, cb: F) where F: FnOnce(Vec<u8>) + 'static {
        type Ty = unsafe extern "C" fn(*const u8, *const u8, *const c_void);
        unsafe {
            (get_xbd_fn!("xbd_gcoap_req_send", Ty))(
                null_terminate_str!(addr).as_ptr(),
                null_terminate_str!(uri).as_ptr(),
                callbacks::into_raw(cb)); // context
        }
    }

    //

    pub fn async_sleep(msec: u32) -> impl Future<Output = ()> + 'static {
        Timeout::new(msec, None)
    }

    pub fn async_set_timeout<F>(msec: u32, cb: F) -> impl Future<Output = ()> + 'static where F: FnOnce() + 'static {
        Timeout::new(msec, Some(Box::new(cb)))
    }

    pub fn async_gcoap_get(addr: &str, uri: &str) -> impl Future<Output = Vec<u8>> + 'static {
        GcoapGet::new(addr, uri)
    }
}

//

#[no_mangle]
pub extern fn xbd_resp_handler(memo: *const c_void, pdu: *const c_void, remote: *const c_void) {
    let mut context: *const c_void = core::ptr::null_mut();
    let mut payload_ptr: *const u8 = core::ptr::null_mut();
    let mut payload_len: usize = 0;
    unsafe {
        _xbd_resp_handler(
            memo, pdu, remote,
            (&mut payload_ptr) as *mut *const u8 as *mut c_void,
            (&mut payload_len) as *mut usize as *mut c_void,
            (&mut context) as *mut *const c_void as *mut c_void);
    }

    let payload = if payload_len > 0 {
        u8_slice_from(payload_ptr, payload_len).to_vec()
    } else {
        assert_eq!(payload_ptr, core::ptr::null_mut());
        vec![]
    };
    //crate::println!("xbd_resp_handler(): --------\n  context: {:?}\n  payload: {:?}", context, payload);

    add_xbd_gcoap_get_callback(
        Box::into_raw(Box::new((context /* cb_ptr */, payload))) as *const c_void); // arg_ptr
}