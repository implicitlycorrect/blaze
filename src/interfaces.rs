use std::{
    ffi::{c_char, c_void, CString},
    mem, ptr,
};
use toy_arms::get_module_function_address;
use winapi::shared::minwindef::HMODULE;

type CreateInterfaceFn = extern "C" fn(name: *const c_char, rc: *mut i32) -> *mut c_void;

pub fn get_factory(module: HMODULE) -> CreateInterfaceFn {
    unsafe {
        mem::transmute::<_, CreateInterfaceFn>(
            get_module_function_address(module, "CreateInterface").unwrap(),
        )
    }
}

pub fn get_interface(factory: CreateInterfaceFn, version: &str) -> Option<*mut c_void> {
    let c_version = CString::new(version).unwrap();
    let interface = factory(c_version.as_ptr(), ptr::null_mut());
    if !interface.is_null() {
        return Some(interface);
    }
    None
}
