use cheatlib::*;

use std::{
    sync::Mutex,
    time::{Duration, Instant},
};

use crate::{config::*, context::CheatContext, interfaces, offsets, sdk::LocalPlayer};
use lazy_static::lazy_static;

lazy_static! {
    static ref CHEAT_CONTEXT: Mutex<CheatContext> = Mutex::new(CheatContext::default());
}

pub fn initialize() -> Result<()> {
    set_console_title("Blaze");

    println!("initializing!");

    let mut context = CHEAT_CONTEXT.lock().unwrap();

    context.client = Module::from_name("client.dll")?;
    println!("loaded client.dll {:#0x}", context.client.base_address);

    context.engine = Module::from_name("engine2.dll")?;
    println!("loaded engine2.dll {:#0x}", context.engine.base_address);

    let Some(engine_client_interface) = interfaces::get_interface(
        interfaces::get_factory(&context.engine).unwrap(),
        "Source2EngineToClient001",
    ) else {
        return Err(anyhow!(
            "Failed to get engine2.Source2EngineToClient001 interface"
        ));
    };
    context.cengine_client.base = engine_client_interface as *mut usize;
    println!(
        "Found interface Source2EngineToClient001 at {:#0x}",
        context.cengine_client.base as usize
    );

    Ok(())
}

pub fn run() {
    let mut last_shot_time = Instant::now() - Duration::from_secs(1);

    while !keyboard::detect_keypress(EXIT_KEY) {
        std::thread::sleep(Duration::from_millis(1));

        let context = CHEAT_CONTEXT.lock().unwrap();
        if !context.cengine_client.get_is_in_game().unwrap_or_default()
            || !context
                .cengine_client
                .get_is_connected()
                .unwrap_or_default()
        {
            continue;
        }

        let Some(local_player) = context.get_local_player() else {
            continue;
        };

        // FOV CHANGER
        fov_changer(&local_player);

        // TRIGGERBOT
        if keyboard::detect_keypress(TRIGGERBOT_KEY)
            && triggerbot(&context, &local_player, last_shot_time)
        {
            last_shot_time = Instant::now();
        }

        // BHOP
        bhop(&context, &local_player);
    }
}

fn fov_changer(local_player: &LocalPlayer) {
    if local_player.get_is_scoped().unwrap_or_default() {
        return;
    }

    let Some(camera_services) = local_player.get_camera_services() else {
        return;
    };

    let Some(current_fov) = camera_services.get_fov() else {
        return;
    };

    if current_fov == DESIRED_FOV {
        return;
    }

    camera_services.set_fov(DESIRED_FOV);
}

fn triggerbot(
    context: &CheatContext,
    local_player: &LocalPlayer,
    last_shot_instant: Instant,
) -> bool {
    unsafe {
        let attack_pointer = (context.client.base_address + offsets::buttons::attack) as *mut i32;
        if attack_pointer.is_null() {
            return false;
        }

        // +attack;-attack fix
        if *attack_pointer == 257 {
            *attack_pointer = 256;
        }

        if *attack_pointer > 256
            || Instant::now().duration_since(last_shot_instant) < TRIGGERBOT_TIME_BETWEEN_SHOTS
        {
            return false;
        }
    }

    let crosshair_entity_handle = local_player
        .get_handle_of_entity_in_crosshair()
        .unwrap_or_default();
    if crosshair_entity_handle <= 0 {
        return false;
    }

    let _ = context
        .cengine_client
        .execute_client_command("+attack;-attack");

    true
}

fn bhop(context: &CheatContext, local_player: &LocalPlayer) {
    let Some(on_ground) = local_player.get_on_ground() else {
        return;
    };

    let jump_held = keyboard::detect_keypress(BHOP_KEY);
    let should_jump = jump_held && on_ground;
    if !should_jump {
        return;
    }

    unsafe {
        let jump_pointer = (context.client.base_address + offsets::buttons::jump) as *mut i32;
        if jump_pointer.is_null() && *jump_pointer <= 256 {
            return;
        }

        let _ = context.cengine_client.execute_client_command("+jump;-jump");
        *jump_pointer = 256;
    }
}
