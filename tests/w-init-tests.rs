#[path = "../src/wotog.rs"]
mod wotog;
// tests for wotog init

// TODO: enable this test if and only if wotog init should run git init if no .git directory is found
// #[test]
// fn test_wotog_init_no_git() {
//     let tmp_dir = tempdir::TempDir::new("t").unwrap();
//     let _ = match std::env::set_current_dir(tmp_dir.path()) {
//         Ok(_) => {},
//         Err(e) => panic!("couldn't change directory to t :: {}", e),
//     };
//     // third argument controls 'git init' execution
//     let _ = match wotog_init(2, None, true) {
//         Ok(_) => {},
//         Err(e) => panic!("{}", e),
//     };
//     assert!(std::Path::new(".git").exists());
//     assert!(std::Path::new(".wotog").exists());
//     assert!(std::Path::new(".wotog").join("config.toml").exists());
// }

#[test]
fn test_wotog_init() {
    let tmp_dir = tempdir::TempDir::new("t").unwrap();
    let _ = match std::env::set_current_dir(tmp_dir.path()) {
        Ok(_) => {},
        Err(e) => panic!("couldn't change directory to t :: {}", e),
    };
    let _ = match std::fs::create_dir(".git") {
        Ok(_) => {},
        Err(e) => panic!("couldn't create dummy .git directory :: {}", e),
    };
    let _ = match wotog::wotog_init(2, None) {
        Ok(_) => {},
        Err(e) => panic!("couldn't complete wotog init :: {}", e),
    };
    assert!(std::path::Path::new(".wotog").exists());
    assert!(std::path::Path::new(".wotog").join("config.toml").exists());
}