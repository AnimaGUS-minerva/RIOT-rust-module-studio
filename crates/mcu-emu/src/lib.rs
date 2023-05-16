mod native;
mod esp32;

pub use native::run_native;
pub use esp32::run_esp32;

pub enum NetOpt {
    Tap(Option<String>),
    Nic(Option<String>),
}

pub fn run(args: &Vec<String>) -> std::io::Result<()> {
    let argc = args.len();
    assert!(argc > 2);
    let board = args[1].as_str();
    let timeout = args[2].parse::<u64>().ok().filter(|n| *n > 0);
    let net = if argc > 3 { Some(args[3].clone()) } else { None };

    match board {
        "native" => run_native(&std::env::var("RIOT_NATIVE_ELF")
            .unwrap_or("./main/bin/native/main.elf".to_string()), timeout, NetOpt::Tap(net))?,
        "esp32" => run_esp32(&std::env::var("RIOT_ESP32_BIN")
            .unwrap_or("./main.esp32.bin".to_string()), timeout, NetOpt::Nic(net))?,
        _ => panic!("Unsupported board: {}", board),
    }

    Ok(())
}
