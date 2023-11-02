pub mod keyboard;
pub mod simple_executor;
pub mod executor;

pub mod task;
pub use task::{Task, TaskId};

use mcu_if::println;

//

async fn async_number() -> u32 { 42 }

pub async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
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

//

pub fn test_misc() {
    println!("@@ test_misc(): ^^");

    use self::{
        example_task as blogos12_example_task,
        keyboard::print_keypresses as process_blogos12_scancodes,
        simple_executor::SimpleExecutor,
    };

    if 0 == 1 {
        let mut exe = SimpleExecutor::new();
        exe.spawn(Task::new(blogos12_example_task())); // ok
        exe.spawn(Task::new(process_blogos12_scancodes())); // ok, CPU busy without Waker support
        exe.run();
    }

    if 0 == 1 {
        Executor::new()
            .spawn(blogos12_example_task())
            .spawn(process_blogos12_scancodes()) // processor
            .spawn(process_xbd_callbacks()) // processor
            .spawn(async move { // main
                println!("@@ hello");
            })
            .run();
    }

    println!("@@ test_misc(): $$");
}