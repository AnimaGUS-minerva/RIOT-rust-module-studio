use futures_util::stream::StreamExt;
use super::stream::{XbdStream, StreamData, stream_uninit};

extern "C" {
    fn xbd_shell_init() -> i8;
}

#[no_mangle]
pub extern fn xbd_shell_on_read_line(/* TODO */) {
    crate::println!("@@ xbd_shell_on_read_line(): ^^");
}

static SD: StreamData<u8> = stream_uninit();

pub async fn process_shell_stream() -> Result<(), i8> {
    // TODO conditional !! native only for now
    let ret = unsafe { xbd_shell_init() };
    if ret != 0 { return Err(ret); }

    let mut stream = XbdStream::new(&SD);

    while let Some(cb) = stream.next().await {
        if 0 == 1 { crate::Xbd::async_sleep(1_000).await; } // debug, ok

        match cb {
            _ => (),
        }
    }

    Ok(())
}