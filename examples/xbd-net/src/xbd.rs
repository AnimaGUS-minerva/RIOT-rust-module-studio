use mcu_if::{alloc::boxed::Box, c_types::c_void};

pub type SleepFnPtr = unsafe extern "C" fn(u32);
pub type SetTimeoutFnPtr = unsafe extern "C" fn(u32, *const c_void, *const c_void);

pub struct Xbd {
    _usleep: SleepFnPtr,
    _ztimer_msleep: SleepFnPtr,
    _ztimer_set: SetTimeoutFnPtr,
}

impl Xbd {
    pub fn new(
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

    pub fn usleep(&self, usec: u32) {
        unsafe { (self._usleep)(usec); }
    }

    pub fn msleep(&self, msec: u32) {
        unsafe { (self._ztimer_msleep)(msec); }
    }

    pub fn set_timeout<F>(&self, msec: u32, cb: F) where F: FnOnce() + 'static {
        let cb: Box<Box<dyn FnOnce()>> = Box::new(Box::new(cb));
        let cb_ptr = Box::into_raw(cb) as *const _;

        unsafe { (self._ztimer_set)(msec, Self::cb_handler as *const c_void, cb_ptr); }
    }

    fn cb_handler(cb_ptr: *const c_void) {
        let cb: Box<Box<dyn FnOnce()>> = unsafe { Box::from_raw(cb_ptr as *mut _) };

        (*cb)(); // call, move, drop
    }
}