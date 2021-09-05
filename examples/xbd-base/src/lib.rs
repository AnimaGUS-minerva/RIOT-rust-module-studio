#![no_std]

use mcu_if::println;
use mcu_if::alloc::{boxed::Box, vec, vec::Vec};
use mcu_if::core2::io::{self as io, Cursor, Seek, SeekFrom, Write};

#[no_mangle]
pub extern fn square(input: i32) -> i32 {
    println!("[src/lib.rs] square(): input: {}", input);

    demo_alloc();
    demo_io();

    input * input
}

//

fn demo_alloc() {
    println!("Box::new(42): {:?}", Box::new(42));
    println!("Box::new([0; 10]): {:?}", Box::new([0; 10]));
    println!("Vec::from([0, 1, 2]): {:?}", Vec::from([0, 1, 2]));
    println!("vec![0, 1, 2]: {:?}", vec![0, 1, 2]);
}

// code adapted from https://doc.rust-lang.org/std/io/struct.Cursor.html

fn demo_io() {
    let mut array = [0; 15];
    let mut buff = Cursor::new(&mut array[..]);

    write_ten_bytes_at_end(&mut buff).unwrap();
    println!("buff: {:?}", buff);

    let check = &buff.get_ref()[5..15];
    println!("check: {:?}", check);
    assert_eq!(check, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

fn write_ten_bytes_at_end<W: Write + Seek>(writer: &mut W) -> io::Result<()> {
    writer.seek(SeekFrom::End(-10))?;

    for i in 0..10 {
        writer.write(&[i])?;
    }

    Ok(())
}
