use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::{Command, ExitStatus, Stdio};
use std::{thread, time};

static QEMU_RES_DIR: &str = "../../toolchain/xtensa/qemu";
static FLASH_BIN: &str = "esp32flash.bin";

pub fn run_esp32(riot_bin: &str, timeout_ms: Option<u64>) -> std::io::Result<()> {
    generate_esp32flash(riot_bin, FLASH_BIN)?;

    if let Some(ms) = timeout_ms {
        Qemu::new().run_with_timeout(ms)?;
    } else {
        Qemu::new().run()?;
    }

    Ok(())
}

//

pub struct Qemu(Command);

impl Qemu {
    pub fn new() -> Self {
        let nic = "user,model=open_eth,id=lo0"; // SLIRP
        // let nic = "user,model=open_eth,id=lo0,hostfwd=tcp:127.0.0.1:60080-:80"; // SLIRP_HOSTFWD

        let mut cmd = Command::new(&format!("{}/qemu/bin/qemu-system-xtensa", QEMU_RES_DIR));
        cmd.args(&["-nographic",
            "-machine", "esp32",
            "-drive", &format!("file={},if=mtd,format=raw", FLASH_BIN),
            "-nic", nic,
            "-global", "driver=timer.esp32.timg,property=wdt_disable,value=true"]);

        Self(cmd)
    }

    pub fn run(&mut self) -> std::io::Result<ExitStatus> {
        println!("Running qemu... ('Ctrl-a x' to exit)");

        self.0.status()
    }

    pub fn run_with_timeout(&mut self, ms: u64) -> std::io::Result<()> {
        println!("Running qemu... (timeout {} ms)", ms);
        let process = self.0
            .stdin(Stdio::piped())
            .spawn()?;

        thread::sleep(time::Duration::from_millis(ms));

        println!("Quiting qemu...");
        // $ echo $'\cax' | hexdump -C
        // 00000000  01 78 0a                                          |.x.|
        // 00000003
        process.stdin.unwrap().write_all(&[0x01, 0x78, 0x0a])?;

        Ok(())
    }
}

//

pub fn generate_esp32flash<P: AsRef<Path>>(riot_bin: P, flash_bin: P) -> std::io::Result<()> {
    Flash::new(4194304) // 4 * 1024 * 1024 (4 MiB)
        .merge(0x1000, &format!("{}/build-dio-riot/bootloader/bootloader.bin", QEMU_RES_DIR))?
        .merge(0x8000, &format!("{}/build-dio-riot/partition_table/partition-table.bin", QEMU_RES_DIR))?
        .merge(0x10000, riot_bin)?
        .write(flash_bin)?;

    Ok(())
}

pub struct Flash(Vec<u8>);

impl Flash {
    pub fn new(size: usize) -> Self {
        Self(vec![0xff; size])
    }

    pub fn merge<P: AsRef<Path>>(&mut self, offset: usize, path: P) -> std::io::Result<&mut Self> {
        let mut f = File::open(path)?;
        let len = f.metadata()?.len() as usize;
        f.read_exact(&mut self.0[offset .. (offset + len)])?;

        Ok(self)
    }
    pub fn write<P: AsRef<Path>>(&self, path: P) -> std::io::Result<&Self> {
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

    let gt = [255, 11, 11, 11, 11, 255, 22, 22, 22, 22, 22, 22, 22, 22, 255, 255];
    assert_eq!(
        Flash::new(16)
            .merge(1, bin4)?
            .merge(6, bin8)?
            .write(binflash)?
            .0, gt);

    let mut f = File::open(binflash)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    assert_eq!(buf, gt);

    fs::remove_file(bin4)?;
    fs::remove_file(bin8)?;
    fs::remove_file(binflash)?;

    Ok(())
}
