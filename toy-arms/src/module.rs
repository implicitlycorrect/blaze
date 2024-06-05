use crate::{cast, utils};
use crate::common::get_module_handle;
use anyhow::Result;
use std::mem::{size_of, zeroed};
use std::ptr::copy_nonoverlapping;
use std::str::Utf8Error;
use utils::{is_page_readable, read_null_terminated_string};
use winapi::shared::minwindef::{DWORD, HMODULE, LPCVOID, LPVOID, MAX_PATH};
use winapi::um::memoryapi::{VirtualProtect, VirtualQuery};
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::psapi::{GetModuleBaseNameA, GetModuleInformation, MODULEINFO};
use winapi::um::winnt::{CHAR, LPSTR, MEMORY_BASIC_INFORMATION, PAGE_READWRITE};

#[derive(Debug)]
pub struct Module {
    pub handle: HMODULE,
    pub size: u32,
    pub base_address: usize,
    pub data: Vec<u8>,
}

impl Default for Module {
    fn default() -> Self {
        Module {
            handle: 0x0 as HMODULE,
            size: 0,
            base_address: 0,
            data: vec![0u8; 80000000],
        }
    }
}

fn get_module_info(module: HMODULE) -> MODULEINFO {
    return unsafe {
        let mut module_info: MODULEINFO = zeroed::<MODULEINFO>();
        GetModuleInformation(
            GetCurrentProcess(),
            module,
            &mut module_info,
            size_of::<MODULEINFO>() as u32,
        );
        module_info
    };
}

fn get_module_data(module_info: MODULEINFO) -> Vec<u8> {
    unsafe {
        let mut data: Vec<u8> = Vec::with_capacity(module_info.SizeOfImage as usize);
        let data_ptr = data.as_mut_ptr();
        data.set_len(0);
        copy_nonoverlapping(
            module_info.lpBaseOfDll as *const u8,
            data_ptr,
            module_info.SizeOfImage as usize,
        );
        data.set_len(module_info.SizeOfImage as usize);
        data
    }
}

impl Module {
    pub fn from_name(module_name: &str) -> Option<Self> {
        let module = get_module_handle(module_name)?;
        Self::from_handle(module)
    }

    pub fn get_name(&self) -> Result<String, Utf8Error> {
        let mut name_buffer: [CHAR; MAX_PATH] = [0; MAX_PATH];
        unsafe {
            GetModuleBaseNameA(
                GetCurrentProcess(),
                self.handle,
                &mut name_buffer as LPSTR,
                std::mem::size_of_val(&name_buffer) as u32,
            );

            read_null_terminated_string(&mut name_buffer as *mut i8 as usize)
        }
    }

    pub fn from_handle(module: HMODULE) -> Option<Self> {
        let module_info = get_module_info(module);
        let data = get_module_data(module_info);

        let module = Module {
            handle: module,
            base_address: module_info.lpBaseOfDll as usize,
            size: module_info.SizeOfImage,
            data,
        };
        Some(module)
    }

    /// read fetches the value that given address is holding.
    /// * `base_address` - the address that is supposed to have the value you want
    #[inline]
    pub fn read<T>(&self, address: usize) -> *mut T {
        let mut memory_info: MEMORY_BASIC_INFORMATION = unsafe { zeroed::<MEMORY_BASIC_INFORMATION>() };
        unsafe {
            VirtualQuery(
                (self.base_address + address) as LPCVOID,
                &mut memory_info,
                std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
            );
        }
        let is_readable = is_page_readable(&memory_info);
        let mut old_protect = PAGE_READWRITE;
        let mut new_protect = PAGE_READWRITE;
        if !is_readable {
            unsafe {
                VirtualProtect(
                    (self.base_address + address) as LPVOID,
                    std::mem::size_of::<LPVOID>(),
                    new_protect,
                    &mut old_protect as *mut DWORD,
                );
            }
        }
        let result = cast!(mut self.base_address + address, T);
        if !is_readable {
            unsafe {
                VirtualProtect(
                    (self.base_address + address) as LPVOID,
                    std::mem::size_of::<LPVOID>(),
                    old_protect,
                    &mut new_protect as *mut DWORD,
                );
            }
        }
        result
    }

    /// read_string reads the string untill the null terminator that is in the given module
    /// * `address` - relative address of the head of the string.
    #[inline]
    pub fn read_string(&self, address: i32) -> Result<std::string::String, Utf8Error> {
        unsafe { read_null_terminated_string(self.handle as usize + address as usize) }
    }
}
