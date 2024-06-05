use std::{ffi::c_void, sync::Mutex};

use anyhow::{anyhow, Result};
use winapi::um::winuser::VK_DELETE;

use crate::{
    hook, interfaces, offsets,
    sdk::{get_virtual_function, CEngineClient, LocalPlayer},
};
use lazy_static::lazy_static;
use toy_arms::{keyboard::detect_keypress, module::Module, GameObject};

const EXIT_KEY: i32 = VK_DELETE;

lazy_static! {
    static ref CHEAT_CONTEXT: Mutex<CheatContext> = Mutex::new(CheatContext::default());
}

struct CheatContext {
    client_module: Module,
    engine2_module: Module,
    engine_client: CEngineClient,
}

impl CheatContext {
    fn default() -> Self {
        Self {
            client_module: Module::default(),
            engine2_module: Module::default(),
            engine_client: CEngineClient::default(),
        }
    }

    unsafe fn get_local_player(&self) -> Option<LocalPlayer> {
        Some(
            LocalPlayer::from_raw(self.client_module.read(offsets::client::dwLocalPlayerPawn))?
                .read(),
        )
    }
}

unsafe impl Sync for CheatContext {}
unsafe impl Send for CheatContext {}

pub fn initialize() -> Result<()> {
    println!("initializing!");
    std::thread::sleep(std::time::Duration::from_millis(400));

    let mut context = CHEAT_CONTEXT.lock().unwrap();

    hook::initialize()?;

    let Some(client) = Module::from_name("client.dll") else {
        return Err(anyhow!("Failed to get client.dll module"));
    };
    context.client_module = client;
    println!(
        "loaded client.dll {:#0x}",
        context.client_module.base_address
    );

    let Some(engine2) = Module::from_name("engine2.dll") else {
        return Err(anyhow!("Failed to get engine2 module"));
    };
    context.engine2_module = engine2;
    println!(
        "loaded engine2.dll {:#0x}",
        context.engine2_module.base_address
    );

    let module_factory = interfaces::get_factory(context.engine2_module.handle);
    let Some(source2_engine_to_client_interface) =
        interfaces::get_interface(module_factory, "Source2EngineToClient001")
    else {
        return Err(anyhow!(
            "Failed to get engine2.Source2EngineToClient001 interface"
        ));
    };
    context.engine_client.base = source2_engine_to_client_interface as *mut usize;
    println!(
        "Found interface Source2EngineToClient001 at {:#0x}",
        context.engine_client.base as usize
    );

    println!("hooking functions!");
    std::thread::sleep(std::time::Duration::from_millis(400));

    let frame_stage_notify_address = get_virtual_function(context.engine_client.base, 36);

    unsafe {
        FRAME_STAGE_NOTIFY = hook::create_hook(
            frame_stage_notify_address as *mut c_void,
            hook_frame_stage_notify as *mut c_void,
        )?;

        println!("hooked frame stage notify");
    }

    hook::enable_hooks()
}

static mut FRAME_STAGE_NOTIFY: *mut c_void = std::ptr::null_mut();

type FrameStageNotify = extern "fastcall" fn(rcx: *mut c_void, stage: i32);

unsafe extern "fastcall" fn hook_frame_stage_notify(rcx: *mut c_void, stage: i32) {
    let frame_stage_notify: FrameStageNotify = std::mem::transmute(FRAME_STAGE_NOTIFY);
    if stage == 5 || stage == 6 {
        let context = CHEAT_CONTEXT.lock().unwrap();
        if let Some(local_player) = context.get_local_player() {
            if let Some(weapon_services) = local_player.get_weapon_services() {
                let weapons = weapon_services.get_weapons();
                for weapon in weapons {}
            }
        }
    }
    frame_stage_notify(rcx, stage)
}

pub unsafe fn run() {
    while !detect_keypress(EXIT_KEY) {
        let context = CHEAT_CONTEXT.lock().unwrap();
        if !context.engine_client.get_is_in_game() || !context.engine_client.get_is_connected() {
            continue;
        }

        let Some(local_player) = context.get_local_player() else {
            continue;
        };

        let health = local_player.get_health();
        if health < 1 {
            continue;
        }

        const DESIRED_FOV: u32 = 120;

        if !local_player.get_is_scoped() {
            if let Some(camera_services) = local_player.get_camera_services() {
                let current_fov = camera_services.get_fov();
                if current_fov != DESIRED_FOV {
                    camera_services.set_fov(DESIRED_FOV);
                }
            }
        }
    }
}

pub fn uninitialize() {
    println!("uninitiliazing!");
    std::thread::sleep(std::time::Duration::from_millis(200));

    if let Err(error) = hook::disable_hooks() {
        eprintln!("failed to disable hooks: {error}");
    }

    if let Err(error) = hook::uninitialize() {
        eprintln!("failed to uninitilize minhook: {error}");
    }
}
