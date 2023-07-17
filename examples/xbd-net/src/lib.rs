#![no_std]
#![feature(alloc_error_handler)]

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
    rustmod_tests();
}

fn rustmod_tests() {
    println!("@@ rustmod_tests(): ^^");

    //let queue = Rc::new(RefCell::new(futures_channel::mpsc::unbounded::<Runnable>()));
    let queue = Rc::new(RefCell::new(crossbeam_queue::ArrayQueue::<Runnable>::new(99)));

    //----@@ adaptation of https://github.com/smol-rs/async-task/blob/9ff587ecab7b9a9fa81672f4dbf315ff375b6e5e/examples/spawn-local.rs#L51
    let val = Rc::new(Cell::new(0));
    println!("@@ rustmod_tests(): val: {}", val.get());

    // Run a future that increments a non-`Send` value.
    run(queue.clone(), {
        let val = val.clone();
        async move {
            println!("@@ future1: ^^ val: {}", val.get());

            // Spawn a future that increments the value.
            let task = spawn(queue, {
                let val = val.clone();
                async move {
                    println!("@@ future2: ^^ val: {}", val.get());
                    val.set(val.get() + 1);
                    println!("@@ future2: $$ val: {}", val.get());
                }
            });

            val.set(val.get() + 1);
            task.await;

            println!("@@ future1: $$ val: {}", val.get());
        }
    });

    // The value should be 2 at the end of the program.
    println!("@@ rustmod_tests(): $$ val: {}", val.get());
    //----@@
}

/// Spawns a future on the executor.
fn spawn<F, T>(queue: Rc<RefCell<ArrayQueue<Runnable>>>, future: F) -> Task<T>
where
    F: Future<Output = T> + 'static,
    T: 'static,
{
    println!("@@ spawn(): ^^");
    // Create a task that is scheduled by pushing itself into the queue.
    let schedule = |runnable| queue.borrow().push(runnable).unwrap();
    let (runnable, task) = unsafe { async_task::spawn_unchecked(future, schedule) };

    // Schedule the task by pushing it into the queue.
    runnable.schedule();

    task
}

/// Runs a future to completion.
fn run<F, T>(queue: Rc<RefCell<ArrayQueue<Runnable>>>, future: F) -> T
where
    F: Future<Output = T> + 'static,
    T: 'static,
{
    println!("@@ run(): ^^");

    // Spawn a task that sends its result through a channel.
    let oneshot = Rc::new(RefCell::new(crossbeam_queue::ArrayQueue::new(1)));
    let oneshot_cloned = oneshot.clone();
    spawn(queue.clone(), async move {
        println!("@@ future-run: ^^");
        drop(oneshot.borrow().push(future.await))
    }).detach();

    loop {
        println!("@@ loop: ^^");
        // If the original task has completed, return its result.
        //@@if let Ok(val) = r.try_recv() {
        if let Some(val) = oneshot_cloned.borrow().pop() {
            return val;
        }
        println!("@@ loop: --");

        // Otherwise, take a task from the queue and run it. (@@ invokes "future2" above)
        //@@QUEUE.with(|(_, r)| r.recv().unwrap().run());
        queue.borrow().pop().unwrap().run();
        println!("@@ loop: $$");
    }
}