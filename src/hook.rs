use anyhow::{anyhow, Result};
use std::{ffi::c_void, mem::zeroed};

use minhook_sys::MH_OK;
use toy_arms::utils::is_page_executable;
use winapi::{
    shared::minwindef::{DWORD, LPCVOID, LPVOID},
    um::{
        memoryapi::{VirtualProtect, VirtualQuery},
        winnt::{MEMORY_BASIC_INFORMATION, PAGE_EXECUTE},
    },
};

pub fn initialize() -> Result<()> {
    let status = unsafe { minhook_sys::MH_Initialize() };
    if status != MH_OK {
        return Err(anyhow!(format!(
            "minhook initialization error: {:?}",
            status
        )));
    }
    Ok(())
}

pub fn uninitialize() -> Result<()> {
    let status = unsafe { minhook_sys::MH_Uninitialize() };
    if status != MH_OK {
        return Err(anyhow!(format!(
            "minhook uninitialization error: {:?}",
            status
        )));
    }
    Ok(())
}

pub fn enable_hooks() -> Result<()> {
    let status = unsafe { minhook_sys::MH_EnableHook(std::ptr::null_mut()) };
    if status != MH_OK {
        return Err(anyhow!(format!(
            "minhook hooks enabling error: {:?}",
            status
        )));
    }
    Ok(())
}

pub fn disable_hooks() -> Result<()> {
    let status = unsafe { minhook_sys::MH_DisableHook(std::ptr::null_mut()) };
    if status != MH_OK {
        return Err(anyhow!(format!(
            "minhook hooks disabling error: {:?}",
            status
        )));
    }
    Ok(())
}

pub unsafe fn create_hook(target: *mut c_void, detour: *mut c_void) -> Result<*mut c_void> {
    let mut memory_info: MEMORY_BASIC_INFORMATION = zeroed::<MEMORY_BASIC_INFORMATION>();
    VirtualQuery(
        target as LPCVOID,
        &mut memory_info,
        std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
    );
    let is_executable = is_page_executable(&memory_info);
    let mut old_protect = PAGE_EXECUTE;
    let mut new_protect = PAGE_EXECUTE;
    if !is_executable {
        VirtualProtect(
            target as LPVOID,
            std::mem::size_of::<LPVOID>(),
            new_protect,
            &mut old_protect,
        );
    }

    let mut original = std::ptr::null_mut();
    let status = unsafe { minhook_sys::MH_CreateHook(target, detour, &mut original) };
    if status != MH_OK {
        return Err(anyhow!(format!(
            "failed to hook function at {:#0x} status: {:}",
            target as usize, status
        )));
    }
    if !is_executable {
        VirtualProtect(
            target as LPVOID,
            std::mem::size_of::<LPVOID>(),
            old_protect,
            &mut new_protect as *mut DWORD,
        );
    }

    Ok(original)
}
