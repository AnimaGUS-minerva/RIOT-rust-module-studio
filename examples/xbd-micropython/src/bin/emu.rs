fn main() -> std::io::Result<()> {
    mcu_emu::run_qemu_xtensa("riot.esp32.bin", Some(4_000))?;
    // mcu_emu::run_qemu_xtensa("riot.esp32.bin", None)?;

    Ok(())
}
