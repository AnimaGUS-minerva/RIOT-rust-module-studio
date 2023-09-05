#![no_std]
#![feature(alloc_error_handler)]
#![feature(stmt_expr_attributes)]

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! { mcu_if::panic(info) }

#[alloc_error_handler]
fn alloc_error(layout: mcu_if::alloc::alloc::Layout) -> ! { mcu_if::alloc_error(layout) }

use core::cell::Cell;
use mcu_if::{println, alloc::boxed::Box, null_terminate_bytes};

mod xbd;
use xbd::{Xbd, SleepFnPtr, SetTimeoutFnPtr, GcoapReqSendFnPtr, process_timeout_callbacks};

mod runtime;
mod blogos12;

//

#[no_mangle]
pub extern fn rustmod_start(
    xbd_usleep: SleepFnPtr,
    xbd_ztimer_msleep: SleepFnPtr,
    xbd_ztimer_set: SetTimeoutFnPtr,
    xbd_gcoap_req_send: GcoapReqSendFnPtr
) {
    println!("[src/lib.rs] rustmod_start(): ^^");

    if 100 == 1 { loop { unsafe { xbd_usleep(500_000); } } } // ok

    //

    xbd::init_once(xbd_usleep, xbd_ztimer_msleep, xbd_ztimer_set, xbd_gcoap_req_send);

    if 100 == 1 { loop { Xbd::usleep(500_000); } } // ok
    if 100 == 1 { loop { Xbd::msleep(500); } } // ok
    if 1 == 1 { rustmod_test_blogos12(); }

    //

    if 100 == 1 { rustmod_test_runtime(); }
}

//

fn rustmod_test_blogos12() {
    println!("@@ rustmod_test_blogos12(): ^^");

    //

    use blogos12::{
        Task,
        example_task as blogos12_example_task,
        keyboard::print_keypresses as process_blogos12_scancodes,
        keyboard::add_scancode as blogos12_add_scancode,
        simple_executor::SimpleExecutor,
        executor::Executor,
    };

    //

    if 0 == 1 {
        let mut exe = SimpleExecutor::new();
        exe.spawn(Task::new(blogos12_example_task())); // ok
        exe.spawn(Task::new(process_blogos12_scancodes())); // ok, CPU busy without Waker support
        exe.run();
    }

    //

    if 1 == 1 {
        Executor::new()
            .spawn(blogos12_example_task())
            .spawn(process_timeout_callbacks()) // processor
            .spawn(process_blogos12_scancodes()) // processor
            .spawn(async move { // main

                let req_internal = ("[fe80::78ec:5fff:febd:add9]:5683", "/.well-known/core");
                let req_external = ("[fe80::20be:cdff:fe0e:44a1]:5683", "/hello");
                if 1 == 1 { // !!!! WIP
                    let cb = |x| { x * x }; // TODO
                    //==== native, internal server
                    let (addr, uri) = req_internal;
                    Xbd::gcoap_get(addr, uri, cb);
                    //==== native, external server -- LD_LIBRARY_PATH=./libcoap/local/lib libcoap-minimal/server 5683 fe80::20be:cdff:fe0e:44a1%tap1 &
                    let (addr, uri) = req_external;
                    Xbd::gcoap_get(addr, uri, cb);
                    //==== deprecated notes
/*
                    Xbd::async_gcoap_client_send("coap get [fe80::a00:27ff:fefd:b6f8]:5683 /hello", || {
                        static void _resp_handler(const gcoap_request_memo_t *memo, coap_pkt_t* pdu,
                                                  const sock_udp_ep_t *remote) {

                            printf("gcoap: response %s, code %1u.%02u", class_str,
                                    coap_get_code_class(pdu),
                                    coap_get_code_detail(pdu));
                  }); // !!!!
*/
                    //}).await; // !!!!
                }

                //---- async
                let (addr, uri) = req_internal;
                assert_eq!(Xbd::async_gcoap_get(addr, uri).await, 99);
                if 1==1 { panic!("!!!!!!!!"); }

                Xbd::async_sleep(3500).await; // ok
                Xbd::async_set_timeout(3500, || { println!("@@ ||x: ^^"); }).await; // ok
                //----

                //---- non-blocking, ok
                let foo = Box::new(9);
                Xbd::set_timeout(2500, move || {
                    println!("@@ ||aa: ^^ foo: {:?}", foo);
                    blogos12_add_scancode(8);
                    blogos12_add_scancode(*foo);
                });

                fn ff() { println!("@@ ff(): ^^"); }
                Xbd::set_timeout(2500, ff);
                //----

                // TODOs
                //   - check rust coap code
                //   - async UDP/CoAP serv/cli
            })
            .run();
    }

    //

    use mcu_if::alloc::rc::Rc; // !! temp !!
    let rt = Rc::new(runtime::Runtime::new());
    let rtc = rt.clone();
    rt.spawn_local(async move {
        rtc.exec(blogos12_example_task()).await; // ok
        println!("@@ rustmod_test_blogos12(): ----");
        if 0 == 1 { rtc.exec(process_blogos12_scancodes()).await; } // TODO async stream support in Runtime
    });

    println!("@@ rustmod_test_blogos12(): $$");
}

//


fn rustmod_test_runtime() {
    println!("@@ rustmod_test_runtime(): ^^");

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
    println!("@@ rustmod_test_runtime(): val: {}", val.get());

    let rt = Rc::new(runtime::Runtime::new());
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

    println!("@@ rustmod_test_runtime(): val: {}", val.get());
    assert_eq!(val.get(), 3);

    println!("@@ rustmod_test_runtime(): $$");
}