use std::process::{Command, Stdio};
use std::{thread, time};

pub fn run_native(riot_elf: &str, timeout_ms: Option<u64>) -> std::io::Result<()> {
    let mut cmd = Command::new(riot_elf);

    if let Some(ms) = timeout_ms {
        println!("Running native... (timeout {} ms)", ms);
        let mut process = cmd
            .stdin(Stdio::piped())
            .spawn()?;

        thread::sleep(time::Duration::from_millis(ms));

        println!("Quiting native...");
        process.kill()?;
    } else {
        cmd.status()?;
    }

    Ok(())
}