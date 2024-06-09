use cheatlib::{anyhow, Result};

pub trait GameObject {
    fn from_raw(pointer: usize) -> Result<*mut Self>
    where
        Self: Sized,
    {
        let pointer = pointer as *mut Self;
        if pointer.is_null() {
            return Err(anyhow!(
                "GameObject from_raw: pointer is null (pointer: {:p})",
                pointer
            ));
        }
        Ok(pointer)
    }

    fn is_valid(&self) -> bool;

    fn get_base_address(&self) -> usize;

    unsafe fn set_self<T>(&self, value: T) -> Result<()> {
        if !self.is_valid() {
            return Err(anyhow!(
                "set_self: invalid GameObject state (base address: {:#x})",
                self.get_base_address()
            ));
        }
        *(self.get_base_address() as *mut T) = value;
        Ok(())
    }

    unsafe fn set_at<T>(&self, address: usize, value: T) -> Result<()> {
        if !self.is_valid() {
            return Err(anyhow!(
                "set_at: invalid GameObject state (base address: {:#x})",
                self.get_base_address()
            ));
        }
        *((self.get_base_address() + address) as *mut T) = value;
        Ok(())
    }

    unsafe fn get_self<T>(&self) -> Result<T>
    where
        T: Clone + Copy,
    {
        if !self.is_valid() {
            return Err(anyhow!(
                "get_self: invalid GameObject state (base address: {:#x})",
                self.get_base_address()
            ));
        }
        Ok(*(self.get_base_address() as *const T))
    }

    unsafe fn get_at<T>(&self, address: usize) -> Result<T>
    where
        T: Clone + Copy,
    {
        if !self.is_valid() {
            return Err(anyhow!(
                "get_at: invalid GameObject state (base address: {:#x})",
                self.get_base_address()
            ));
        }
        let value_pointer = (self.get_base_address() + address) as *const T;
        if value_pointer.is_null() {
            return Err(anyhow!(
                "get_at: address points to null (address: {:p})",
                value_pointer
            ));
        }
        Ok(*value_pointer)
    }

    unsafe fn get_game_object_at<TGameObject: GameObject>(
        &self,
        address: usize,
    ) -> Result<*mut TGameObject> {
        TGameObject::from_raw(self.get_base_address() + address)
    }
}
