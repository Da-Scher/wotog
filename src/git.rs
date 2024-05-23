pub fn find_git_dir(dir: std::path::PathBuf) -> Result<std::path::PathBuf, std::io::Error> {
    // go back a directory until you see that the ".git" directory is a neighbor
    let _ = match std::fs::read_dir(&dir) {
        Ok(_) => {
            match std::path::Path::new(dir.join(".git").to_str().unwrap()).try_exists() {
                Ok(true)  => return Ok(dir),
                Ok(false) => return find_git_dir(dir.parent().unwrap().to_path_buf()),
                Err(e)    => panic!("{e}"),
            }
        },
        Err(e) => panic!("{e}"),
    };
}
