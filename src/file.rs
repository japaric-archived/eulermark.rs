use std::io::fs::File;
use std::io::{Open,Write};

pub fn read(path: &Path) -> ~str {
    match File::open(path) {
        Err(_) => fail!("failed to open {}", path.display()),
        Ok(mut file) => match file.read_to_str() {
            Err(_) => fail!("failed to read {}", path.display()),
            Ok(contents) => contents,
        }
    }
}

pub fn write(path: &Path, string: &str) {
    match File::open_mode(path, Open, Write) {
        Err(_) => fail!("couldn't open {}", path.display()),
        Ok(mut file) => if file.write_str(string).is_err() {
            fail!("couldn't write to {}", path.display());
        },
    }
}
