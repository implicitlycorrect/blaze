use std::ffi::{c_char, CString};

use anyhow::{anyhow, Result};
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
    pub fn get_health(&self) -> Option<u32> {
        if self.pointer.is_null() {
            return None;
        }

        Some(unsafe {
            *cast!(
                self.pointer as usize + offsets::client::C_BaseEntity::m_iHealth,
                u32
            )
        })
    }

    pub fn get_team_number(&self) -> Option<u32> {
        if self.pointer.is_null() {
            return None;
        }

        Some(unsafe {
            *cast!(
                self.pointer as usize + offsets::client::C_BaseEntity::m_iTeamNum,
                u32
            )
        })
    }

    pub fn get_weapon_services(&self) -> Option<WeaponServices> {
        if self.pointer.is_null() {
            return None;
        }

        Some(unsafe {
            WeaponServices::from_raw(
                (self.pointer as usize + offsets::client::C_BasePlayerPawn::m_pWeaponServices)
                    as *const usize,
            )?
            .read()
        })
    }

    pub fn get_viewmodel_services(&self) -> Option<ViewModelServices> {
        if self.pointer.is_null() {
            return None;
        }

        Some(unsafe {
            ViewModelServices::from_raw(
                (self.pointer as usize + offsets::client::C_CSPlayerPawnBase::m_pViewModelServices)
                    as *const usize,
            )?
            .read()
        })
    }

    pub fn get_is_scoped(&self) -> Option<bool> {
        if self.pointer.is_null() {
            return None;
        }

        Some(unsafe {
            *cast!(
                self.pointer as usize + offsets::client::C_CSPlayerPawn::m_bIsScoped,
                bool
            )
        })
    }

    pub fn get_camera_services(&self) -> Option<CameraServices> {
        if self.pointer.is_null() {
            return None;
        }

        Some(unsafe {
            CameraServices::from_raw(
                (self.pointer as usize + offsets::client::C_BasePlayerPawn::m_pCameraServices)
                    as *const usize,
            )?
            .read()
        })
    }
}

pub struct CSource2Client {
    pub base: *mut usize,
}

unsafe impl Send for CSource2Client {}
unsafe impl Sync for CSource2Client {}

impl CSource2Client {
    pub fn default() -> Self {
        Self {
            base: std::ptr::null_mut(),
        }
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

    pub fn get_is_in_game(&self) -> Option<bool> {
        if self.base.is_null() {
            return None;
        }

        type GetInGameFn = unsafe extern "thiscall" fn(*mut usize) -> bool;
        Some(unsafe {
            std::mem::transmute::<_, GetInGameFn>(get_virtual_function(self.base, 34))(self.base)
        })
    }

    pub fn get_is_connected(&self) -> Option<bool> {
        if self.base.is_null() {
            return None;
        }

        type GetIsConnectedFn = unsafe extern "thiscall" fn(*mut usize) -> bool;
        Some(unsafe {
            std::mem::transmute::<_, GetIsConnectedFn>(get_virtual_function(self.base, 35))(
                self.base,
            )
        })
    }

    pub fn execute_client_command(&self, command: &str) -> Result<()> {
        if self.base.is_null() {
            return Err(anyhow!("*CEngineClient points to null"));
        }

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

        Ok(())
    }
}

#[derive(GameObject)]
pub struct ViewModelServices {
    pointer: *const usize,
}

impl ViewModelServices {
    pub fn get_viewmodel_handle(&self) -> Option<u32> {
        if self.pointer.is_null() {
            return None;
        }

        Some(unsafe {
            *cast!(
                self.pointer as usize + offsets::client::CCSPlayer_ViewModelServices::m_hViewModel,
                u32
            )
        })
    }
}

#[derive(GameObject)]
pub struct WeaponServices {
    pointer: *const usize,
}

impl WeaponServices {
    pub fn get_weapon_size(&self) -> Option<usize> {
        if self.pointer.is_null() {
            return None;
        }

        Some(unsafe { *cast!(self.pointer as usize + 0x50, usize) })
    }

    pub fn get_weapon_handle_at_index(&self, index: usize) -> Option<i32> {
        if self.pointer.is_null() {
            return None;
        }

        Some(unsafe {
            *cast!(
                (self.pointer as usize + offsets::client::CPlayer_WeaponServices::m_hActiveWeapon)
                    + 0x4 * index,
                i32
            )
        })
    }
}

#[derive(GameObject)]
pub struct CameraServices {
    pointer: *const usize,
}

impl CameraServices {
    pub fn set_fov(&self, desired_fov: u32) {
        if self.pointer.is_null() {
            return;
        }

        unsafe {
            let current_fov = cast!(mut self.pointer as usize + offsets::client::CCSPlayerBase_CameraServices::m_iFOV, u32);
            *current_fov = desired_fov;
        }
    }

    pub fn get_fov(&self) -> u32 {
        if self.pointer.is_null() {
            return 0;
        }

        unsafe {
            *cast!(
                self.pointer as usize + offsets::client::CCSPlayerBase_CameraServices::m_iFOV,
                u32
            )
        }
    }
}
