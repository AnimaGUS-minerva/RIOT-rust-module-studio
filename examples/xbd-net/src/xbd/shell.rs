use super::stream::{XbdStream, StreamData, stream_uninit, StreamExt};
use crate::println;

extern "C" {
    fn xbd_shell_init() -> i8;
    fn xbd_shell_bufsize() -> usize;
    fn xbd_shell_prompt();
}

const SHELL_BUFSIZE: usize = 128;
static mut SHELL_BUF: heapless::String<SHELL_BUFSIZE> = heapless::String::new();

#[no_mangle]
pub extern fn xbd_shell_on_char(ch: u8) {
    //println!("@@ xbd_shell_on_char(): {}", ch);

    if let Some(xs) = prompt_is_ready() {
        match ch as char {
            '\0' => { // end of input
                unsafe {
                    if SHELL_BUF.len() < SHELL_BUFSIZE { // allow up to SHELL_BUFSIZE - 1
                        xs.add(SHELL_BUF.clone()); // send input
                        SHELL_BUF.clear();
                    } else {
                        println!("@@ input too long (> {}); to be ignored", SHELL_BUFSIZE - 1);
                        SHELL_BUF.clear();
                        xs.add(SHELL_BUF.clone()); // send empty input
                    }
                }
            },
            '\r' | '\n' => (), // ignore it
            _ => { // keep it
                unsafe {
                    SHELL_BUF
                        .push(ch as char)
                        .unwrap_or_else(|_| { // input too long
                            //println!("@@ NOP; input (> SHELL_BUFSIZE={})", SHELL_BUFSIZE);
                        });
                }
            },
        }
    }
}

type StreamItem = heapless::String<{ SHELL_BUFSIZE }>;
static SD: StreamData<StreamItem> = stream_uninit();

pub async fn process_shell_stream() -> Result<(), i8> {
    assert_eq!(unsafe { xbd_shell_bufsize() }, SHELL_BUFSIZE);

    let ret = unsafe { xbd_shell_init() };
    match ret {
        0 => (), // ok, continue
        2 => { // kludge
            println!("@@ process_shell_stream(): TODO - support non-native board");
            return Ok(());
        },
        _ => return Err(ret),
    }

    let mut stream = XbdStream::new_with_cap(&SD, 1);
    prompt();

    while let Some(input) = stream.next().await {
        assert!(input.len() < SHELL_BUFSIZE);
        println!("@@ [async shell] input: {} (len: {} SHELL_BUFSIZE: {})",
                 input, input.len(), SHELL_BUFSIZE);
        println!("@@ [async shell] input.as_bytes(): {:?}", input.as_bytes());

        match input {
            _ => (),
        }

        if 0 == 1 { crate::Xbd::async_sleep(1_000).await; } // debug, ok

        prompt();
    }

    Ok(())
}

fn prompt() {
    unsafe { xbd_shell_prompt(); }
}

fn prompt_is_ready() -> Option<XbdStream<StreamItem>> {
    let xs = XbdStream::get(&SD).unwrap();

    if xs.len() == 0 { // no pending items
        Some(xs)
    } else { None }
}