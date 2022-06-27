use std::{path::Path, env, ffi::OsString, io::{BufReader, Read}, fs};

const LIB_PATH: &str = "LIB_PATH";

pub fn lib_loc() -> OsString {
    match env::var(LIB_PATH) {
        Ok(s) => {
            Path::new(s.as_str()).as_os_str().to_os_string()
        },
        _ => Path::new("./").as_os_str().to_os_string(),
    }
}

pub fn open_lib(path: &Path) -> String {
    let mut s = String::new();
    match fs::File::open(path) {
        Ok(f) => {
            BufReader::new(f).read_to_string(&mut s).unwrap();
        },
        Err(_) => {
            panic!("There are no files in an expected path.");
        },
    }
    s
}

pub fn open_main() -> String {
    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() == 2 {
        &args[1]
    } else {
        panic!("unexpected or not given arguments")
    };
    let mut s = String::new();
    match fs::File::open(input_file) {
        Ok(f) => {
            BufReader::new(f).read_to_string(&mut s).unwrap();
        },
        Err(_) => {
            panic!("unexpected file")
        }
    };

    s
}
