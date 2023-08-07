use core::future::Future;
use mcu_if::{println, alloc::rc::Rc};
use async_task::{Runnable, Task};
use crossbeam_queue::ArrayQueue;

const RUNTIME_CAP_DEFAULT: usize = 16;

pub struct Runtime(Rc<ArrayQueue<Runnable>>);

impl Runtime { // adaptation of https://github.com/smol-rs/async-task/blob/9ff587ecab7b9a9fa81672f4dbf315ff375b6e5e/examples/spawn-local.rs#L51
    pub fn new() -> Self {//@@ TODO !!!! use Arc (for the Waker trait compat) like 'blogos/executor.rs'
        Self(Rc::new(
            crossbeam_queue::ArrayQueue::<Runnable>::new(RUNTIME_CAP_DEFAULT)))
    }

    /// Spawns a future on the executor.
    pub fn exec<F, T>(&self, future: F) -> Task<T>
    where
        F: Future<Output = T> + 'static,
        T: 'static,
    {
        // Create a task that is scheduled by pushing itself into the queue.
        let schedule = |runnable| self.0.push(runnable).unwrap();
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
        let oneshot = Rc::new(crossbeam_queue::ArrayQueue::new(1));
        let oneshotc = oneshot.clone();
        self.exec(async move {
            println!("@@ future-spawn-local: ^^");
            drop(oneshot.push(future.await))
        }).detach();

        loop {
            println!("@@ loop: ^^");
            // If the original task has completed, return its result.
            if let Some(val) = oneshotc.pop() {
                return val;
            }
            println!("@@ loop: --");

            // Otherwise, take a task from the queue and run it.
            self.0.pop().unwrap().run();
            println!("@@ loop: $$");
        }
    }
}