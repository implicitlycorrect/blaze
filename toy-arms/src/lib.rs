pub mod common;

pub mod pattern_scan;

pub mod module;

pub mod utils;

pub mod keyboard;

use common::get_module_handle;
use std::mem::size_of;
use winapi::shared::minwindef::{DWORD, FARPROC, HMODULE};
use winapi::um::libloaderapi::GetProcAddress;
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::psapi::EnumProcessModules;

pub enum TAInternalError {
    GetAllModuleHandlesFailed,
}

pub trait GameObject {
    unsafe fn from_raw(address: *const usize) -> Option<*mut Self>;
}

// #[inline]
// pub fn pattern_scan_specific_range(pattern: &str, start: usize, end: usize) -> Option<*mut u8> {
//     unsafe { boyer_moore_horspool(pattern, start, end) }
// }

/// * `module_name` - name of module that the desired function is in.
/// * `function_name` - name of the function you want
#[inline]
pub unsafe fn get_module_function_address_with_module_name(
    module_name: &str,
    function_name: &str,
) -> Option<FARPROC> {
    let module_handle = match get_module_handle(module_name) {
        Some(e) => e,
        None => return None,
    };
    get_module_function_address(module_handle, function_name)
}

#[inline]
pub unsafe fn get_module_function_address(
    module_handle: HMODULE,
    function_name: &str,
) -> Option<FARPROC> {
    Some(GetProcAddress(
        module_handle,
        common::make_lpcstr(function_name),
    ))
}

fn get_all_module_handles() -> Result<Vec<HMODULE>, TAInternalError> {
    unsafe {
        for size_indice in 3..=10 {
            // Buffer size is size_indice * sizeof(HMODULE) * 100
            let mut module_handles = vec![0 as HMODULE; size_indice * 100];
            // Make a buffer for required_size[out] by zero initializing the DWORD space
            let mut required_size = std::mem::zeroed::<DWORD>();
            // The last parameter is implicitly: &mut required_size as *mut DWORD
            return if EnumProcessModules(
                GetCurrentProcess(),
                module_handles.as_mut_ptr(),
                (module_handles.len() * size_of::<HMODULE>()) as u32,
                &mut required_size,
            ) != 0
            {
                let number_of_handles = required_size as usize / std::mem::size_of::<HMODULE>();
                // If buffer is smaller than required, loop to call EnumProcessModules with bigger buffer
                if size_indice * 100 < number_of_handles {
                    continue;
                }
                Ok(module_handles
                    .iter()
                    .filter(|e| **e != 0 as HMODULE)
                    .map(|e| e.clone())
                    .collect::<Vec<HMODULE>>())
            } else {
                Err(TAInternalError::GetAllModuleHandlesFailed)
            };
        }
        Err(TAInternalError::GetAllModuleHandlesFailed)
    }
}
