#![no_std]
#![feature(alloc_error_handler)]
#![feature(stmt_expr_attributes)]

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! { mcu_if::panic(info) }

#[alloc_error_handler]
fn alloc_error(layout: mcu_if::alloc::alloc::Layout) -> ! { mcu_if::alloc_error(layout) }

use core::cell::{Cell, RefCell};
use core::future::Future;
use mcu_if::{println, alloc::rc::Rc};
use async_task::{Runnable, Task};
use crossbeam_queue::ArrayQueue;

#[no_mangle]
pub extern fn rustmod_start() {
    println!("[src/lib.rs] rustmod_start(): ^^");

    rustmod_tests_blogos12();
    //rustmod_tests();
}

//

mod blogos12;

use blogos12::{example_task, keyboard, simple_executor::SimpleExecutor};

fn rustmod_tests_blogos12() {
    println!("@@ rustmod_tests_blogos12(): ^^");

    //

    let mut executor = SimpleExecutor::new();
    executor.spawn(blogos12::Task::new(example_task())); // ok
    executor.spawn(blogos12::Task::new(keyboard::print_keypresses())); // ok, CPU busy without Waker support
    executor.run();

    // let mut executor = Executor::new();
    // executor.spawn(Task::new(example_task()));
    // executor.spawn(Task::new(keyboard::print_keypresses()));
    // executor.run();

    let rt = Rc::new(Runtime::new());
    let rtc = rt.clone();
    rt.spawn_local(async move {
        rtc.exec(example_task()).await; // ok
        println!("@@ rustmod_tests_blogos12(): ----");
        if 0 == 1 { rtc.exec(keyboard::print_keypresses()).await; } // TODO async stream support in Runtime
    });
}

//

async fn inc(val: Rc<Cell<u8>>) -> Result<u8, ()>{
    println!("@@ inc(): ^^ val: {}", val.get());
    val.set(val.get() + 1);
    if 0 == 1 { loop {} } // debug

    Ok(val.get())
}

fn rustmod_tests() {
    println!("@@ rustmod_tests(): ^^");

    let val = Rc::new(Cell::new(0));
    println!("@@ rustmod_tests(): val: {}", val.get());

    let rt = Rc::new(Runtime::new());
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

    println!("@@ rustmod_tests(): $$ val: {}", val.get());
}

//----@@ adaptation of https://github.com/smol-rs/async-task/blob/9ff587ecab7b9a9fa81672f4dbf315ff375b6e5e/examples/spawn-local.rs#L51
const RUNTIME_CAP_DEFAULT: usize = 16;
struct Runtime(Rc<RefCell<ArrayQueue<Runnable>>>);
impl Runtime {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(
            crossbeam_queue::ArrayQueue::<Runnable>::new(RUNTIME_CAP_DEFAULT))))
    }

    /// Spawns a future on the executor.
    pub fn exec<F, T>(&self, future: F) -> Task<T>
    where
        F: Future<Output = T> + 'static,
        T: 'static,
    {
        // Create a task that is scheduled by pushing itself into the queue.
        let schedule = |runnable| self.0.borrow().push(runnable).unwrap();
        let (runnable, task) = unsafe { async_task::spawn_unchecked(future, schedule) };

        // Schedule the task by pushing it into the queue.
        runnable.schedule();

        task
    }

    pub fn spawn_local<F, T>(&self, future: F) -> T
    where
        F: Future<Output = T> + 'static,
        T: 'static,
    {
        // Spawn a task that sends its result through a channel.
        let oneshot = Rc::new(RefCell::new(crossbeam_queue::ArrayQueue::new(1)));
        let oneshotc = oneshot.clone();
        self.exec(async move {
            println!("@@ future-spawn-local: ^^");
            drop(oneshot.borrow().push(future.await))
        }).detach();

        loop {
            println!("@@ loop: ^^");
            // If the original task has completed, return its result.
            if let Some(val) = oneshotc.borrow().pop() {
                return val;
            }
            println!("@@ loop: --");

            // Otherwise, take a task from the queue and run it.
            self.0.borrow().pop().unwrap().run();
            println!("@@ loop: $$");
        }
    }
}