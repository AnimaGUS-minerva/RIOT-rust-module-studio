use super::stream::{XbdStream, StreamData, stream_uninit, StreamExt};

extern "C" {
    fn xbd_shell_init() -> i8;
}

#[no_mangle]
pub extern fn xbd_shell_on_read_line(/* TODO */) {
    crate::println!("@@ xbd_shell_on_read_line(): ^^");
}

static SD: StreamData<u8> = stream_uninit();

pub async fn process_shell_stream() -> Result<(), i8> {
    let ret = unsafe { xbd_shell_init() };
    match ret {
        0 => (), // ok, continue
        2 => { // kludge
            crate::println!("@@ process_shell_stream(): TODO - support non-native board");
            return Ok(());
        },
        _ => return Err(ret),
    }

    let mut stream = XbdStream::new(&SD);
    while let Some(cb) = stream.next().await {
        if 0 == 1 { crate::Xbd::async_sleep(1_000).await; } // debug, ok

        match cb {
            _ => (),
        }
    }

    Ok(())
}