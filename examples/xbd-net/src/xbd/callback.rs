use mcu_if::{alloc::boxed::Box, c_types::c_void};
use super::gcoap::GcoapMemoState;
use super::stream::{XStream, XStreamData, StreamExt};

extern "C" {
    fn free(ptr: *mut c_void);
}

type CVoidPtr = *const c_void;
pub type Ptr32Send = u32;

enum ApiCallback {
    Timeout(Ptr32Send),
    _GcoapPing(Ptr32Send),
    GcoapReq(Ptr32Send),
}

static mut SD: XStreamData<ApiCallback, 64> = XStream::init();

fn add_api_callback(cb: ApiCallback) {
    XStream::get(static_borrow_mut!(SD)).add(cb);
}

pub fn add_xbd_timeout_callback(arg_ptr: CVoidPtr) {
    add_api_callback(ApiCallback::Timeout(arg_ptr as Ptr32Send));
}
pub fn add_xbd_gcoap_req_callback(arg_ptr: CVoidPtr) {
    add_api_callback(ApiCallback::GcoapReq(arg_ptr as Ptr32Send));
}

pub async fn process_api_stream() -> Result<(), i8> {
    let mut xs = XStream::get(static_borrow_mut!(SD));

    while let Some(cb) = xs.next().await {
        match cb {
            ApiCallback::Timeout(arg_ptr) => {
                let (cb_ptr, timeout_pp): (CVoidPtr, *mut CVoidPtr) = arg_from(arg_ptr);

                let timeout_ptr = unsafe { *Box::from_raw(timeout_pp) };
                assert_ne!(timeout_ptr, core::ptr::null());
                unsafe { free(timeout_ptr as *mut _); }// !!!!

                call(cb_ptr, ());
            },
            ApiCallback::_GcoapPing(_) => todo!(),
            ApiCallback::GcoapReq(arg_ptr) => {
                let (cb_ptr, out) = arg_from::<GcoapMemoState>(arg_ptr);
                call(cb_ptr, out);
            },
        }
    }

    Ok(())
}

//

pub fn into_raw<F, T>(cb: F) -> CVoidPtr where F: FnOnce(T) + 'static {
    let cb: Box<Box<dyn FnOnce(T) + 'static>> = Box::new(Box::new(cb));

    Box::into_raw(cb) as *const _
}

pub fn arg_from<T>(arg_ptr: Ptr32Send) -> (CVoidPtr, T) {
    unsafe { *Box::from_raw(arg_ptr as *mut _) }
}

fn cb_from<T>(cb_ptr: CVoidPtr) -> Box<Box<dyn FnOnce(T) + 'static>> {
    unsafe { Box::from_raw(cb_ptr as *mut _) }
}

fn call<T>(cb_ptr: CVoidPtr, out: T) {
    assert_ne!(cb_ptr, core::ptr::null());
    (*(cb_from(cb_ptr)))(out); // call, move, drop
}