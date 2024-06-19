use super::xbd;

mod executor;
use executor::Executor;

#[embassy_executor::task]
async fn task_xbd_main() {
    super::xbd_main().await.unwrap();
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

pub struct Runtime(Executor);

impl Runtime {
    pub fn new() -> Self {
        Self(Executor::new(Some(100)))
    }

    pub fn run(&'static mut self) -> ! {
        self.0.run(|spawner| {
            spawner.spawn(task_xbd_main()).unwrap();
            spawner.spawn(task_shell_stream()).unwrap();
            spawner.spawn(task_gcoap_server_stream()).unwrap();
            spawner.spawn(task_api_stream()).unwrap();
        });
    }
}

//
// dummy `critical_section` cf. 'rust-riot-wrappers/src/impl_critical_section.rs'
//

use critical_section::RawRestoreState;

#[allow(dead_code)]// @@
struct CriticalSection(usize);
critical_section::set_impl!(CriticalSection);

unsafe impl critical_section::Impl for CriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        // If this fails to compile (because Rust-on-RIOT has gained support for non-32bit
        // architectures), by that time hopefully critical-section > 1.1.2 has been released, which
        // has restore-state-usize. Just increment the dependency version and set its feature from
        // restore-state-u32 to restore-state-usize.
//@@        unsafe { riot_sys::irq_disable() }
    }

    #[allow(unused_variables)]// @@
    unsafe fn release(token: RawRestoreState) {
//@@        unsafe { riot_sys::irq_restore(token) };
    }
}