pub use cheatlib::*;

use crate::context::Context;

pub mod bunnyhop;
pub mod fov_changer;
pub mod no_flash;
pub mod triggerbot;

pub trait Feature {
    fn run(&mut self, context: &Context) -> Result<()>;
}
