use std::time::Duration;

use cheatlib::keyboard::VirtualKeyCode;

#[derive(Clone, Copy)]
pub struct Config {
    pub exit_key: i32,
    pub bunnyhop_jump_key: i32,
    pub fov_changer_fov: u32,
    pub triggerbot_key: i32,
    pub triggerbot_time_between_shots: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            exit_key: VirtualKeyCode::VK_DELETE,
            bunnyhop_jump_key: VirtualKeyCode::VK_SPACE,
            fov_changer_fov: 110,
            triggerbot_key: VirtualKeyCode::VK_XBUTTON2,
            triggerbot_time_between_shots: Duration::from_millis(14 * 3),
        }
    }
}
