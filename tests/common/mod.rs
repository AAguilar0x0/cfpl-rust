use cfpl::file;
use std::fs;

pub fn no_input(expected: bool, path: &str) {
    let paths = fs::read_dir(path);
    assert!(paths.is_ok());
    let paths = paths.unwrap();
    for path in paths {
        assert!(path.is_ok());
        let path = path.unwrap();
        let path = path.path().to_str().unwrap().to_owned();
        println!("Test file full path: {}", &path);
        let result = file(&path);
        assert_eq!(expected, result);
        println!();
    }
}
