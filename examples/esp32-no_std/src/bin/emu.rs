use mcu_emu::generate_esp32flash;

fn main() -> std::io::Result<()> {
    println!("src/bin/emu.rs::main(): ^^");

    generate_esp32flash("riot.esp32.bin", "esp32flash.bin")?;

    Ok(())
}
