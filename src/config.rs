use std::time::Duration;

use cheatlib::keyboard::VirtualKeyCode;

pub const EXIT_KEY: i32 = VirtualKeyCode::VK_DELETE;
pub const DESIRED_FOV: u32 = 120;
pub const TRIGGERBOT_KEY: i32 = VirtualKeyCode::VK_XBUTTON2;
pub const TRIGGERBOT_TIME_BETWEEN_SHOTS: Duration = Duration::from_millis(14 * 3);
pub const BHOP_KEY: i32 = VirtualKeyCode::VK_SPACE;
