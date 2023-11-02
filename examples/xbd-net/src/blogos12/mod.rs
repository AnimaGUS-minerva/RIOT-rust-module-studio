pub mod keyboard;
pub mod simple_executor;
pub mod executor;

pub mod task;
pub use task::{Task, TaskId};

//

async fn async_number() -> u32 { 42 }

pub async fn example_task() {
    let number = async_number().await;
    mcu_if::println!("async number: {}", number);
}

//

use executor::Executor;
use core::future::Future;
use super::xbd::process_xbd_callbacks;

pub struct Runtime(Executor);

impl Runtime {
    pub fn new() -> Result<Self, ()> {
        let mut ex = Executor::new();
        ex
            //.spawn(example_task()) // debug
            //.spawn(keyboard::print_keypresses()) // processor, debug
            .spawn(process_xbd_callbacks()); // processor

        Ok(Self(ex))
    }

    // c.f. https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html#method.block_on
    pub fn block_on<F: Future<Output = ()> + 'static>(&mut self, future: F) -> F::Output {
        self.0.spawn(future).run();
    }
}