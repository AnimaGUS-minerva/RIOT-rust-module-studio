use embassy_executor::{Spawner, raw::Executor as RawExecutor};
use super::{to_raw, static_from_raw};

// https://github.com/embassy-rs/embassy/blob/b6fc682117a41e8e63a9632e06da5a17f46d9ab0/embassy-executor/src/raw/mod.rs#L465
#[export_name = "__pender"]
fn pender(context: *mut ()) {
    //@@ let signaler: &'static Signaler = unsafe { core::mem::transmute(context) };
    let signaler: &'static Signaler = static_from_raw(context);
    if 0 == 1 { crate::println!("@@ pender(): signaler: {:?}", signaler); }
}

pub struct Executor {
    executor: RawExecutor,
    //@@ _signaler: &'static Signaler, // c.f. embassy/embassy-executor/src/arch/std.rs
    _signaler: Signaler,
}

#[derive(Debug)]
struct Signaler(()); // TODO

impl Executor {
    pub fn new() -> Self {
        //@@ let signaler = Box::leak(Box::new(Signaler(())));
        let mut signaler = Signaler(());

        Self {
            executor: RawExecutor::new(to_raw(&mut signaler)),
            _signaler: signaler,
        }
    }

    pub fn run(&'static mut self, init: impl FnOnce(Spawner)) -> ! {
        init(self.executor.spawner());

        let throttle = 100;
        crate::println!("@@ Executor::run(): throttle: {} ms", throttle);

        loop {
            crate::Xbd::msleep(throttle, false);
            unsafe { self.executor.poll() };
        }
    }
}
