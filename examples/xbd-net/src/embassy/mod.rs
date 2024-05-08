use mcu_if::alloc::boxed::Box;
use super::xbd::{self, Xbd};

mod executor;
use executor::Executor;

#[embassy_executor::task]
async fn task_xbd_main(throttle: u32) {
    super::xbd_main().await.unwrap();

    //loop { Xbd::async_sleep(1000).await; } // yield -> executor busy
    //====
    //loop { Xbd::msleep(1000, true); } // not yield (debug for internal async API calls only) -> executor not busy
    //==== kludge
    loop { Xbd::async_sleep(1).await;  Xbd::msleep(throttle, false); } // yield && less busy
}

#[embassy_executor::task]
async fn task_api_stream() {
    xbd::process_api_stream().await.unwrap();
}

#[embassy_executor::task]
async fn task_gcoap_server_stream() {
    xbd::process_gcoap_server_stream().await.unwrap();
}

#[embassy_executor::task]
async fn task_shell_stream() {
    xbd::process_shell_stream().await.unwrap();
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
        let throttle = 500;
        crate::println!("@@ task_xbd_main(): throttle: {} ms", throttle);

        self.0.run(|spawner| {
            spawner.spawn(task_xbd_main(throttle)).unwrap();
            spawner.spawn(task_shell_stream()).unwrap();
            spawner.spawn(task_gcoap_server_stream()).unwrap();
            spawner.spawn(task_api_stream()).unwrap();
        });
    }
}
