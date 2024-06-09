use cheatlib::{anyhow, module::Module, Result};
use std::{
    ffi::{c_char, c_void, CString},
    mem, ptr,
};

pub trait Interface {
    fn get_base_address(&self) -> *mut usize;

    fn get_virtual_function<TVirtualFunction>(&self, index: usize) -> Result<TVirtualFunction>
    where
        TVirtualFunction: Sized,
    {
        unsafe {
            let base_address = self.get_base_address();
            if base_address.is_null() {
                return Err(anyhow!("Base address of interface is null"));
            }

            let vtable = *base_address as *mut usize;
            if vtable.is_null() {
                return Err(anyhow!(
                    "Virtual table (vtable) at {:p} is null",
                    base_address
                ));
            }

            let function_pointer = vtable.add(index).read() as *mut TVirtualFunction;
            if function_pointer.is_null() {
                return Err(anyhow!(
                    "Function pointer at {:p} (index {}) in vtable at {:p} is null",
                    function_pointer,
                    index,
                    base_address
                ));
            }

            if !function_pointer.is_aligned() {
                return Err(anyhow!(
                    "Function pointer at {:p} (index {}) in vtable at {:p} is not aligned with TVirtualFunction",
                    function_pointer,
                    index,
                    base_address
                ));
            }

            let function = std::mem::transmute_copy::<*mut TVirtualFunction, TVirtualFunction>(
                &function_pointer,
            );
            Ok(function)
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
