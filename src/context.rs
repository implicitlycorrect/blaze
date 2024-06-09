use cheatlib::*;

use crate::{offsets, sdk::*};

pub struct Context {
    pub client: Module,
    pub engine: Module,
    pub cengine_client: CEngineClient,
}

impl Context {
    pub fn create() -> Result<Self> {
        let client = Module::from_name("client.dll")?;
        let engine = Module::from_name("engine2.dll")?;
        let cengine_client = CEngineClient::create(&engine)?;
        Ok(Self {
            client,
            engine,
            cengine_client,
        })
    }

    pub fn get_local_player(&self) -> Result<LocalPlayer> {
        let local_player_pointer =
            LocalPlayer::from_raw(self.client.base_address + offsets::client::dwLocalPlayerPawn)?;
        Ok(unsafe { local_player_pointer.read() })
    }
}
