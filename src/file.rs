use std::io::fs;
use std::io::fs::File;
use std::io::{Truncate,Write};

pub struct Symlink {
    dst: Path,
}

impl Symlink {
    pub fn new(pid: &str) -> Symlink {
        let src = Path::new(format!("problems/{0}/{0}.txt", pid));
        let dst = Path::new(format!("{}.txt", pid));

        match fs::symlink(&src, &dst) {
            Err(_) => fail!("failed to create symlink"),
            Ok(_) => {},
        }

        Symlink {
            dst: dst,
        }
    }
}

impl Drop for Symlink {
    fn drop(&mut self) {
        match fs::unlink(&self.dst) {
            Err(_) => println!("failed to remove {}", self.dst.display()),
            Ok(_) => {},
        }
    }
}

pub fn read(path: &Path) -> String {
    match File::open(path) {
        Err(_) => fail!("failed to open {}", path.display()),
        Ok(mut file) => match file.read_to_str() {
            Err(_) => fail!("failed to read {}", path.display()),
            Ok(contents) => contents,
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
