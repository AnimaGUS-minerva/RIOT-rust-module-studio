fn main() -> std::io::Result<()> {
    println!("src/bin/emu.rs::main(): ^^");

    mcu_emu::run_qemu_xtensa("riot.esp32.bin", Some(4_000))?;
    // mcu_emu::run_qemu_xtensa("riot.esp32.bin", None)?;

    Ok(())
}