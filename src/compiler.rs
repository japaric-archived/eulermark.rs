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
    pub fn compile(&self, source: &Path) -> Option<CompilerOutput> {
        let command = self.command.as_slice();
        let flags = self.flags.as_slice();
        let output_file = match source.with_extension("").filename_str() {
            None => {
                println!("Malformed input file");

                return None;
            },
            Some(filename) => str::replace(self.output.as_slice(),
                                           "*",
                                           filename)
        };

        match Command::new(command).args(flags).arg(source).output() {
            Err(_) => {
                println!("Couldn't find the compiler");

                None
            },
            Ok(output) => if output.status.success() {
                Some(CompilerOutput::new(output_file))
            } else {
                println!("Compiler error");
                print!("{}", StrBuf::from_utf8(output.error).unwrap());

                None
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

