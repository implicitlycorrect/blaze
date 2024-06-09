use super::*;

pub struct NoFlash;

impl Feature for NoFlash {
    fn get_name(&self) -> &str {
        "No flash"
    }

    fn get_is_enabled(&self, config: &Config) -> bool {
        config.features.no_flash.enabled
    }

    fn run(&mut self, context: &Context) -> cheatlib::Result<()> {
        context.local_player.set_flash_duration(0f32)
    }
}
