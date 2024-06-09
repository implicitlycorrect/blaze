use cheatlib::{anyhow, module::Module, Result};
use std::{
    ffi::{c_char, c_void, CString},
    mem, ptr,
};

pub trait Interface {
    fn get_base_address(&self) -> *mut usize;

    fn get_virtual_function(&self, index: usize) -> Result<*mut usize> {
        unsafe {
            let base_address = self.get_base_address();
            if base_address.is_null() {
                return Err(anyhow!("interface points to null"));
            }
            let vtable = *base_address as *mut usize;
            if vtable.is_null() {
                return Err(anyhow!("vtable points to null"));
            }
            let function_ptr = vtable.add(index).read() as *mut usize;
            if function_ptr.is_null() {
                return Err(anyhow!("function points to null"));
            }
            Ok(function_ptr)
        }
    }

    fn create(module: &Module) -> Result<Self>
    where
        Self: Sized;
}

type CreateInterfaceFn = extern "C" fn(name: *const c_char, rc: *mut i32) -> *mut c_void;

pub(super) fn get_factory(module: &Module) -> Option<CreateInterfaceFn> {
    module
        .get_function_address("CreateInterface")
        .map(|create_interface_fn_address| unsafe {
            mem::transmute::<_, CreateInterfaceFn>(create_interface_fn_address)
        })
}

pub(super) fn create_interface(factory: CreateInterfaceFn, version: &str) -> Option<*mut c_void> {
    let c_version = CString::new(version).unwrap();
    let interface = factory(c_version.as_ptr(), ptr::null_mut());
    if !interface.is_null() {
        return Some(interface);
    }
    None
}
