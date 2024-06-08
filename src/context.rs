use cheatlib::*;

use crate::{
    offsets,
    sdk::{CEngineClient, LocalPlayer},
};

pub struct CheatContext {
    pub client: Module,
    pub engine: Module,
    pub cengine_client: CEngineClient,
}

impl CheatContext {
    pub fn default() -> Self {
        Self {
            client: Module::default(),
            engine: Module::default(),
            cengine_client: CEngineClient::default(),
        }
    }

    pub fn get_local_player(&self) -> Option<LocalPlayer> {
        match self.client.read(offsets::client::dwLocalPlayerPawn) {
            Ok(local_player_raw_pointer) => {
                Some(unsafe { LocalPlayer::from_raw(local_player_raw_pointer)?.read() })
            }
            _ => None,
        }
    }
}

unsafe impl Sync for CheatContext {}
unsafe impl Send for CheatContext {}
