use super::*;

pub struct Bunnyhop;

impl Feature for Bunnyhop {
    fn get_name(&self) -> &str {
        "Bunnyhop"
    }

    fn get_is_enabled(&self, config: &Config) -> bool {
        config.features.bunnyhop.enabled
    }

    fn run(&mut self, context: &Context) -> cheatlib::Result<()> {
        let on_ground = context.local_player.get_on_ground()?;
        let jump_held = keyboard::detect_keypress(context.config.features.bunnyhop.jump_key);
        if !jump_held || !on_ground {
            return Ok(());
        }
        context.cengine_client.execute_client_command("+jump;-jump")
    }
}
