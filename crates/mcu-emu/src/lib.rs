mod native;
mod esp32;

pub use native::run_native;
pub use esp32::run_esp32;

pub fn run(args: &Vec<String>) -> std::io::Result<()> {
    let board = args[1].as_str();
    let timeout = if args.len() > 2 { Some(args[2].parse::<u64>().unwrap()) } else { None };

    match board {
        "native" => run_native(&std::env::var("RIOT_NATIVE_ELF")
            .unwrap_or("./main/bin/native/main.elf".to_string()), timeout)?,
        "esp32" => run_esp32(&std::env::var("RIOT_ESP32_BIN")
            .unwrap_or("./main.esp32.bin".to_string()), timeout)?,
        _ => panic!("Unsupported board: {}", board),
    }

    Ok(())
}
