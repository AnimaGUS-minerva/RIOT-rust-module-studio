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