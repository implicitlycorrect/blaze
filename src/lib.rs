use winapi::{
    shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE},
    um::{
        consoleapi::AllocConsole,
        wincon::{FreeConsole, SetConsoleTitleA},
        winnt::DLL_PROCESS_ATTACH,
    },
};

mod cheat;
mod hook;
mod interfaces;
mod offsets;
mod sdk;

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: DWORD, _reserved: LPVOID) -> BOOL {
    if call_reason == DLL_PROCESS_ATTACH {
        std::thread::spawn(|| unsafe {
            AllocConsole();
            SetConsoleTitleA(b"Blaze\0".as_ptr() as _);

            if let Err(error) = cheat::initialize() {
                eprintln!("Failed to initialize cheat {:?}", error);
            } else {
                cheat::run();
            }

            cheat::uninitialize();
            FreeConsole();
        });
    }
    TRUE
}
