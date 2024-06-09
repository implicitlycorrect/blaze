use super::*;

pub struct LocalPlayer {
    pointer: *mut LocalPlayer,
}

impl GameObject for LocalPlayer {
    fn is_valid(&self) -> bool {
        !self.pointer.is_null()
    }

    fn get_base_address(&self) -> usize {
        self.pointer as usize
    }
}

impl LocalPlayer {
    pub fn empty() -> Self {
        Self {
            pointer: ptr::null_mut(),
        }
    }

    pub fn set_flash_duration(&self, duration: f32) -> Result<()> {
        unsafe {
            self.set_at(
                offsets::client::C_CSPlayerPawnBase::m_flFlashDuration,
                duration,
            )
        }
    }

    pub fn get_on_ground(&self) -> Result<bool> {
        let flags = unsafe { self.get_at::<u32>(offsets::client::C_BaseEntity::m_fFlags)? };
        let on_ground = (flags & (1 << 0)) > 0;
        Ok(on_ground)
    }

    pub fn get_is_scoped(&self) -> Result<bool> {
        unsafe { self.get_at(offsets::client::C_CSPlayerPawn::m_bIsScoped) }
    }

    pub fn get_camera_services(&self) -> Result<*mut CameraServices> {
        unsafe { self.get_game_object_at(offsets::client::C_BasePlayerPawn::m_pCameraServices) }
    }

    pub fn get_handle_of_entity_in_crosshair(&self) -> Result<i32> {
        unsafe { self.get_at(offsets::client::C_CSPlayerPawnBase::m_iIDEntIndex) }
    }
}
