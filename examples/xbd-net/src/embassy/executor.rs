use mcu_if::{println, alloc::boxed::Box};
use embassy_executor::{Spawner, raw::Executor as RawExecutor};

// https://github.com/embassy-rs/embassy/blob/b6fc682117a41e8e63a9632e06da5a17f46d9ab0/embassy-executor/src/raw/mod.rs#L465
#[export_name = "__pender"]
fn pender(context: *mut ()) {
    let signaler: &'static Signaler = unsafe { core::mem::transmute(context) };
    if 0 == 1 { println!("@@ pender(): signaler: {:?}", signaler); }
}

pub struct Executor {
    executor: RawExecutor,
    _signaler: &'static Signaler, // c.f. embassy/embassy-executor/src/arch/std.rs
}

#[derive(Debug)]
struct Signaler(u8); // TODO

impl Executor {
    pub fn new() -> Self {
        let signaler = Box::leak(Box::new(Signaler(42)));

        Self {
            executor: RawExecutor::new(signaler as *mut _ as *mut ()),
            _signaler: signaler,
        }
    }

    pub fn run(&'static mut self, init: impl FnOnce(Spawner)) -> ! {
        init(self.executor.spawner());

        loop {
            unsafe { self.executor.poll() };
        }
    }
}