pub use cheatlib::*;

use crate::{config::Config, context::Context};

pub mod bunnyhop;
pub mod fov_changer;
pub mod no_flash;
pub mod triggerbot;

pub trait Feature {
    fn get_name(&self) -> &str;
    fn get_is_enabled(&self, config: &Config) -> bool;
    fn run(&mut self, context: &Context) -> Result<()>;
    fn undo(&mut self, context: &Context) -> Result<()>;
}
