use core::future::Future;
use super::xbd::process_xbd_callbacks;
use super::blogos12::{
    //example_task as blogos12_example_task,
    //keyboard::print_keypresses as process_blogos12_scancodes,
    executor::Executor,
};

pub struct Runtime(Executor);

impl Runtime {
    pub fn new() -> Result<Self, ()> {
        let mut ex = Executor::new();
        ex//.spawn(blogos12_example_task()) // debug
            //.spawn(process_blogos12_scancodes()) // processor, debug
            .spawn(process_xbd_callbacks()); // processor

        Ok(Self(ex))
    }

    pub fn block_on<F: Future<Output = ()> + 'static>(&mut self, future: F) -> F::Output {
        self.0.spawn(future).run();
    }
}