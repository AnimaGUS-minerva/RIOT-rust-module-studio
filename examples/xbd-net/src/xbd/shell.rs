use super::stream::{XbdStream, StreamData, stream_uninit, StreamExt};
use crate::println;
use mcu_if::c_types::c_void;

extern "C" {
    fn xbd_shell_get_commands() -> *const c_void;
    fn xbd_async_shell_init() -> i8;
    fn xbd_async_shell_bufsize() -> usize;
    fn xbd_async_shell_prompt(tag_cstr: *const u8, highlight: bool);
    fn handle_input_line_minerva(command_list: *const c_void, line: *const u8);
}

const SHELL_BUFSIZE: usize = 128;
type ShellBuf = heapless::String<SHELL_BUFSIZE>;

static mut SHELL_BUF: ShellBuf = heapless::String::new();
static SD: StreamData<ShellBuf> = stream_uninit();

#[no_mangle]
pub extern fn xbd_async_shell_on_char(ch: u8) {
    //println!("@@ xbd_async_shell_on_char(): {}", ch);

    if let Some(xs) = prompt_is_ready() {
        let ch = ch as char;
        match ch {
            '\0' => { // end of input
                unsafe {
                    let null_terminated;
                    if let Ok(_) = SHELL_BUF.push(ch) {
                        null_terminated = SHELL_BUF.clone();
                        SHELL_BUF.clear();
                    } else {
                        println!("@@ input too long (> {}); to be ignored", SHELL_BUFSIZE - 1);
                        SHELL_BUF.clear();
                        SHELL_BUF.push(ch).unwrap();
                        null_terminated = SHELL_BUF.clone();
                    }

                    xs.add(null_terminated);
                }
            },
            '\r' | '\n' => (), // ignore it
            _ => { // keep it
                unsafe {
                    SHELL_BUF
                        .push(ch)
                        .unwrap_or_else(|_| { // input too long
                            //println!("@@ NOP; input (> SHELL_BUFSIZE={})", SHELL_BUFSIZE);
                        });
                }
            },
        }
    }
}

pub async fn process_shell_stream() -> Result<(), i8> {
    assert_eq!(unsafe { xbd_async_shell_bufsize() }, SHELL_BUFSIZE);

    let ret = unsafe { xbd_async_shell_init() };
    match ret {
        0 => (), // ok, continue
        2 => { // kludge
            println!("@@ process_shell_stream(): TODO - support non-native board");
            return Ok(());
        },
        _ => return Err(ret),
    }

    //let shell_commands = core::ptr::null(); // system commands only
    let shell_commands = unsafe { xbd_shell_get_commands() };

    let mut stream = XbdStream::new_with_cap(&SD, 1);
    prompt();

    while let Some(mut line) = stream.next().await {
        println!("[async shell] (null terminated) line: {} (len: {} SHELL_BUFSIZE: {})",
                 line, line.len(), SHELL_BUFSIZE);
        //println!("  line.as_bytes(): {:?}", line.as_bytes());
        //println!("  line: {:?}", line);

        match line.as_str() { // alias handling
            "h\0" => {
                line.clear();
                line.push_str("help\0").unwrap();
            },
            // ...
            _ => (),
        }

        unsafe { handle_input_line_minerva(shell_commands, line.as_ptr()); }

        if 0 == 1 { crate::Xbd::async_sleep(1_000).await; } // debug, ok

        prompt();
    }

    Ok(())
}

fn prompt() {
    //let tag: Option<&str> = None;
    let tag = Some("(async)\0");

    let tag = if let Some(x) = tag {
        assert!(x.ends_with("\0"));
        x.as_ptr()
    } else {
        core::ptr::null()
    };

    unsafe { xbd_async_shell_prompt(tag, true); }
}

fn prompt_is_ready() -> Option<XbdStream<ShellBuf>> {
    let xs = XbdStream::get(&SD).unwrap();

    if xs.len() == 0 { // no pending items
        Some(xs)
    } else { None }
}