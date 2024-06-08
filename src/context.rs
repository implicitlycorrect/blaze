use cheatlib::*;

use crate::{
    offsets,
    sdk::{CEngineClient, LocalPlayer},
};

pub struct CheatContext {
    pub client_module: Module,
    pub engine2_module: Module,
    pub engine_client_interface: CEngineClient,
}

impl CheatContext {
    pub fn default() -> Self {
        Self {
            client_module: Module::default(),
            engine2_module: Module::default(),
            engine_client_interface: CEngineClient::default(),
        }
    }

    pub fn get_local_player(&self) -> Option<LocalPlayer> {
        match self.client_module.read(offsets::client::dwLocalPlayerPawn) {
            Ok(local_player_raw_pointer) => {
                Some(unsafe { LocalPlayer::from_raw(local_player_raw_pointer)?.read() })
            }
            _ => None,
        }
    }
}

unsafe impl Sync for CheatContext {}
unsafe impl Send for CheatContext {}
