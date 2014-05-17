use std::io::fs;
use std::io::process::Command;
use std::path::BytesContainer;
use std::str;

#[deriving(Decodable)]
pub struct Compiler {
    byproduct: Vec<StrBuf>,
    command: StrBuf,
    flags: Vec<StrBuf>,
    output: StrBuf,
}

impl Compiler {
    pub fn compile(&self, source: &Path) -> CompilerOutput {
        let command = self.command.as_slice();
        let flags = self.flags.as_slice();
        let output_file = match source.with_extension("").filename_str() {
            None => fail!("{} seems to be malformed", source.display()),
            Some(filename) => str::replace(self.output.as_slice(),
                                           "*",
                                           filename)
        };

        match Command::new(command).args(flags).arg(source).output() {
            Err(_) => fail!("couldn't find {}", command),
            Ok(output) => if output.status.success() {
                CompilerOutput::new(output_file)
            } else {
                fail!("failed to compile {}", source.display())
            },
        }
    }
}

// TODO handle byproduct
pub struct CompilerOutput {
    binary: Path,
}

impl CompilerOutput {
    fn new<T: BytesContainer>(path: T) -> CompilerOutput {
        CompilerOutput {
            binary: Path::new(path),
        }
    }

    pub fn binary<'a>(&'a self) -> &'a Path {
        &self.binary
    }
}

impl Drop for CompilerOutput {
    fn drop(&mut self) {
        if fs::unlink(&self.binary).is_err() {
            println!("failed to unlink {}", self.binary.display())
        }
    }
}

