use mcu_if::alloc::boxed::Box;
use super::xbd::{self, Xbd};

mod executor;
use executor::Executor;

#[embassy_executor::task]
async fn task_xbd_main() {
    super::xbd_main().await.unwrap();

    //loop { Xbd::async_sleep(1000).await; } // yield -> executor busy
    loop { Xbd::msleep(1000, true); } // not yield (debug only) -> executor not busy
}

#[embassy_executor::task]
async fn task_api_stream() {
    xbd::process_api_stream().await.unwrap();
}

#[embassy_executor::task]
async fn task_gcoap_server_stream() {
    xbd::process_gcoap_server_stream().await.unwrap();
}

pub struct Runtime(&'static mut Executor);

impl Runtime {
    pub fn new_static() -> Result<&'static mut Self, ()> {
        Ok(Self::get_static(Self::new()))
    }

    fn new() -> Self {
        Self(Self::get_static(Executor::new()))
    }

    fn get_static<T>(x: T) -> &'static mut T {
        Box::leak(Box::new(x))
    }

    pub fn run(&'static mut self) -> ! {
        self.0.run(|spawner| {
            spawner.spawn(task_xbd_main()).unwrap();
            spawner.spawn(task_gcoap_server_stream()).unwrap();
            spawner.spawn(task_api_stream()).unwrap();
        });
    }
}
