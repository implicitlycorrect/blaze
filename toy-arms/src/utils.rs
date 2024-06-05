use std::str::Utf8Error;

use winapi::um::winnt::{MEMORY_BASIC_INFORMATION, MEM_COMMIT, PAGE_EXECUTE_READWRITE, PAGE_NOACCESS};

pub fn is_page_readable(memory_info: &MEMORY_BASIC_INFORMATION) -> bool {
    if memory_info.State != MEM_COMMIT
        || memory_info.Protect == 0x0
        || memory_info.Protect == PAGE_NOACCESS
    {
        return false;
    }
    true
}

pub fn is_page_executable(memory_info: &MEMORY_BASIC_INFORMATION) -> bool {
    if memory_info.Protect == PAGE_EXECUTE_READWRITE {
        return true;
    }
    false
}

#[inline]
pub unsafe fn read_null_terminated_string(base_address: usize) -> Result<String, Utf8Error> {
    let len = (0..500)
        .take_while(|&i| *(base_address as *const u8).offset(i) != 0)
        .count();
    let slice = std::slice::from_raw_parts(base_address as *const u8, len);

    match String::from_utf8(slice.to_vec()) {
        Ok(val) => Ok(val),
        Err(e) => return Err(e.utf8_error()),
    }
}
