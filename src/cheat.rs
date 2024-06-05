use std::{
    f64::NAN,
    ffi::{c_void, CStr, CString},
    sync::Mutex,
};

use anyhow::{anyhow, Result};
use winapi::um::winuser::{VK_DELETE, VK_RSHIFT};

use crate::{
    hook, interfaces, offsets,
    sdk::{CEngineClient, LocalPlayer},
};
use lazy_static::lazy_static;
use toy_arms::{keyboard::detect_keypress, module::Module, GameObject};

const EXIT_KEY: i32 = VK_DELETE;

lazy_static! {
    static ref CHEAT_CONTEXT: Mutex<CheatContext> = Mutex::new(CheatContext::default());
}

struct CheatContext {
    client_base: usize,
    engine2_base: usize,
    engine_client: CEngineClient,
}

impl CheatContext {
    fn default() -> Self {
        Self {
            client_base: 0,
            engine2_base: 0,
            engine_client: CEngineClient::default(),
        }
    }

    unsafe fn get_local_player(&self) -> Option<LocalPlayer> {
        let address = (self.client_base + offsets::client::dwLocalPlayerPawn) as *const usize;
        if address as usize == 0 {
            return None;
        }
        let address = LocalPlayer::from_raw(address)?;
        if address.is_null() {
            return None;
        }
        let local_player = address.read();
        Some(local_player)
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
    context.client_base = client.base_address;
    println!("loaded client.dll {:#0x}", context.client_base);

    let Some(engine2) = Module::from_name("engine2.dll") else {
        return Err(anyhow!("Failed to get engine2 module"));
    };
    context.engine2_base = engine2.base_address;
    println!("loaded engine2.dll {:#0x}", context.engine2_base);

    let module_factory = interfaces::get_factory(engine2.handle);
    let Some(source2_engine_to_client_interface) =
        interfaces::get_interface(module_factory, "Source2EngineToClient001")
    else {
        return Err(anyhow!(
            "Failed to get engine2.Source2EngineToClient001 interface"
        ));
    };
    context.engine_client.base = source2_engine_to_client_interface as *mut usize;

    println!("hooking functions!");
    std::thread::sleep(std::time::Duration::from_millis(400));

    let Some(execute_command_direct) =
        engine2.find_pattern("40 53 55 56 57 48 81 EC ? ? ? ? 41 8B E9")
    else {
        return Err(anyhow!(
            "unable to find execute command direct in engine2 using provided signature"
        ));
    };

    unsafe {
        EXECUTE_CLIENT_CMD_DIRECT = hook::create_hook(
            (engine2.base_address + execute_command_direct) as *mut c_void,
            hook_execute_client_cmd_direct as *mut c_void,
        )?;
    }

    hook::enable_hooks()
}

static mut EXECUTE_CLIENT_CMD_DIRECT: *mut c_void = std::ptr::null_mut();

type ExecuteClientCmdDirect = extern "fastcall" fn(
    a1: i64,
    a2: i32,
    a3: *const std::ffi::c_char,
    a4: i32,
    a5: i32,
    a6: i64,
    a7: i8,
    a8: f64,
);

unsafe extern "fastcall" fn hook_execute_client_cmd_direct(
    a1: i64,
    a2: i32,
    a3: *const std::ffi::c_char,
    a4: i32,
    a5: i32,
    a6: i64,
    a7: i8,
    a8: f64,
) {
    let original: ExecuteClientCmdDirect = std::mem::transmute(EXECUTE_CLIENT_CMD_DIRECT);
    original(a1, a2, a3, a4, a5, a6, a7, a8);

    let command = CStr::from_ptr(a3);
    println!(
        "ExecuteClientCmdDirect({:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?});",
        a1,
        a2,
        command.to_str().unwrap().trim_end(),
        a4,
        a5,
        a6,
        a7,
        a8
    );
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

        if detect_keypress(VK_RSHIFT) {
            let execute_client_cmd_direct: ExecuteClientCmdDirect =
                std::mem::transmute(EXECUTE_CLIENT_CMD_DIRECT);
            let command: CString = CString::new("connect mirage.epidemic.gg").unwrap();
            execute_client_cmd_direct(140714815246160, 0, command.as_ptr(), 0, 0, 0, 0, NAN);
        }

        let health = local_player.get_health();
        if health < 1 {
            continue;
        }

        const DESIRED_FOV: i32 = 120;

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
