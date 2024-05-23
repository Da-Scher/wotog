#[path="git.rs"]
mod git;

pub fn find_wotog_dir(dir: std::path::PathBuf) -> Result<std::path::PathBuf, std::io::Error> {
    println!("{}", dir.display());
    // go back a directory until you see that the ".git" directory is a neighbor
    let _ = match std::fs::read_dir(&dir) {
        Ok(d) => {
            match std::path::Path::new(dir.join(".wotog").to_str().unwrap()).try_exists() {
                Ok(true)  => return Ok(dir.join(".wotog")),
                Ok(false) => {
                    match dir.parent() {
                        Some(_) => return find_wotog_dir(dir.parent().unwrap().to_path_buf()),
                        None    => return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "wotog directory not found")),
                    }
                }
                Err(e) => panic!("{}", e),
            }
        },
        Err(e) => panic!("{e}"),
    };
}

pub fn wotog_init(debug_level: u8, config_changes: Option<String>) -> Result<(), std::io::Error> {
    if debug_level == 2 {
        println!("Wotog Init Command");
    }
    let _ = match find_wotog_dir(std::env::current_dir().unwrap()) {
        Ok(p) => {
            if debug_level == 2 {
                println!("wotog already initialized for this project.");
            }
            return Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, format!("wotog already initialized at path: {}", p.display())));
        }
        _ => {
            if debug_level == 2 {
                println!("initializing wotog");
            }
        } 
    };
    let cwd: std::path::PathBuf = match std::env::current_dir() {
        Ok(d) => d,
        Err(e) => panic!("{e}"),
    };
    // find .git directory
    let root_dir = match git::find_git_dir(cwd) {
        Ok(path) => path,
        Err(e)   => panic!("Error occured: {e}"),
    };

    let wotog_dir: std::path::PathBuf = root_dir.join(".wotog");
    match std::fs::create_dir(root_dir.join(".wotog")) {
        Ok(()) => {
            if debug_level >= 1 {
                println!("creating wotog directory at {}", root_dir.display());
            }
            if debug_level == 2 { 
                println!("creating config toml at {}", root_dir.join(".wotog").join("config.toml").display());
            }
            wotog_create_config(wotog_dir, config_changes, debug_level);

        },
        Err(e) => panic!("{}", e),
    }

    if debug_level == 2 {
        println!("wotog initialized.");
    }
    return Ok(());
}

fn wotog_create_config(dir: std::path::PathBuf, changes: Option<String>, debug_level: u8) -> Result<(), std::io::Error> {
    // write the config file
    match std::fs::write(dir.join("config.toml"), "#wotog local configuration\nmotd = \"nice stack, buddy.\"") {
        Ok(()) => {
            if debug_level == 2 {
                println!("creating config file.");
            }
        },
        Err(e) => panic!("error: {}", e),

    };
    return Ok(());
}

fn make_config_builder() -> Result<config::Config, std::io::Error> {
    let cwd = match std::env::current_dir() {
        Ok(c) => c,
        Err(e) => panic!("no wotog directory"),
    };
    let wotog_dir: std::path::PathBuf = match find_wotog_dir(cwd) {
        Ok(c) => c,
        Err(e) => panic!("error occurred"),
    };
    let builder = config::Config::builder()
        .add_source(config::File::new(wotog_dir.join("config").to_str().expect("could not find"), config::FileFormat::Toml));
    let config = match builder.build() {
        Ok(c) => return Ok(c),
        Err(e) => match e {
            config::ConfigError::Frozen => return Err(std::io::Error::new(std::io::ErrorKind::Other, "file frozen")),
            config::ConfigError::NotFound(s) => return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{} not found", s))),
            config::ConfigError::PathParse(ek) => return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Could not find configuration file in the wotog root project directory. {:?}", ek))),
            config::ConfigError::FileParse {uri, cause} => return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{}", uri.unwrap()))),
            config::ConfigError::Type {origin, unexpected, expected, key} => return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("type {}", origin.unwrap()))),
            config::ConfigError::Message(s) => return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("message {}", s))),
            config::ConfigError::Foreign(b) => return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("foreign {}", b))),
        },
    };
}