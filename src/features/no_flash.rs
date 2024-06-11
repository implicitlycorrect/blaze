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
        context.get_local_player()?.set_flash_duration(0f32)
    }

    fn undo(&mut self, context: &Context) -> Result<()> {
        const DEFAULT_FLASH_DURATION: f32 = 4.87f32;

        context
            .get_local_player()?
            .set_flash_duration(DEFAULT_FLASH_DURATION)
    }
}
