use super::*;

#[derive(Default)]
pub struct NoFlash;

impl Feature for NoFlash {
    fn run(&mut self, context: &Context) -> cheatlib::Result<()> {
        context.local_player.set_flash_duration(0f32)
    }
}
