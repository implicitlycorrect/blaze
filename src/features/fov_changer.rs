use super::*;

#[derive(Default)]
pub struct FovChanger;

impl Feature for FovChanger {
    fn run(&mut self, context: &Context) -> cheatlib::Result<()> {
        let is_scoped = context.local_player.get_is_scoped()?;
        if is_scoped {
            return Ok(());
        }

        let camera_services = unsafe { context.local_player.get_camera_services()?.read() };
        let current_fov = camera_services.get_fov()?;
        if current_fov == context.config.fov_changer_fov {
            return Ok(());
        }

        camera_services.set_fov(context.config.fov_changer_fov)
    }
}
