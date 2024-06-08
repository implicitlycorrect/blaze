use cheatlib::*;

use std::{
    sync::Mutex,
    time::{Duration, Instant},
};

use crate::{config::*, context::CheatContext, interfaces, offsets};
use lazy_static::lazy_static;

lazy_static! {
    static ref CHEAT_CONTEXT: Mutex<CheatContext> = Mutex::new(CheatContext::default());
}

pub fn initialize() -> Result<()> {
    set_console_title("Blaze");

    println!("initializing!");

    let mut context = CHEAT_CONTEXT.lock().unwrap();

    context.client_module = Module::from_name("client.dll")?;
    println!(
        "loaded client.dll {:#0x}",
        context.client_module.base_address
    );

    context.engine2_module = Module::from_name("engine2.dll")?;
    println!(
        "loaded engine2.dll {:#0x}",
        context.engine2_module.base_address
    );

    let Some(client_interface) = interfaces::get_interface(
        interfaces::get_factory(&context.client_module).unwrap(),
        "Source2Client002",
    ) else {
        return Err(anyhow!("Failed to get client.Source2Client002 interface"));
    };
    context.client_interface.base = client_interface as *mut usize;

    let Some(engine_client_interface) = interfaces::get_interface(
        interfaces::get_factory(&context.engine2_module).unwrap(),
        "Source2EngineToClient001",
    ) else {
        return Err(anyhow!(
            "Failed to get engine2.Source2EngineToClient001 interface"
        ));
    };
    context.engine_client_interface.base = engine_client_interface as *mut usize;
    println!(
        "Found interface Source2EngineToClient001 at {:#0x}",
        context.engine_client_interface.base as usize
    );

    Ok(())
}

pub fn run() {
    let mut time_since_last_shot: Instant = Instant::now() - Duration::from_secs(1);

    while !keyboard::detect_keypress(EXIT_KEY) {
        std::thread::sleep(Duration::from_millis(1));

        let context = CHEAT_CONTEXT.lock().unwrap();
        if !context
            .engine_client_interface
            .get_is_in_game()
            .unwrap_or_default()
            || !context
                .engine_client_interface
                .get_is_connected()
                .unwrap_or_default()
        {
            continue;
        }

        let Some(local_player) = context.get_local_player() else {
            continue;
        };

        // FOV CHANGER
        const DESIRED_FOV: u32 = 120;

        if !local_player.get_is_scoped().unwrap_or_default() {
            if let Some(camera_services) = local_player.get_camera_services() {
                if let Some(current_fov) = camera_services.get_fov() {
                    if current_fov != DESIRED_FOV {
                        camera_services.set_fov(DESIRED_FOV);
                    }
                }
            }
        }

        // TRIGGERBOT
        if !keyboard::detect_keypress(TRIGGERBOT_KEY) {
            continue;
        }

        let attack_pointer =
            (context.client_module.base_address + offsets::buttons::attack) as *mut i32;
        if attack_pointer.is_null() {
            continue;
        }

        let Some(crosshair_entity_handle) = local_player.get_handle_of_entity_in_crosshair() else {
            continue;
        };

        if crosshair_entity_handle <= 0 {
            continue;
        }

        let now = Instant::now();

        let should_shoot = now.duration_since(time_since_last_shot) > Duration::from_millis(16)
            && unsafe { *attack_pointer } <= 256;

        if should_shoot {
            let _ = context
                .engine_client_interface
                .execute_client_command("+attack;-attack");
            time_since_last_shot = now;
        }

        // +attack;-attack; fix !!!
        unsafe {
            if *attack_pointer == 257 {
                *attack_pointer = 256;
            }
        }
    }
}
