use super::*;

pub struct Button {
    pointer: *mut i32,
}

const BUTTON_STATE_UP: i32 = 65537;
const BUTTON_STATE_DOWN: i32 = 256;

impl GameObject for Button {
    fn is_valid(&self) -> bool {
        !self.pointer.is_null() && self.pointer.is_aligned()
    }

    fn get_base_address(&self) -> usize {
        self.pointer as usize
    }
}

impl Button {
    pub fn get_is_down(&self) -> Result<bool> {
        unsafe { Ok(self.get_self::<i32>()? <= BUTTON_STATE_DOWN) }
    }

    pub fn get_is_up(&self) -> Result<bool> {
        Ok(!self.get_is_down()?)
    }

    pub fn down(&self) -> Result<()> {
        if self.get_is_down()? {
            return Ok(());
        }
        unsafe { self.set_self(BUTTON_STATE_DOWN) }
    }

    pub fn up(&self) -> Result<()> {
        if self.get_is_up()? {
            return Ok(());
        }
        unsafe { self.set_self(BUTTON_STATE_UP) }
    }
}

impl Drop for Button {
    fn drop(&mut self) {
        let _ = self.up();
    }
}
