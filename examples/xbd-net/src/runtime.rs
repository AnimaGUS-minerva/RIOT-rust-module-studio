use core::future::Future;
use super::blogos12::{
    example_task as blogos12_example_task,
    keyboard::print_keypresses as process_blogos12_scancodes,
    executor::Executor,
};
use super::xbd::{Xbd, SleepFnPtr, SetTimeoutFnPtr, GcoapReqSendFnPtr, process_xbd_callbacks};

pub struct Runtime(Executor);

impl Runtime {
    pub fn new() -> Result<Self, ()> {
        // !! todo refactor
        //xbd::init_once(xbd_usleep, xbd_ztimer_msleep, xbd_ztimer_set, xbd_gcoap_req_send);

        let mut ex = Executor::new();
        ex.spawn(blogos12_example_task()) // debug
            .spawn(process_blogos12_scancodes()) // processor, debug
            .spawn(process_xbd_callbacks()); // processor

        Ok(Self(ex))
    }

    pub fn block_on<F: Future<Output = ()> + 'static>(&mut self, future: F) -> F::Output {
        self.0.spawn(future).run();
    }
}