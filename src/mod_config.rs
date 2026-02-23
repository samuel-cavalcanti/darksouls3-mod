use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModConfig {
    pub fps: f32,
    pub skip_intro: bool,
    pub center_x: bool,
    pub enable_borderless: bool,
}

impl Default for ModConfig {
    fn default() -> Self {
        Self {
            fps: 60.0,
            skip_intro: false,
            center_x: false,
            enable_borderless: false,
        }
    }
}

impl ModConfig {
    pub fn load_ini_file() -> ModConfig {
        const CONFIG_FILE: &str = "mod_ds3.ini";
        if let Ok(config_str) = std::fs::read_to_string(CONFIG_FILE) {
            eprintln!("{CONFIG_FILE} LOADED successful");

            let config = serini::from_str::<ModConfig>(&config_str);
            match config {
                Ok(c) => {
                    eprintln!("{c:?} LOADED successful");
                    c
                }
                Err(e) => {
                    eprintln!("Unable to parse:{config_str}, Error:{e:?}");

                    ModConfig::default()
                }
            }
        } else {
            eprintln!("unable do load {CONFIG_FILE}");
            ModConfig::default()
        }
    }
}
