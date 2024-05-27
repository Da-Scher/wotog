// made this because i wanted to use config that can be changed by the user using the CLI application which config-rs does not support.
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub motd: String,
    pub colorized: bool,
    pub homepage: String,
    pub root_dir: std::path::PathBuf,
}

impl Config {
    pub fn new(dir: std::path::PathBuf, colorized: Option<bool>, homepage: Option<String>) -> Config {
        Config {
            motd: String::from("nice stack, buddy."),
            colorized: match colorized {
                Some(c) => c,
                None    => false,
            },
            homepage: match homepage {
                Some(h) => h,
                None    => String::from("localhost:8080"),
            },
            root_dir: dir,
        }
    }
}