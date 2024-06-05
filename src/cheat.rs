use std::{
    ffi::{c_char, c_void, CStr, CString},
    sync::Mutex,
};

use anyhow::{anyhow, Result};
use winapi::um::winuser::VK_DELETE;

use crate::{
    hook, interfaces, offsets,
    sdk::{get_virtual_function, CEngineClient, CSource2Client, LocalPlayer},
};
use lazy_static::lazy_static;
use toy_arms::{cast, keyboard::detect_keypress, module::Module, GameObject};

const EXIT_KEY: i32 = VK_DELETE;

lazy_static! {
    static ref CHEAT_CONTEXT: Mutex<CheatContext> = Mutex::new(CheatContext::default());
}

unsafe fn get_controller_from_handle(entity_list: usize, handle: usize) -> usize {
    let handle = handle & 0x7FFF;
    let list_address = entity_list + 0x8 * (handle >> 0x9) + 0x10;
    let list = *(list_address as *const usize);
    let controller_address = list + 0x78 * (handle & 0x1FF);
    let controller = *(controller_address as *const usize);
    controller
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
        std::thread::sleep(std::time::Duration::from_secs(1));
        ORIGINAL_SET_MODEL = hook::create_hook(
            set_model_address as *mut c_void,
            hook_set_model as *mut c_void,
        )?;
        println!("hooked void SetModel(*const char)");

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

static mut ORIGINAL_SET_MODEL: *mut c_void = std::ptr::null_mut();

unsafe extern "fastcall" fn hook_set_model(rcx: *mut c_void, model: *const c_char) {
    let original: extern "fastcall" fn(*mut c_void, *const c_char) =
        std::mem::transmute(ORIGINAL_SET_MODEL);
    let model_name = CStr::from_ptr(model);
    let model_name = model_name.to_str().unwrap();
    println!("{model_name}");

    if model_name == "weapons/models/knife/knife_default_t/weapon_knife_default_t.vmdl" {
        let new_model =
            CString::new("weapons/models/knife/knife_butterfly/weapon_knife_butterfly.vmdl")
                .unwrap();
        let new_model = new_model.as_ptr();
        original(rcx, new_model);
    } else {
        original(rcx, model);
    }
}

const WEAPON_STATE_HELD: i32 = 2;

static mut ORIGINAL_FRAME_STAGE_NOTIFY: *mut c_void = std::ptr::null_mut();

unsafe extern "fastcall" fn hook_frame_stage_notify(rcx: *mut c_void, stage: i32) {
    let context = CHEAT_CONTEXT.lock().unwrap();
    let original: extern "fastcall" fn(*mut c_void, i32) = std::mem::transmute(ORIGINAL_FRAME_STAGE_NOTIFY);

    if (context.engine_client_interface.get_is_in_game() && context.engine_client_interface.get_is_connected()) && (stage == 5 || stage == 6) {
        // run skin changer
        if let Some(local_player) = context.get_local_player() {
            let entity_list = context.entity_list() as usize;

            if let Some(weapon_services) = local_player.get_weapon_services() {
                if let Some(weapon_size) = weapon_services.get_weapon_size() {
                    for index in 0..weapon_size {
                        let Some(weapon_handle) = weapon_services.get_weapon_handle_at_index(index) else {
                            continue;
                        };
                        let weapon_controller =
                            get_controller_from_handle(entity_list, weapon_handle);
                        let weapon_state = *cast!(
                            weapon_controller + offsets::client::C_CSWeaponBase::m_iState,
                            i32
                        );
                        if weapon_state < WEAPON_STATE_HELD {
                            continue;
                        }
                        // C_EconItemView
                        let weapon_econ_item_view = weapon_controller
                            + offsets::client::C_EconEntity::m_AttributeManager
                            + offsets::client::C_AttributeContainer::m_Item;
                        let weapon_id = *cast!(
                            weapon_econ_item_view
                                + offsets::client::C_EconItemView::m_iItemDefinitionIndex,
                            i32
                        );
                        println!("held weapon id: {weapon_id}");
                    }
                }
            }
        }
    }
    original(rcx, stage);
}

pub fn run() {
    while !detect_keypress(EXIT_KEY) {
        let context = CHEAT_CONTEXT.lock().unwrap();
        if !context.engine_client_interface.get_is_in_game()
            || !context.engine_client_interface.get_is_connected()
        {
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
