use cheatlib::*;

type CreateInterfaceFn = extern "C" fn(name: *const c_char, rc: *mut i32) -> *mut c_void;

pub fn get_factory(module: &Module) -> Option<CreateInterfaceFn> {
    module
        .get_function_address("CreateInterface")
        .map(|create_interface_fn_address| unsafe {
            mem::transmute::<_, CreateInterfaceFn>(create_interface_fn_address)
        })
}

pub fn get_interface(factory: CreateInterfaceFn, version: &str) -> Option<*mut c_void> {
    let c_version = CString::new(version).unwrap();
    let interface = factory(c_version.as_ptr(), ptr::null_mut());
    if !interface.is_null() {
        return Some(interface);
    }
    None
}
