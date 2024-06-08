use cheatlib::*;

use super::offsets;

pub fn get_virtual_function(base: *mut usize, index: usize) -> *mut usize {
    unsafe {
        let vt = *base as *mut usize;
        vt.add(index).read() as *mut usize
    }
}

pub struct LocalPlayer {
    pointer: *const usize,
}

impl LocalPlayer {
    pub unsafe fn from_raw(pointer: *mut usize) -> Option<*mut LocalPlayer> {
        let local_player_pointer = pointer as *mut LocalPlayer;
        if local_player_pointer.is_null() {
            return None;
        }
        Some(local_player_pointer)
    }

    pub fn get_is_scoped(&self) -> Option<bool> {
        if self.pointer.is_null() {
            return None;
        }

        let is_scoped_pointer =
            (self.pointer as usize + offsets::client::C_CSPlayerPawn::m_bIsScoped) as *const bool;
        if is_scoped_pointer.is_null() {
            return None;
        }

        Some(unsafe { *is_scoped_pointer })
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

    pub fn get_handle_of_entity_in_crosshair(&self) -> Option<i32> {
        if self.pointer.is_null() {
            return None;
        }

        let entity_index_pointer = (self.pointer as usize
            + offsets::client::C_CSPlayerPawnBase::m_iIDEntIndex)
            as *const i32;
        if entity_index_pointer.is_null() {
            return None;
        }
        Some(unsafe { *entity_index_pointer })
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

pub struct CameraServices {
    pointer: *const usize,
}

impl CameraServices {
    pub unsafe fn from_raw(pointer: *const usize) -> Option<*const CameraServices> {
        let camera_services_pointer = pointer as *const CameraServices;
        if camera_services_pointer.is_null() {
            return None;
        }
        Some(camera_services_pointer)
    }

    pub fn set_fov(&self, desired_fov: u32) {
        if self.pointer.is_null() {
            return;
        }

        let fov_pointer = (self.pointer as usize
            + offsets::client::CCSPlayerBase_CameraServices::m_iFOV)
            as *mut u32;
        if fov_pointer.is_null() {
            return;
        }
        unsafe {
            *fov_pointer = desired_fov;
        }
    }

    pub fn get_fov(&self) -> Option<u32> {
        if self.pointer.is_null() {
            return None;
        }

        let fov_pointer = (self.pointer as usize
            + offsets::client::CCSPlayerBase_CameraServices::m_iFOV)
            as *const u32;

        if fov_pointer.is_null() {
            return None;
        }

        Some(unsafe { *fov_pointer })
    }
}
