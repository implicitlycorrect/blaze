mod cheat;
mod config;
mod context;
mod interfaces;
mod offsets;
mod sdk;

use cheatlib::*;

fn main() -> Result<()> {
    cheat::initialize()?;
    cheat::run();
    Ok(())
}

dll_main!(main);
