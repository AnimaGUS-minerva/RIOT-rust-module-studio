use conquer_once::spin::OnceCell;
use core::{pin::Pin, task::{Context, Poll}};
use crossbeam_queue::ArrayQueue;
use futures_util::{stream::Stream, task::AtomicWaker};

pub struct XbdStream<T: 'static> {
    queue: &'static OnceCell<ArrayQueue<T>>,
    waker: &'static AtomicWaker,
}

const QUEUE_CAP_DEFAULT: usize = 100;

impl<T> XbdStream<T> {
    pub fn new(queue: &'static OnceCell<ArrayQueue<T>>, waker: &'static AtomicWaker) -> Self {
        queue.try_init_once(|| ArrayQueue::new(QUEUE_CAP_DEFAULT))
            .expect("XbdStream::new should only be called once");

        XbdStream { queue, waker }
    }
}

impl<T> Stream for XbdStream<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let queue = self.queue
            .try_get()
            .expect("queue not initialized");

        // fast path
        if let Some(arg_ptr) = queue.pop() {
            return Poll::Ready(Some(arg_ptr));
        }

        self.waker.register(&cx.waker());
        match queue.pop() {
            Some(arg_ptr) => {
                self.waker.take();
                Poll::Ready(Some(arg_ptr))
            }
            None => Poll::Pending,
        }
    }
}