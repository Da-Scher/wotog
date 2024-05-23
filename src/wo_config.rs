// made this because i wanted to use config that can be changed by the user using the CLI application which config-rs does not support.

struct Config {
    pub motd: String,
    pub colorized: bool,
}