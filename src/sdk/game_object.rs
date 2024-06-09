use cheatlib::{anyhow, Result};

pub trait GameObject {
    fn from_raw(pointer: usize) -> Result<*mut Self>
    where
        Self: Sized,
    {
        let pointer = pointer as *mut Self;
        if pointer.is_null() {
            return Err(anyhow!("GameObject points to null"));
        }
        Ok(pointer)
    }

    fn is_valid(&self) -> bool;

    fn get_base_address(&self) -> usize;

    unsafe fn set_at<T>(&self, address: usize, value: T) -> Result<()> {
        if !self.is_valid() {
            return Err(anyhow!("invalid GameObject state"));
        }
        *((self.get_base_address() + address) as *mut T) = value;
        Ok(())
    }

    unsafe fn get_at<T>(&self, address: usize) -> Result<T>
    where
        T: Clone + Copy,
    {
        if !self.is_valid() {
            return Err(anyhow!("invalid GameObject state"));
        }
        let value_pointer = (self.get_base_address() + address) as *const T;
        if value_pointer.is_null() {
            return Err(anyhow!("address points to null"));
        }
        Ok(*value_pointer)
    }

    unsafe fn get_game_object_at<TGameObject: GameObject>(&self, address: usize) -> Result<*mut TGameObject> {
        TGameObject::from_raw(self.get_base_address() + address)
    }
}
