use super::*;

#[derive(Default)]
pub struct Bunnyhop;

impl Feature for Bunnyhop {
    fn run(&mut self, context: &Context) -> cheatlib::Result<()> {
        let on_ground = context.local_player.get_on_ground()?;
        let jump_held = keyboard::detect_keypress(context.config.bunnyhop_jump_key);
        if !jump_held || !on_ground {
            return Ok(());
        }
        context.cengine_client.execute_client_command("+jump;-jump")
    }
}
