#![no_std]
//#![feature(alloc_error_handler)]//@@ *** disabled, due to conflict with futures-channel

//@@ ***
// #[panic_handler]
// fn panic(info: &core::panic::PanicInfo) -> ! { mcu_if::panic(info) }
//
// #[alloc_error_handler]
// fn alloc_error(layout: mcu_if::alloc::alloc::Layout) -> ! { mcu_if::alloc_error(layout) }

use core::cell::{Cell, RefCell};
use core::future::Future;
use mcu_if::{println, alloc::rc::Rc};

#[no_mangle]
pub extern fn rustmod_start() {
    println!("[src/lib.rs] rustmod_start(): ^^");
    rustmod_tests();
}

fn rustmod_tests() {
    println!("@@ rustmod_tests(): ^^");

    //----@@ https://github.com/smol-rs/async-task/blob/9ff587ecab7b9a9fa81672f4dbf315ff375b6e5e/examples/spawn-local.rs#L51
    let val = Rc::new(Cell::new(0));

    // Run a future that increments a non-`Send` value.
    run({
        let val = val.clone();
        async move {
            println!("@@ future1: ^^");
            // Spawn a future that increments the value.
            /*let task = spawn({
                let val = val.clone();
                async move {
                    val.set(dbg!(val.get()) + 1);
                }
            });*/

            val.set(val.get() + 1);
            //task.await;
            println!("@@ future1: $$");
        }
    });

    // The value should be 2 at the end of the program.
    val.get();
    //----@@
}

//

use async_task::{Runnable, Task};
use futures_channel::mpsc::{UnboundedSender,UnboundedReceiver};

// FIXME !! for `ee`
/*
error[E0463]: can't find crate for `std`
  |
  = note: the `xtensa-esp32-none-elf` target may not support the standard library
  = note: `std` is required by `futures_core` because it does not declare `#![no_std]`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`

For more information about this error, try `rustc --explain E0463`.
error: could not compile `futures-core` due to previous error
 */

/// Spawns a future on the executor.
//@@fn spawn<F, T>(future: F) -> Task<T>
fn spawn<F, T>(queue: Rc<RefCell<(UnboundedSender<Runnable>, UnboundedReceiver<Runnable>)>>, future: F) -> Task<T>
where
    F: Future<Output = T> + 'static,
    T: 'static,
{
    println!("@@ spawn(): ^^");
    // Create a task that is scheduled by pushing itself into the queue.
    //let schedule = |runnable| unsafe { QUEUE.0.unbounded_send(runnable).unwrap() };//@@
    //let (runnable, task) = async_task::spawn_local(future, schedule);
    //==== @@
    let schedule = |runnable| queue.borrow().0.unbounded_send(runnable).unwrap();
    let (runnable, task) = unsafe { async_task::spawn_unchecked(future, schedule) };//@@ for no_std

    // Schedule the task by pushing it into the queue.
    runnable.schedule();

    task
}

/// Runs a future to completion.
fn run<F, T>(future: F) -> T
where
    F: Future<Output = T> + 'static,
    T: 'static,
{
    let queue = Rc::new(RefCell::new(futures_channel::mpsc::unbounded::<Runnable>()));

    // Spawn a task that sends its result through a channel.
    //@@let (s, r) = flume::unbounded();
    let (s, mut r) = futures_channel::mpsc::unbounded();
    //@@spawn(async move { drop(s.send(future.await)) }).detach();
    spawn(queue.clone(), async move {
        println!("@@ future2: ^^");
        //@@drop(s.unbounded_send(future.await))
        drop(s.unbounded_send(future.await))//@@ ok
    }).detach();

    loop {
        println!("@@ loop: ^^");
        // If the original task has completed, return its result.
        //@@if let Ok(val) = r.try_recv() {
        if let Ok(val) = r.try_next() {
            //@@return val;
            return val.unwrap();
        }

        // Otherwise, take a task from the queue and run it. (@@ invokes "future2" above)
        //QUEUE.with(|(_, r)| r.recv().unwrap().run());
        //====
        println!("@@ loop: --");
        queue.borrow_mut().1.try_next().unwrap().unwrap().run();
        println!("@@ loop: $$");
    }
}