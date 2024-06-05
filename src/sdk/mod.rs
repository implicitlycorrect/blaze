use std::ffi::{c_char, CString};

use toy_arms::{cast, GameObject};
use toy_arms_derive::GameObject;

use super::offsets;

pub fn get_virtual_function(base: *mut usize, index: usize) -> *mut usize {
    unsafe {
        let vt = *base as *mut usize;
        vt.add(index).read() as *mut usize
    }
}

#[derive(GameObject)]
pub struct LocalPlayer {
    pointer: *const usize,
}

impl LocalPlayer {
    pub unsafe fn get_health(&self) -> i32 {
        *cast!(
            self.pointer as usize + offsets::client::C_BaseEntity::m_iHealth,
            i32
        )
    }

    pub unsafe fn get_is_scoped(&self) -> bool {
        *cast!(
            self.pointer as usize + offsets::client::C_CSPlayerPawn::m_bIsScoped,
            bool
        )
    }

    pub unsafe fn get_camera_services(&self) -> Option<CameraServices> {
        let camera_services_ptr = CameraServices::from_raw(cast!(
            self.pointer as usize + offsets::client::C_BasePlayerPawn::m_pCameraServices,
            usize
        ))?;
        let camera_services = camera_services_ptr.read();
        Some(camera_services)
    }
}

pub struct CEngineClient {
    pub base: *mut usize,
}

unsafe impl Send for CEngineClient {}
unsafe impl Sync for CEngineClient {}

impl CEngineClient {
    pub fn default() -> Self {
        Self {
            base: std::ptr::null_mut(),
        }
    }

    pub unsafe fn get_is_in_game(&self) -> bool {
        type GetInGameFn = unsafe extern "thiscall" fn(*mut usize) -> bool;
        unsafe {
            std::mem::transmute::<_, GetInGameFn>(get_virtual_function(self.base, 34))(self.base)
        }
    }

    pub unsafe fn get_is_connected(&self) -> bool {
        type GetIsConnectedFn = unsafe extern "thiscall" fn(*mut usize) -> bool;
        unsafe {
            std::mem::transmute::<_, GetIsConnectedFn>(get_virtual_function(self.base, 35))(
                self.base,
            )
        }
    }

    pub unsafe fn execute_client_command(&self, command: &str) {
        type ExecuteClientCmdFn = unsafe extern "thiscall" fn(*mut usize, i32, *const c_char, bool);
        unsafe {
            let command = CString::new(command).unwrap();
            let command_pointer = command.as_ptr();
            std::mem::transmute::<_, ExecuteClientCmdFn>(get_virtual_function(self.base, 43))(
                self.base,
                0,
                command_pointer,
                true,
            );
        }
    }
}

#[derive(GameObject)]
pub struct CameraServices {
    pointer: *const usize,
}

impl CameraServices {
    pub unsafe fn set_fov(&self, desired_fov: i32) {
        *cast!(mut self.pointer as usize + offsets::client::CCSPlayerBase_CameraServices::m_iFOV, i32) =
            desired_fov;
    }

    pub unsafe fn get_fov(&self) -> i32 {
        *cast!(
            self.pointer as usize + offsets::client::CCSPlayerBase_CameraServices::m_iFOV,
            i32
        )
    }
}
