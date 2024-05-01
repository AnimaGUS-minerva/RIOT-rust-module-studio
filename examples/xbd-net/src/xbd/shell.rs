use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use futures_util::{stream::StreamExt, task::AtomicWaker};
use super::stream::XbdStream;

extern "C" {
    fn init_native_async_read() -> i8;
}

static SERVER_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static SERVER_WAKER: AtomicWaker = AtomicWaker::new();

pub async fn process_shell_stream() -> Result<(), i8> {
    let ret = unsafe { init_native_async_read() };
    if ret != 0 { return Err(ret); }

    let mut stream = XbdStream::new(&SERVER_QUEUE, &SERVER_WAKER);

    while let Some(cb) = stream.next().await {
        if 0 == 1 { crate::Xbd::async_sleep(1_000).await; } // debug, ok

        match cb {
            _ => (),
        }
    }

    Ok(())
}