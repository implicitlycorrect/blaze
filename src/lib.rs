mod config;
mod context;
mod features;
mod offsets;
mod sdk;

use cheatlib::*;
use features::{
    bunnyhop::Bunnyhop, fov_changer::FovChanger, no_flash::NoFlash, triggerbot::Triggerbot, Feature,
};

use std::time::Duration;

use crate::{config::Config, context::Context, sdk::interfaces::Interface};

fn main() -> Result<()> {
    set_console_title("Blaze");

    let mut context = Context::create(Config::default())?;

    println!("loaded client.dll {:#0x}", context.client.base_address);
    println!("loaded engine2.dll {:#0x}", context.engine.base_address);
    println!(
        "Created CEngineClient interface: {:p}",
        context.cengine_client.get_base_address()
    );

    let mut features: Vec<Box<dyn Feature>> = vec![
        Box::new(Bunnyhop),
        Box::new(FovChanger),
        Box::new(NoFlash),
        Box::new(Triggerbot::default()),
    ];

    while !keyboard::detect_keypress(context.config.exit_key) {
        std::thread::sleep(Duration::from_millis(1));

        let is_in_game = context.cengine_client.get_is_in_game().unwrap_or_default();

        let is_connected = context
            .cengine_client
            .get_is_connected()
            .unwrap_or_default();

        if !is_in_game || !is_connected {
            continue;
        }

        let Ok(local_player) = context.get_local_player() else {
            continue;
        };
        context.local_player = local_player;

        for feature in features.iter_mut() {
            if let Err(error) = feature.run(&context) {
                eprintln!("{error}")
            }
        }
    }
    Ok(())
}

dll_main!(main, true);
