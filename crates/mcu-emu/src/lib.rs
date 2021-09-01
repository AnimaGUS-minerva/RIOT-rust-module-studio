mod native;
mod esp32;

pub use native::run_native;
pub use esp32::run_esp32;

static RIOT_NATIVE_ELF: &str = "./riot/bin/native/riot.elf";
static RIOT_ESP32_BIN: &str = "riot.esp32.bin";

pub fn run(args: &Vec<String>) -> std::io::Result<()> {
    let board = args[1].as_str();
    let timeout = if args.len() > 2 { Some(args[2].parse::<u64>().unwrap()) } else { None };

    match board {
        "native" => run_native(RIOT_NATIVE_ELF, timeout)?,
        "esp32" => run_esp32(RIOT_ESP32_BIN, timeout)?,
        _ => panic!("Unsupported board: {}", board),
    }

    Ok(())
}
