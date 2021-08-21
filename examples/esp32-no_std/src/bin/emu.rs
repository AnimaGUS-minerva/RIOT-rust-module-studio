use mcu_emu::generate_esp32flash;

fn main() -> std::io::Result<()> {
    println!("hi from src/bin/emu.rs");

    generate_esp32flash()?;

    Ok(())
}
