use std::{
    collections::HashMap,
    ffi::c_void,
    sync::Mutex,
    time::{Duration, Instant},
};

use anyhow::{anyhow, Result};
use winapi::um::winuser::{VK_DELETE, VK_XBUTTON2};

use crate::{
    hook, interfaces, offsets,
    sdk::{get_virtual_function, CEngineClient, CSource2Client, LocalPlayer},
};
use lazy_static::lazy_static;
use toy_arms::{cast, keyboard::detect_keypress, module::Module, GameObject};

const EXIT_KEY: i32 = VK_DELETE;
const TRIGGERBOT_KEY: i32 = VK_XBUTTON2;

lazy_static! {
    static ref CHEAT_CONTEXT: Mutex<CheatContext> = Mutex::new(CheatContext::default());
    static ref ITEM_DEFINITION_INDEX_TO_SKIN_MAP: Mutex<HashMap<i16, i32>> =
        Mutex::new(HashMap::new());
}

struct CheatContext {
    client_module: Module,
    engine2_module: Module,
    client_interface: CSource2Client,
    engine_client_interface: CEngineClient,
}

impl CheatContext {
    fn default() -> Self {
        Self {
            client_module: Module::default(),
            engine2_module: Module::default(),
            client_interface: CSource2Client::default(),
            engine_client_interface: CEngineClient::default(),
        }
    }

    fn entity_list(&self) -> *mut usize {
        self.client_module.read(offsets::client::dwEntityList)
    }

