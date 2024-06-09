use std::time::Instant;

use super::*;

pub struct Triggerbot {
    last_shot_time: Instant,
}

impl Default for Triggerbot {
    fn default() -> Self {
        Self {
            last_shot_time: Instant::now(),
        }
    }
}

impl Feature for Triggerbot {
    fn run(&mut self, context: &Context) -> cheatlib::Result<()> {
        if !keyboard::detect_keypress(context.config.triggerbot_key) {
            return Ok(());
        }

        if Instant::now().duration_since(self.last_shot_time)
            < context.config.triggerbot_time_between_shots
        {
            return Ok(());
        }

        let crosshair_entity_handle = context
            .local_player
            .get_handle_of_entity_in_crosshair()
            .unwrap_or_default();
        if crosshair_entity_handle <= 0 {
            return Ok(());
        }

        match context
            .cengine_client
            .execute_client_command("+attack;-attack")
        {
            Ok(_) => {
                self.last_shot_time = Instant::now();
                Ok(())
            }
            Err(error) => Err(error),
        }
    }
}
