mod config;
mod context;
mod offsets;
mod sdk;

use cheatlib::*;

use std::time::{Duration, Instant};

use crate::{
    config::*,
    context::Context,
    sdk::{interfaces::Interface, LocalPlayer},
};

fn main() -> Result<()> {
    set_console_title("Blaze");

    let context = Context::create()?;
    println!("loaded client.dll {:#0x}", context.client.base_address);
    println!("loaded engine2.dll {:#0x}", context.engine.base_address);
    println!(
        "Created CEngineClient interface: {:p}",
        context.cengine_client.get_base_address()
    );

    let mut last_shot_time = Instant::now() - Duration::from_secs(1);

    while !keyboard::detect_keypress(EXIT_KEY) {
        std::thread::sleep(Duration::from_millis(1));

        if !context
            .cengine_client
            .get_is_connected()
            .unwrap_or_default()
        {
            continue;
        }

        let Ok(local_player) = context.get_local_player() else {
            continue;
        };

        // FOV CHANGER
        let _ = fov_changer(&local_player);

        // NO FLASH
        let _ = local_player.set_flash_duration(FLASH_DURATION);

        // TRIGGERBOT
        if keyboard::detect_keypress(TRIGGERBOT_KEY)
            && triggerbot(&context, &local_player, last_shot_time)
        {
            last_shot_time = Instant::now();
        }

        // BHOP
        let _ = bhop(&context, &local_player);
    }

    std::thread::sleep(Duration::from_secs(2));
    Ok(())
}

dll_main!(main, true);

fn fov_changer(local_player: &LocalPlayer) -> Result<()> {
    let is_scoped = local_player.get_is_scoped()?;
    if is_scoped {
        return Ok(());
    }

    let camera_services = unsafe { local_player.get_camera_services()?.read() };
    let current_fov = camera_services.get_fov()?;
    if current_fov == DESIRED_FOV {
        return Ok(());
    }

    camera_services.set_fov(DESIRED_FOV)
}

fn triggerbot(context: &Context, local_player: &LocalPlayer, last_shot_instant: Instant) -> bool {
    let attack_pointer = (context.client.base_address + offsets::buttons::attack) as *mut i32;
    if attack_pointer.is_null()
        || Instant::now().duration_since(last_shot_instant) < TRIGGERBOT_TIME_BETWEEN_SHOTS
    {
        return false;
    }

    unsafe {
        // +attack;-attack fix
        if *attack_pointer == 257 {
            *attack_pointer = 256;
        }

        if *attack_pointer > 256 {
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

fn bhop(context: &Context, local_player: &LocalPlayer) -> Result<()> {
    let on_ground = local_player.get_on_ground()?;
    let jump_held = keyboard::detect_keypress(JUMP_KEY);
    if !jump_held || !on_ground {
        return Ok(());
    }

    let jump_pointer = (context.client.base_address + offsets::buttons::jump) as *mut i32;
    if jump_pointer.is_null() {
        return Err(anyhow!("jump button pointer points to null"));
    }

    let _ = context.cengine_client.execute_client_command("+jump;-jump");
    unsafe {
        *jump_pointer = 256;
    }

    Ok(())
}
