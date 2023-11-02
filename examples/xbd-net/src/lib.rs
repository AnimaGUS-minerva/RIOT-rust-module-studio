#![no_std]
#![feature(alloc_error_handler)]
#![feature(stmt_expr_attributes)]
#![feature(type_alias_impl_trait)]

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! { mcu_if::panic(info) }

#[alloc_error_handler]
fn alloc_error(layout: mcu_if::alloc::alloc::Layout) -> ! { mcu_if::alloc_error(layout) }

use mcu_if::{println, alloc::boxed::Box, null_terminate_bytes};

mod xbd;
use xbd::{Xbd, XbdFnsEnt, process_xbd_callbacks};

mod blogos12;

//

use embassy_executor::{Spawner, raw::Executor as RawExecutor};

// https://github.com/embassy-rs/embassy/blob/b6fc682117a41e8e63a9632e06da5a17f46d9ab0/embassy-executor/src/raw/mod.rs#L465
#[export_name = "__pender"]
fn pender(context: *mut ()) {
    let signaler: &'static Signaler = unsafe { core::mem::transmute(context) };
    if 0 == 1 { println!("@@ pender(): signaler: {:?}", signaler); }
}

pub struct EmbassyExecutor {
    executor: RawExecutor,
    _signaler: &'static Signaler, // c.f. embassy/embassy-executor/src/arch/std.rs
}

#[derive(Debug)]
struct Signaler(u8); // TODO

impl EmbassyExecutor {
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

#[embassy_executor::task]
async fn task_xbd_main() {
    Xbd::async_set_timeout(999, || { println!("!!!!---- async APIs"); }).await;

    let req_internal_native = ("[fe80::78ec:5fff:febd:add9]:5683", "/.well-known/core");
    let (addr, uri) = req_internal_native;
    let out = Xbd::async_gcoap_get(addr, uri).await;
    println!("@@ out: {:?}", out);

    //

    //loop { Xbd::async_sleep(1000).await; } // yield -> executor busy
    loop { Xbd::msleep(1000, true); } // not yield (debug only) -> executor not busy
}

#[embassy_executor::task]
async fn task_xbd_callbacks() {
    process_xbd_callbacks().await;
}

pub struct EmbassyRuntime(&'static mut EmbassyExecutor);

impl EmbassyRuntime {
    pub fn new_static() -> Result<&'static mut Self, ()> {
        Ok(Self::get_static(Self::new()))
    }

    fn new() -> Self {
        Self(Self::get_static(EmbassyExecutor::new()))
    }

    fn get_static<T>(x: T) -> &'static mut T {
        Box::leak(Box::new(x))
    }

    pub fn run(&'static mut self) -> ! {
        self.0.run(|spawner| {
            spawner.spawn(task_xbd_main()).unwrap();
            spawner.spawn(task_xbd_callbacks()).unwrap();
        });
    }
}

//

#[no_mangle]
pub extern fn rustmod_start(
    xbd_fns_ptr: *const XbdFnsEnt,
    xbd_fns_sz: usize
) {
    println!("[src/lib.rs] rustmod_start(): ^^");

    xbd::init_once(xbd_fns_ptr, xbd_fns_sz);

    if 0 == 1 { // debug
        Xbd::usleep(1_000_000);
        rustmod_test_blogos12();
        return;
    }

    if 100 == 1 { // !!!!
        let rt = EmbassyRuntime::new_static().unwrap();
        rt.run();
    }

    blogos12::Runtime::new().unwrap().block_on(async move {

        if 0 == 1 { // non-blocking, ok
            use blogos12::keyboard::add_scancode as blogos12_add_scancode;

            let foo = Box::new(9);
            Xbd::set_timeout(2500, move |_| {
                println!("@@ ||aa: ^^ foo: {:?}", foo);
                blogos12_add_scancode(8);
                blogos12_add_scancode(*foo);
            });

            fn ff(_: ()) { println!("@@ ff(): ^^"); }
            Xbd::set_timeout(2500, ff);
        }

        if 0 == 1 { // async, ok
            Xbd::async_sleep(3500).await; // ok
            Xbd::async_set_timeout(3500, || { println!("@@ ||x: ^^"); }).await; // ok
        }

        //

        let req_internal_native = ("[fe80::78ec:5fff:febd:add9]:5683", "/.well-known/core");
        let req_external_native = ("[fe80::20be:cdff:fe0e:44a1]:5683", "/hello");

        if 0 == 1 { // non-blocking, ok
            let cb = |out| { println!("@@ out: {:?}", out); };

            //==== native, internal server
            let (addr, uri) = req_internal_native;
            Xbd::gcoap_get(addr, uri, cb);

            //==== native, external server -- LD_LIBRARY_PATH=./libcoap/local/lib libcoap-minimal/server 5683 fe80::20be:cdff:fe0e:44a1%tap1 &
            let (addr, uri) = req_external_native;
            Xbd::gcoap_get(addr, uri, cb);
        }

        if 1 == 1 { // async, ok
            Xbd::async_set_timeout(999, || { println!("!!!!---- async APIs"); }).await;
            let (addr, uri) = req_internal_native;

            // test case invalid `addr`
            let out = Xbd::async_gcoap_get("[fe80::78ec:5fff:febd:aaaa]:5683", uri).await;
            println!("@@ out: {:?}", out);

            // test case invalid `uri`
            let out = Xbd::async_gcoap_get(addr, "/.well-known/cccc").await;
            println!("@@ out: {:?}", out);

            // test hitting the internal server, native-only!!
            let out = Xbd::async_gcoap_get(addr, uri).await;
            println!("@@ out: {:?}", out);

            // test hitting the external server
            let out = Xbd::async_gcoap_get(req_external_native.0, req_external_native.1).await;
            println!("@@ out: {:?}", out);
        }
    });
    panic!("should be never reached");
}

fn rustmod_test_blogos12() {
    println!("@@ rustmod_test_blogos12(): ^^");

    use blogos12::{
        Task,
        example_task as blogos12_example_task,
        keyboard::print_keypresses as process_blogos12_scancodes,
        simple_executor::SimpleExecutor,
        executor::Executor,
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

    println!("@@ rustmod_test_blogos12(): $$");
}