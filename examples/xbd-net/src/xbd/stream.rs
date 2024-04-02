use conquer_once::spin::OnceCell;
use core::{pin::Pin, task::{Context, Poll}};
use crossbeam_queue::ArrayQueue;
use futures_util::{stream::Stream, task::AtomicWaker};

#[derive(Debug)]
pub struct XbdStream<T: 'static> {
    queue: &'static OnceCell<ArrayQueue<T>>,
    waker: &'static AtomicWaker,
}

const QUEUE_CAP_DEFAULT: usize = 100;

impl<T> XbdStream<T> {
    pub fn new(queue: &'static OnceCell<ArrayQueue<T>>, waker: &'static AtomicWaker) -> Self {
        Self::new_with_cap(queue, waker, QUEUE_CAP_DEFAULT)
    }

    pub fn new_with_cap(queue: &'static OnceCell<ArrayQueue<T>>, waker: &'static AtomicWaker, cap: usize) -> Self {
        queue.try_init_once(|| ArrayQueue::new(cap))
            .expect("XbdStream::new should only be called once");

        XbdStream { queue, waker }
    }

    pub fn get(queue: &'static OnceCell<ArrayQueue<T>>, waker: &'static AtomicWaker) -> Option<Self> {
        if queue.get().is_some() { // already init_once
            Some(XbdStream { queue, waker })
        } else {
            None
        }
    }

    // must not block/alloc/dealloc
    fn _add(queue: &'static OnceCell<ArrayQueue<T>>, waker: &'static AtomicWaker, item: T) {
        if let Ok(queue) = queue.try_get() {
            if let Err(_) = queue.push(item) {
                panic!("queue full");
            } else {
                waker.wake();
            }
        } else {
            panic!("queue uninitialized");
        }
    }

    pub fn add(&self, item: T) {
        Self::_add(self.queue, self.waker, item);
    }

    pub fn empty(&self) {
        let queue = self.queue.try_get().unwrap();
        while queue.len() > 0 {
            queue.pop();
        }
    }

    pub fn len(&self) -> usize {
        self.queue.try_get().unwrap().len()
    }
}

impl<T> Stream for XbdStream<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let queue = self.queue
            .try_get()
            .expect("queue not initialized");

        // fast path
        if let Some(item) = queue.pop() {
            return Poll::Ready(Some(item));
        }

        self.waker.register(&cx.waker());
        match queue.pop() {
            Some(item) => {
                self.waker.take();
                Poll::Ready(Some(item))
            }
            None => Poll::Pending,
        }
    }
}