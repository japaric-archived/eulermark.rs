use std::io::fs::File;
use std::io::{Truncate,Write};

pub fn read(path: &Path) -> StrBuf {
    match File::open(path) {
        Err(_) => fail!("failed to open {}", path.display()),
        Ok(mut file) => match file.read_to_str() {
            Err(_) => fail!("failed to read {}", path.display()),
            Ok(contents) => StrBuf::from_owned_str(contents),
        }
    }
}

pub fn write(path: &Path, string: &str) {
    match File::open_mode(path, Truncate, Write) {
        Err(_) => fail!("couldn't open {}", path.display()),
        Ok(mut file) => if file.write_str(string).is_err() {
            fail!("couldn't write to {}", path.display());
        },
    }
}
