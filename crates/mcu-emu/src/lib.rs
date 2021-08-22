use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn generate_esp32flash<P: AsRef<Path>>(app_bin: P, flash_bin: P) -> std::io::Result<()> {
    let mut flash = vec![0xff; 4194304]; // 4 * 1024 * 1024 (4 MiB)
    merge(&mut flash, 0x1000, "build-dio-riot/bootloader/bootloader.bin")?;
    merge(&mut flash, 0x8000, "build-dio-riot/partition_table/partition-table.bin")?;
    merge(&mut flash, 0x10000, app_bin)?;
    fs::write(flash_bin, &flash)?;

    Ok(())
}

fn merge<P: AsRef<Path>>(flash: &mut Vec<u8>, offset: usize, path: P) -> std::io::Result<()> {
    let mut f = File::open(path)?;
    let len = f.metadata()?.len() as usize;
    f.read_exact(&mut flash[offset .. (offset + len)])?;

    Ok(())
}

#[test]
fn test_merge() -> std::io::Result<()> {
    let mut flash = vec![0xff; 16];

    let bin = "four.bin";
    fs::write(bin, &vec![11; 4])?;
    merge(&mut flash, 1, bin)?;
    fs::remove_file(bin)?;
    println!("flash: {:?}", flash);

    let bin = "eight.bin";
    fs::write(bin, &vec![22; 8])?;
    merge(&mut flash, 6, bin)?;
    fs::remove_file(bin)?;
    println!("flash: {:?}", flash);

    assert_eq!(flash, [255, 11, 11, 11, 11, 255, 22, 22, 22, 22, 22, 22, 22, 22, 255, 255]);

    Ok(())
}