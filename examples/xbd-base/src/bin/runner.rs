use std::env;

fn main() -> std::io::Result<()> {
    mcu_emu::run(&env::args().collect())
}
