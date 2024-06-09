use std::{io::Write, path::Path};

use cheatlib::{keyboard::VirtualKeyCode, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub exit_key: i32,
    pub features: Features,
}

impl Default for Config {
    fn default() -> Self {
        let default_config = Self {
            exit_key: VirtualKeyCode::VK_DELETE,
            features: Features::default(),
        };

        let _ = default_config.write_to_file("config.toml");

        default_config
    }
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Result<Config> {
        let contents = std::fs::read_to_string(file_path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, file_path: P) -> Result<()> {
        let toml = toml::to_string_pretty(&self)?;
        let mut file = std::fs::File::create(file_path)?;
        file.write_all(toml.as_bytes())?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Features {
    pub triggerbot: Triggerbot,
    pub fov_changer: FovChanger,
    pub bunnyhop: Bunnyhop,
    pub no_flash: NoFlash,
}

impl Default for Features {
    fn default() -> Self {
        Self {
            triggerbot: Triggerbot {
                key: VirtualKeyCode::VK_XBUTTON2,
                time_between_shots_ms: 14 * 3,
                enabled: true,
            },
            fov_changer: FovChanger {
                fov: 110,
                enabled: true,
            },
            bunnyhop: Bunnyhop {
                jump_key: VirtualKeyCode::VK_SPACE,
                enabled: true,
            },
            no_flash: NoFlash { enabled: true },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Triggerbot {
    pub key: i32,
    pub time_between_shots_ms: u64,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FovChanger {
    pub fov: u32,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bunnyhop {
    pub jump_key: i32,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NoFlash {
    pub enabled: bool,
}
