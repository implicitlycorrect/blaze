use super::*;

pub struct CEngineClient {
    base: *mut usize,
}

impl Interface for CEngineClient {
    fn get_base_address(&self) -> *mut usize {
        self.base
    }

    fn create(module: &cheatlib::Module) -> Result<Self> {
        let create_interface_fn = interfaces::get_factory(module).ok_or(anyhow!(
            "failed to get CreateInterface function from provided module"
        ))?;
        let interface_pointer =
            interfaces::create_interface(create_interface_fn, "Source2EngineToClient001")
                .ok_or(anyhow!("failed to create CEngineClient interface"))?;

        Ok(Self {
            base: interface_pointer.cast(),
        })
    }
}

impl CEngineClient {
    pub fn get_is_in_game(&self) -> Result<bool> {
        todo!("doesn't work for debug builds?")

        /*
        type GetIsInGame = unsafe extern "thiscall" fn(*mut usize) -> bool;
        Ok(unsafe {
            mem::transmute::<*mut usize, GetIsInGame>(self.get_virtual_function(34)?)(self.base)
        })
        */
    }

    pub fn get_is_connected(&self) -> Result<bool> {
        type GetIsConnectedFn = unsafe extern "thiscall" fn(*mut usize) -> bool;
        Ok(unsafe {
            mem::transmute::<*mut usize, GetIsConnectedFn>(self.get_virtual_function(35)?)(
                self.base,
            )
        })
    }

    pub fn execute_client_command(&self, command: &str) -> Result<()> {
        type ExecuteClientCmdFn = unsafe extern "thiscall" fn(*mut usize, i32, *const c_char, bool);
        unsafe {
            let command = CString::new(command).unwrap();
            let command_pointer = command.as_ptr();
            mem::transmute::<*mut usize, ExecuteClientCmdFn>(self.get_virtual_function(43)?)(
                self.base,
                0,
                command_pointer,
                true,
            );
        }

        Ok(())
    }
}
