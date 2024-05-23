/* public function find_git_dir( std::path::PathBuf ) -> Result<std::path::PathBuf, std::io::Error> -- find the .git directory relative to the current directory
 *  - std::path::PathBuf: dir    -- the current directory
 *  Returns a Result of Ok(p: std::path::PathBuf) where p is the directory that contains '.git', or Err(std::io::Error) if the directory is not found
**/
pub fn find_git_dir(dir: std::path::PathBuf) -> Result<std::path::PathBuf, std::io::Error> {
    // go back a directory until you see that the ".git" directory is a neighbor
    let _ = match std::fs::read_dir(&dir) {
        Ok(_) => {
            match std::path::Path::new(dir.join(".git").to_str().unwrap()).try_exists() {
                Ok(true)  => return Ok(dir),
                Ok(false) => {
                    match dir.parent() {
                        Some(_) => return find_git_dir(dir.parent().unwrap().to_path_buf()),
                        None    => return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "git directory not found")),
                    };
                },
                Err(e)    => panic!("{e}"),
            };
        }
        Err(e) => panic!("{e}"),
    };
}
