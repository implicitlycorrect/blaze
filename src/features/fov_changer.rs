use super::*;

pub struct FovChanger;

impl Feature for FovChanger {
    fn get_name(&self) -> &str {
        "Fov changer"
    }

    fn get_is_enabled(&self, config: &Config) -> bool {
        config.features.fov_changer.enabled
    }

    fn run(&mut self, context: &Context) -> cheatlib::Result<()> {
        let is_scoped = context.local_player.get_is_scoped()?;
        if is_scoped {
            return Ok(());
        }

        let camera_services = unsafe { context.local_player.get_camera_services()?.read() };
        let current_fov = camera_services.get_fov()?;
        if current_fov == context.config.features.fov_changer.fov {
            return Ok(());
        }

        camera_services.set_fov(context.config.features.fov_changer.fov)
    }
}
