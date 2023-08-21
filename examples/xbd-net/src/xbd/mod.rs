mod callbacks;
pub use callbacks::process_timeout_callbacks;

mod timeout;
use timeout::Timeout;

use core::future::Future;
use conquer_once::spin::OnceCell;
use mcu_if::{alloc::{boxed::Box}, c_types::c_void};

pub type SleepFnPtr = unsafe extern "C" fn(u32);
pub type SetTimeoutFnPtr = unsafe extern "C" fn(u32, *const c_void, *mut (*const c_void, *mut *const c_void), *mut *const c_void);

static XBD_CELL: OnceCell<Xbd> = OnceCell::uninit();

pub fn init_once(
    xbd_usleep: SleepFnPtr,
    xbd_ztimer_msleep: SleepFnPtr,
    xbd_ztimer_set: SetTimeoutFnPtr
) {
    XBD_CELL
        .try_init_once(|| Xbd::_new(xbd_usleep, xbd_ztimer_msleep, xbd_ztimer_set))
        .expect("init_xbd() should only be called once");
}

pub struct Xbd {
    _usleep: SleepFnPtr,
    _ztimer_msleep: SleepFnPtr,
    _ztimer_set: SetTimeoutFnPtr,
}

impl Xbd {
    fn _new(
        xbd_usleep: SleepFnPtr,
        xbd_ztimer_msleep: SleepFnPtr,
        xbd_ztimer_set: SetTimeoutFnPtr
    ) -> Self {
        Self {
            _usleep: xbd_usleep,
            _ztimer_msleep: xbd_ztimer_msleep,
            _ztimer_set: xbd_ztimer_set,
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

    //

    pub fn async_sleep(msec: u32) -> impl Future<Output = ()> + 'static {
        Timeout::new(msec, None)
    }

    pub fn async_set_timeout<F>(msec: u32, cb: F) -> impl Future<Output = ()> + 'static where F: FnOnce() + 'static {
        Timeout::new(msec, Some(Box::new(cb)))
    }
}