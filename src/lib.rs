mod config;
mod context;
mod features;
mod offsets;
mod sdk;

use std::time::Duration;

use cheatlib::*;
use features::{
    bunnyhop::Bunnyhop, fov_changer::FovChanger, no_flash::NoFlash, triggerbot::Triggerbot, Feature,
};
use sdk::Interface;

use crate::{config::Config, context::Context};

fn main() -> Result<()> {
    let config = Config::from_file("config.toml").unwrap_or_default();
    println!("loaded config: {:#?}", config);

    let mut context = Context::create(config)?;
    println!("loaded client.dll {:#0x}", context.client.base_address);
    println!("loaded engine2.dll {:#0x}", context.engine.base_address);
    println!(
        "created CEngineClient interface {:p}",
        context.cengine_client.get_base_address()
    );

    let mut features: Vec<Box<dyn Feature>> = vec![
        Box::new(Triggerbot::default()),
        Box::new(Bunnyhop),
        Box::new(FovChanger),
        Box::new(NoFlash),
    ];

    while !keyboard::detect_keypress(context.config.exit_key) {
        std::thread::sleep(Duration::from_millis(1));

        let is_connected = context
            .cengine_client
            .get_is_connected()
            .unwrap_or_default();

        if !is_connected {
            continue;
        }

        let Ok(local_player) = context.get_local_player() else {
            continue;
        };
        context.local_player = local_player;

        for feature in features.iter_mut() {
            if !feature.get_is_enabled(&context.config) {
                continue;
            }

            if let Err(error) = feature.run(&context) {
                eprintln!(
                    "Error occured when running feature: {}\n{}",
                    feature.get_name(),
                    error
                );
            }
        }
    }

    Ok(())
}

dll_main!(main, true);
