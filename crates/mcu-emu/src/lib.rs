use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

static FLASH_BIN: &str = "esp32flash.bin";

pub fn run_with_qemu(app_bin: &str) -> std::io::Result<()> {
    generate_esp32flash(app_bin, FLASH_BIN)?;

    println!("Running qemu... ('Ctrl-a x' to exit)");
    run_qemu(FLASH_BIN)?;

    { // WIP
        use std::{thread, time};

        thread::sleep(time::Duration::from_millis(4_000));

        Command::new("killall")
            .args(&["qemu-system-xtensa"])
            .status()?;
        println!("done.");
    }

    Ok(())
}

pub fn run_qemu(flash_bin: &str) -> std::io::Result<()> {
    let nic = "user,model=open_eth,id=lo0"; // SLIRP
    // let nic = "user,model=open_eth,id=lo0,hostfwd=tcp:127.0.0.1:60080-:80"; // SLIRP_HOSTFWD

    Command::new("../../toolchain/xtensa/qemu/qemu/bin/qemu-system-xtensa")
        .args(&["-nographic",
            "-machine", "esp32",
            "-drive", &format!("file={},if=mtd,format=raw", flash_bin),
            "-nic", nic,
            "-global", "driver=timer.esp32.timg,property=wdt_disable,value=true"])
        // .status()?;
        .spawn()?; // !!!!

    Ok(())
}

pub fn generate_esp32flash<P: AsRef<Path>>(app_bin: P, flash_bin: P) -> std::io::Result<()> {
    let mut flash = vec![0xff; 4194304]; // 4 * 1024 * 1024 (4 MiB)
    merge(&mut flash, 0x1000, "../../toolchain/xtensa/qemu/build-dio-riot/bootloader/bootloader.bin")?;
    merge(&mut flash, 0x8000, "../../toolchain/xtensa/qemu/build-dio-riot/partition_table/partition-table.bin")?;
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