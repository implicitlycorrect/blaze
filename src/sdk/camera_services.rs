use super::*;

pub struct CameraServices {
    pointer: *mut CameraServices,
}

impl GameObject for CameraServices {
    fn is_valid(&self) -> bool {
        !self.pointer.is_null()
    }

    fn get_base_address(&self) -> usize {
        self.pointer as usize
    }
}

impl CameraServices {
    pub fn set_fov(&self, desired_fov: u32) -> Result<()> {
        unsafe {
            self.set_at(
                offsets::client::CCSPlayerBase_CameraServices::m_iFOV,
                desired_fov,
            )
        }
    }

    pub fn get_fov(&self) -> Result<u32> {
        unsafe { self.get_at(offsets::client::CCSPlayerBase_CameraServices::m_iFOV) }
    }
}
