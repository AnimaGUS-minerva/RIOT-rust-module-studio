#![no_std]
#![feature(alloc_error_handler)]
#![feature(stmt_expr_attributes)]

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! { mcu_if::panic(info) }

#[alloc_error_handler]
fn alloc_error(layout: mcu_if::alloc::alloc::Layout) -> ! { mcu_if::alloc_error(layout) }

use core::cell::Cell;
use mcu_if::{println, alloc::boxed::Box, null_terminate_bytes};

mod runtime_old; // deprecated

mod runtime;
use runtime::Runtime;

mod xbd;
use xbd::{Xbd, XbdFnsEnt, process_xbd_callbacks};

mod blogos12;

//

#[no_mangle]
pub extern fn rustmod_start(
    xbd_fns_ptr: *const XbdFnsEnt,
    xbd_fns_sz: usize
) {
    println!("[src/lib.rs] rustmod_start(): ^^");

    xbd::init_once(xbd_fns_ptr, xbd_fns_sz);

    if 0 == 1 { // debug
        Xbd::usleep(2_000_000);
        Xbd::msleep(2_000, true);
        panic!("!!!! debug ok");
    }

    if 0 == 1 { rustmod_start_debug(); }

    // c.f. https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html#method.block_on
    Runtime::new().unwrap().block_on(async move {

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
            let cb = |payload| { println!("@@ payload: {:?}", payload); };

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
            let payload = Xbd::async_gcoap_get("[fe80::78ec:5fff:febd:aaaa]:5683", uri).await;
            println!("@@ payload: {:?}", payload);
            assert_eq!(payload.len(), 0);

            // test case invalid `uri`
            let payload = Xbd::async_gcoap_get(addr, "/.well-known/cccc").await;
            println!("@@ payload: {:?}", payload);
            assert_eq!(payload.len(), 0);

            // test hitting the internal server, native-only!!
            let payload = Xbd::async_gcoap_get(addr, uri).await;
            println!("@@ payload: {:?}", payload);
            assert_eq!(payload.len(), 46);

            // test hitting the external server
            let payload = Xbd::async_gcoap_get(req_external_native.0, req_external_native.1).await;
            println!("@@ payload: {:?}", payload);
            assert_eq!(payload.len(), 5);
        }
    });
    panic!("should be never reached");
}

fn rustmod_start_debug() {
    if 100 == 1 { rustmod_test_runtime_old_v1(); }
    if 100 == 1 { rustmod_test_runtime_old_v2(); }
    if 100 == 1 { rustmod_test_blogos12(); }
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

fn rustmod_test_runtime_old_v2() {
    use blogos12::{
        example_task as blogos12_example_task,
        keyboard::print_keypresses as process_blogos12_scancodes,
    };

    use mcu_if::alloc::rc::Rc; // !! temp !!
    let rt = Rc::new(runtime_old::Runtime::new());
    let rtc = rt.clone();
    rt.spawn_local(async move {
        rtc.exec(blogos12_example_task()).await; // ok
        println!("@@ rustmod_test_blogos12(): ----");
        if 0 == 1 { rtc.exec(process_blogos12_scancodes()).await; } // TODO async stream support in Runtime
    });
}

fn rustmod_test_runtime_old_v1() {
    println!("@@ rustmod_test_runtime_old_v1(): ^^");

    //

    use mcu_if::alloc::rc::Rc; // !! temp !!
    async fn inc(val: Rc<Cell<u8>>) -> Result<u8, ()>{
        println!("@@ inc(): ^^ val: {}", val.get());
        val.set(val.get() + 1);
        if 0 == 1 { loop {} } // debug

        Ok(val.get())
    }

    //

    let val = Rc::new(Cell::new(0));
    println!("@@ rustmod_test_runtime_old_v1(): val: {}", val.get());

    let rt = Rc::new(runtime_old::Runtime::new());
    {
        let val = val.clone();
        let rtc = rt.clone();
        rt.spawn_local(async move {
            println!("@@ future0: ^^ val: {}", val.get());

            //

            val.set(val.get() + 1);

            //

            let ret = rtc.exec(inc(val.clone())).await;
            println!("@@ ret: {:?}", ret);

            //

            rtc.exec({
                let val = val.clone();
                async move {
                    println!("@@ future1: ^^ val: {}", val.get());
                    val.set(val.get() + 1);
                    println!("@@ future1: $$ val: {}", val.get());
                }
            }).await;

            //

            println!("@@ future0: $$ val: {}", val.get());
        });
    }

    println!("@@ rustmod_test_runtime_old_v1(): val: {}", val.get());
    assert_eq!(val.get(), 3);

    println!("@@ rustmod_test_runtime_old_v1(): $$");
}