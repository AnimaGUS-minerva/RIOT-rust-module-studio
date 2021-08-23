use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

static QEMU_RES_DIR: &str = "../../toolchain/xtensa/qemu";
static FLASH_BIN: &str = "esp32flash.bin";

pub fn run_with_qemu_xtensa(app_bin: &str) -> std::io::Result<()> {
    generate_esp32flash(app_bin, FLASH_BIN)?;

    println!("Running qemu... ('Ctrl-a x' to exit)");
    run_qemu_xtensa(FLASH_BIN)?;

    { // WIP
        use std::{thread, time};

        thread::sleep(time::Duration::from_millis(4_000));

        // FIXME terminal malformatted later??!!
        Command::new("killall")
            .args(&["qemu-system-xtensa"])
            .status()?;
        println!("done.");
    }

    Ok(())
}

pub fn run_qemu_xtensa(flash_bin: &str) -> std::io::Result<()> {
    let nic = "user,model=open_eth,id=lo0"; // SLIRP
    // let nic = "user,model=open_eth,id=lo0,hostfwd=tcp:127.0.0.1:60080-:80"; // SLIRP_HOSTFWD

    Command::new(&format!("{}/qemu/bin/qemu-system-xtensa", QEMU_RES_DIR))
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
    Flash::new(4194304) // 4 * 1024 * 1024 (4 MiB)
        .merge(0x1000, &format!("{}/build-dio-riot/bootloader/bootloader.bin", QEMU_RES_DIR))?
        .merge(0x8000, &format!("{}/build-dio-riot/partition_table/partition-table.bin", QEMU_RES_DIR))?
        .merge(0x10000, app_bin)?
        .write(flash_bin)?;

    Ok(())
}

pub struct Flash(Vec<u8>);

impl Flash {
    pub fn new(size: usize) -> Self {
        Self(vec![0xff; size])
    }

    pub fn merge<P: AsRef<Path>>(mut self, offset: usize, path: P) -> std::io::Result<Self> {
        let mut f = File::open(path)?;
        let len = f.metadata()?.len() as usize;
        f.read_exact(&mut self.0[offset .. (offset + len)])?;

        Ok(self)
    }
    pub fn write<P: AsRef<Path>>(self, path: P) -> std::io::Result<Self> {
        fs::write(path, &self.0)?;

        Ok(self)
    }
}

#[test]
fn test_flash() -> std::io::Result<()> {
    let bin4 = "4.bin";
    fs::write(bin4, &vec![11; 4])?;

    let bin8 = "8.bin";
    fs::write(bin8, &vec![22; 8])?;

    let binflash = "flash.bin";
    let flash = Flash::new(16)
        .merge(1, bin4)?
        .merge(6, bin8)?
        .write(binflash)?;

    let gt = [255, 11, 11, 11, 11, 255, 22, 22, 22, 22, 22, 22, 22, 22, 255, 255];
    assert_eq!(flash.0, gt);

    let mut f = File::open(binflash)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    assert_eq!(buf, gt);

    fs::remove_file(bin4)?;
    fs::remove_file(bin8)?;
    fs::remove_file(binflash)?;

    Ok(())
}
