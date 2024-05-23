#[path="git.rs"]
mod git;

/* public function find_wotog_dir( std::path::PathBuf ) -> Result<std::path::PathBuf, std::io::Error> -- find the .wotog directory relative to the .git directory
 *  - std::path::PathBuf: dir           -- the directory to start the search from
 *  Returns a Result of Ok(std::path::PathBuf) or Err(std::io::Error)
**/
pub fn find_wotog_dir(dir: std::path::PathBuf) -> Result<std::path::PathBuf, std::io::Error> {
    // go back a directory until you see that the ".git" directory is a neighbor
    let _ = match std::fs::read_dir(&dir) {
        Ok(_) => {
            match std::path::Path::new(dir.join(".wotog").to_str().unwrap()).try_exists() {
                Ok(true)  => return Ok(dir.join(".wotog")),
                Ok(false) => {
                    match dir.parent() {
                        // operate recursively until the .git directory is found
                        Some(_) => return find_wotog_dir(dir.parent().unwrap().to_path_buf()),
                        None    => return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "wotog directory not found")),
                    }
                }
                // this should never occur because the Ok(false) case should detect running out of directories
                Err(e) => panic!("{}", e),
            }
        },
        Err(e) => panic!("{}", e),
    };
}

/* public function wotog_init( u8, Option<String> ) -> Result<(), std::io::Error> -- create the .wotog directory and config file relative to the .git directory
 *  - u8: debug_level                   -- the verbosity of the output
 *  - Option<String>: config_changes    -- the changes to the config file
 *  Returns a Result of Ok(()) or Err(std::io::Error)
**/
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
    // start from the current directory
    let cwd: std::path::PathBuf = match std::env::current_dir() {
        Ok(d) => d,
        Err(e) => panic!("{e}"),
    };
    // find .git directory
    // TODO: should wotog init run 'git init' at cwd if no .git directory is found?
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
            let _  = match wotog_create_config(wotog_dir, config_changes, debug_level) {
                Ok(_) => {},
                Err(e) => panic!("{}", e),
            };

        },
        Err(e) => panic!("{}", e),
    }

    if debug_level == 2 {
        println!("wotog initialized.");
    }
    return Ok(());
}

/* function wotog_create_config( std::path::PathBuf, Option<String>, u8 ) -> Result<(), std::io::Error> -- create the config file in the .wotog directory
 *  - std::path::PathBuf: dir           -- the directory to create the config file in
 *  - Option<String>: changes           -- the changes to the config file
 *  - u8: debug_level                   -- the verbosity of the output
 *  Returns a Result of Ok(()) or Err(std::io::Error)
**/
fn wotog_create_config(dir: std::path::PathBuf, changes: Option<String>, debug_level: u8) -> Result<(), std::io::Error> {
    if debug_level == 2 {
        println!("wotog_create_config: {:?}, {:?}", dir, changes);
    }
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