use mcu_if::println;

pub mod keyboard;

async fn async_number() -> u32 {
    42
}

pub async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}