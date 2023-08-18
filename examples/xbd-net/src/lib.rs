#![no_std]
#![feature(alloc_error_handler)]
#![feature(stmt_expr_attributes)]

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! { mcu_if::panic(info) }

#[alloc_error_handler]
fn alloc_error(layout: mcu_if::alloc::alloc::Layout) -> ! { mcu_if::alloc_error(layout) }

use core::cell::Cell;
use mcu_if::{println, alloc::{rc::Rc, boxed::Box}};
use xbd::{Xbd, SleepFnPtr, SetTimeoutFnPtr};

mod xbd;
mod runtime;
mod blogos12;

//

#[no_mangle]
pub extern fn rustmod_start(
    xbd_usleep: SleepFnPtr,
    xbd_ztimer_msleep: SleepFnPtr,
    xbd_ztimer_set: SetTimeoutFnPtr
) {
    println!("[src/lib.rs] rustmod_start(): ^^");

    if 100 == 1 { loop { unsafe { xbd_usleep(500_000); } } } // ok

    let xbd = Rc::new(Xbd::new(xbd_usleep, xbd_ztimer_msleep, xbd_ztimer_set));

    if 100 == 1 { loop { xbd.usleep(500_000); } } // ok
    if 100 == 1 { loop { xbd.msleep(500); } } // ok

    if 1 == 1 { rustmod_test_blogos12(xbd.clone()); }
    if 100 == 1 { rustmod_test_runtime(); }
}

//

fn rustmod_test_blogos12(xbd: Rc<Xbd>) {
    println!("@@ rustmod_test_blogos12(): ^^");

    //

    async fn async_number() -> u32 { 42 }

    pub async fn example_task() {
        let number = async_number().await;
        println!("async number: {}", number);
    }

    use blogos12::keyboard::print_keypresses;

    //

    if 0 == 1 {
        use blogos12::simple_executor::SimpleExecutor;
        let mut executor = SimpleExecutor::new();
        executor.spawn(blogos12::Task::new(example_task())); // ok
        executor.spawn(blogos12::Task::new(print_keypresses())); // ok, CPU busy without Waker support
        executor.run();
    }

    //

    if 1 == 1 {
        //---- ok
        let foo = Box::new(9);
        xbd.set_timeout(2500, move || {
            println!("@@ super_closure(): ^^ foo: {:?}", foo);
            blogos12::keyboard::add_scancode(8);
            blogos12::keyboard::add_scancode(*foo);
            println!("@@ super_closure(): $$");
        });

        fn ff() { println!("@@ ff(): ^^"); }
        xbd.set_timeout(2500, ff);
        //----
        // TODOs
        //   - check rust coap code
        //   - async UDP serv/cli

        use blogos12::executor::Executor;
        let mut executor = Executor::new(xbd);
        executor.spawn(blogos12::Task::new(example_task())); // ok
        executor.spawn(blogos12::Task::new(xbd::process_timeout_callbacks()));
        executor.spawn(blogos12::Task::new(print_keypresses()));
        executor.run();
    }

    //

    let rt = Rc::new(runtime::Runtime::new());
    let rtc = rt.clone();
    rt.spawn_local(async move {
        rtc.exec(example_task()).await; // ok
        println!("@@ rustmod_test_blogos12(): ----");
        if 0 == 1 { rtc.exec(print_keypresses()).await; } // TODO async stream support in Runtime
    });

    println!("@@ rustmod_test_blogos12(): $$");
}

//


fn rustmod_test_runtime() {
    println!("@@ rustmod_test_runtime(): ^^");

    //

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