use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let board = &args[1];
    let timeout = if args.len() > 2 { Some(args[2].parse::<u64>().unwrap()) } else { None };

    match board.as_str() {
        "esp32" => mcu_emu::run_qemu_xtensa("riot.esp32.bin", timeout)?,
        "native" => mcu_emu::run_native("./riot/bin/native/riot.elf", timeout)?,
        _ => panic!("Unsupported board: {}", board),
    }

    Ok(())
}