    fn get_local_player(&self) -> Option<LocalPlayer> {
        Some(unsafe {
            LocalPlayer::from_raw(self.client_module.read(offsets::client::dwLocalPlayerPawn))?
                .read()
        })
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

    let Some(client_interface) = interfaces::get_interface(
        interfaces::get_factory(context.client_module.handle),
        "Source2Client002",
    ) else {
        return Err(anyhow!("Failed to get client.Source2Client002 interface"));
    };
    context.client_interface.base = client_interface as *mut usize;

    let Some(engine_client_interface) = interfaces::get_interface(
        interfaces::get_factory(context.engine2_module.handle),
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

    let mut item_definition_index_to_skin_map = ITEM_DEFINITION_INDEX_TO_SKIN_MAP.lock().unwrap();
    item_definition_index_to_skin_map.insert(9, 344);
    item_definition_index_to_skin_map.insert(4, 38);
    item_definition_index_to_skin_map.insert(61, 313);
    item_definition_index_to_skin_map.insert(32, 591);

    println!("hooking functions!");
    std::thread::sleep(std::time::Duration::from_millis(400));

    let Some(set_model_address) = context
        .client_module
        .find_pattern("48 89 5C 24 ? 48 89 7C 24 ? 55 48 8B EC 48 83 EC ? 48 8B F9 4C 8B C2")
    else {
        return Err(anyhow!(
            "failed to get address of set model function via provided signature"
        ));
    };

    unsafe {
        let set_model_address = context.client_module.base_address + set_model_address;
        println!(
            "found void SetModel(*const char) at {:#0x}",
            set_model_address
        );

        SET_MODEL = set_model_address as *mut c_void;

        let frame_stage_notify_address = get_virtual_function(context.client_interface.base, 36);
        println!(
            "found void FrameStageNotify(int stage) at {:#0x}",
            set_model_address
        );

        ORIGINAL_FRAME_STAGE_NOTIFY = hook::create_hook(
            frame_stage_notify_address as *mut c_void,
            hook_frame_stage_notify as *mut c_void,
        )?;

        println!(
            "hooked void FrameStageNotify(int stage) at {:#0x}",
            set_model_address
        );
    }

    hook::enable_hooks()
}

static mut SET_MODEL: *mut c_void = std::ptr::null_mut();

static mut ORIGINAL_FRAME_STAGE_NOTIFY: *mut c_void = std::ptr::null_mut();

unsafe extern "fastcall" fn hook_frame_stage_notify(rcx: *mut c_void, stage: i32) {
    let context = CHEAT_CONTEXT.lock().unwrap();
    let frame_stage_notify: extern "fastcall" fn(*mut c_void, i32) =
        std::mem::transmute(ORIGINAL_FRAME_STAGE_NOTIFY);

    if (context
        .engine_client_interface
        .get_is_in_game()
        .unwrap_or_default()
        && context
            .engine_client_interface
            .get_is_connected()
            .unwrap_or_default())
        && (stage == 5 || stage == 6)
    {
        // run skin changer
        if let Some(local_player) = context.get_local_player() {
            let entity_list = context.entity_list();
            if entity_list.is_null() {
                return;
            }
            let entity_list = *entity_list;

            let Some(weapon_services) = local_player.get_weapon_services() else {
                return;
            };

            let item_definition_index_to_skin_map =
                ITEM_DEFINITION_INDEX_TO_SKIN_MAP.lock().unwrap();

            let weapon_size = weapon_services.get_weapon_size().unwrap_or_default();
            for weapon_index in 0..weapon_size {
                let weapon_handle = weapon_services
                    .get_weapon_handle_at_index(weapon_index)
                    .unwrap_or_default();

                if weapon_handle <= 0 {
                    continue;
                }

                let weapon_handle = (weapon_handle & 0x7FFF) as usize;
                let weapon_list_entry = *cast!(entity_list + 8 * (weapon_handle >> 9) + 16, usize);
                let weapon_controller =
                    *cast!(weapon_list_entry + 120 * (weapon_handle & 0x1FF), usize);

                let weapon_item = weapon_controller
                    + offsets::client::C_EconEntity::m_AttributeManager
                    + offsets::client::C_AttributeContainer::m_Item;

                let item_definition_index = *cast!(
                    weapon_item + offsets::client::C_EconItemView::m_iItemDefinitionIndex,
                    i16
                );
                if !item_definition_index_to_skin_map.contains_key(&item_definition_index) {
                    continue;
                }
                let desired_paintkit = *item_definition_index_to_skin_map
                    .get(&item_definition_index)
                    .unwrap();
                let paintkit = cast!(mut weapon_controller + offsets::client::C_EconEntity::m_nFallbackPaintKit, i32);
                if *paintkit == desired_paintkit {
                    continue;
                }

                //let game_scene_node = weapon_controller + offsets::client::C_BaseEntity::m_pGameSceneNode as usize;
                //let mesh_group_mask = cast!(mut game_scene_node + 0x160 + offsets::client::CModelState::m_MeshGroupMask, i32);
                //if *mesh_group_mask != -1 {
                //    *mesh_group_mask = -1;
                //}

                *cast!(mut weapon_item + offsets::client::C_EconItemView::m_iItemIDLow, i32) = -1;
                *cast!(mut weapon_item + offsets::client::C_EconItemView::m_iItemIDHigh, i32) = -1;
                *paintkit = desired_paintkit;
            }
        }
    }
    frame_stage_notify(rcx, stage);
}

pub unsafe fn run() {
    let mut time_since_last_shot: Instant = Instant::now() - Duration::from_secs(1);

    while !detect_keypress(EXIT_KEY) {
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

        let health = local_player.get_health().unwrap_or_default();
        if health < 1 {
            continue;
        }

        println!(
            "{}",
            *cast!(
                context.client_module.base_address + offsets::buttons::attack,
                u32
            )
        );

        let attack = cast!(
            mut context.client_module.base_address + offsets::buttons::attack,
            u32
        );

        if detect_keypress(TRIGGERBOT_KEY) {
            if let Some(entity_handle) = local_player.get_handle_of_entity_in_crosshair() {
                if entity_handle != -1 {
                    let now = Instant::now();
                    let should_shoot = now.duration_since(time_since_last_shot)
                        > Duration::from_millis(16)
                        && *attack <= 256;

                    if should_shoot {
                        let _ = context
                            .engine_client_interface
                            .execute_client_command("+attack;-attack");
                        time_since_last_shot = now;
                    }
                }
            }
        }

        // +attack;-attack; fix !!!
        if *attack == 257 {
            *attack = 256;
        }

        const DESIRED_FOV: u32 = 120;

        if !local_player.get_is_scoped().unwrap_or_default() {
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
