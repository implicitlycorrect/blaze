use cheatlib::{anyhow, Result};

pub trait GameObject
where
    Self: Sized,
{
    fn from_raw(pointer: usize) -> Result<*mut Self> {
        let pointer = pointer as *mut Self;
        if pointer.is_null() || !pointer.is_aligned() {
            return Err(anyhow!(
                "GameObject::from_raw: pointer {:p} is null or not aligned",
                pointer
            ));
        }
        Ok(pointer)
    }

    fn is_valid(&self) -> bool;

    fn get_base_address(&self) -> usize;

    unsafe fn set_self<T>(&self, value: T) -> Result<()> {
        self.internal_set_at_address(self.get_base_address(), value)
    }

    unsafe fn set_at<T>(&self, address: usize, value: T) -> Result<()> {
        self.internal_set_at_address(self.get_base_address() + address, value)
    }

    unsafe fn get_self<T>(&self) -> Result<T>
    where
        T: Copy,
    {
        self.internal_get_at_address(self.get_base_address())
    }

    unsafe fn get_game_object_at<TGameObject: GameObject>(
        &self,
        address: usize,
    ) -> Result<*mut TGameObject> {
        GameObject::from_raw(self.get_base_address() + address)
    }

    unsafe fn get_at<T>(&self, address: usize) -> Result<T>
    where
        T: Copy,
    {
        self.internal_get_at_address(self.get_base_address() + address)
    }

    // Internal helper methods
    unsafe fn internal_set_at_address<T>(&self, address: usize, value: T) -> Result<()> {
        if !self.is_valid() {
            return Err(anyhow!(
                "GameObject::internal_set_at_address: invalid state ({:#0x})",
                self.get_base_address()
            ));
        }
        let pointer = address as *mut T;
        if pointer.is_null() || !pointer.is_aligned() {
            return Err(anyhow!(
                "GameObject::internal_set_at_address: pointer {:p} is not aligned with type T",
                pointer
            ));
        }
        *pointer = value;
        Ok(())
    }

    unsafe fn internal_get_at_address<T>(&self, address: usize) -> Result<T>
    where
        T: Copy,
    {
        if !self.is_valid() {
            return Err(anyhow!(
                "GameObject::internal_get_at_address: invalid state ({:#0x})",
                self.get_base_address()
            ));
        }
        let pointer = address as *const T;
        if pointer.is_null() || !pointer.is_aligned() {
            return Err(anyhow!(
                "GameObject::internal_get_at_address: address {:p} does not point to a properly aligned value",
                pointer
            ));
        }
        Ok(*pointer)
    }
}
