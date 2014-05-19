use interpreter::Interpreter;
use std::io::process::Command;

pub struct Executable<'f, 'i> {
    file: &'f Path,
    interpreter: Option<&'i Interpreter>,
}

impl<'f, 'i> Executable<'f, 'i> {
    pub fn new(file: &'f Path, interpreter: Option<&'i Interpreter>)
        -> Executable<'f, 'i>
    {
        Executable {
            file: file,
            interpreter: interpreter,
        }
    }

    pub fn bench(&self, iters: u64) -> u64 {
        let file = self.file;
        let mut command = match self.interpreter {
            None => Command::new(format!("./{}", file.display())),
            Some(interpreter) => Command::new(interpreter.command()),

        };

        let iters = iters.to_str();
        let output = match self.interpreter {
            None => command.arg(iters).output(),
            Some(interpreter) => {
                let flags = interpreter.flags();

                command.args(flags).arg(file).arg(iters).output()
            },
        };

        match output {
            Err(_) => fail!("couldn't find {}", command),
            Ok(output) => if output.status.success() {
                match StrBuf::from_utf8(output.output) {
                    Err(_) => fail!("malformed output"),
                    Ok(string) => match from_str(string.as_slice().trim()) {
                        None => fail!("malformed output"),
                        Some(integer) => integer,
                    },
                }
            } else {
                fail!("{} failed at runtime", file.display())
            },
        }
    }

    pub fn test(&self) -> Option<StrBuf> {
        let file = self.file;
        let mut command = match self.interpreter {
            None => Command::new(format!("./{}", file.display())),
            Some(interpreter) => Command::new(interpreter.command()),

        };

        let output = match self.interpreter {
            None => command.output(),
            Some(interpreter) => {
                command.args(interpreter.flags()).arg(file).output()
            },
        };

        match output {
            Err(_) => {
                println!("Couldn't find the interpreter");

                None
            },
            Ok(output) => if output.status.success() {
                match StrBuf::from_utf8(output.output) {
                    Err(_) => {
                        println!("Bad output");

                        None
                    },
                    Ok(string) => Some(string),
                }
            } else {
                println!("Runtime failure");

                println!("{}", StrBuf::from_utf8(output.error));

                None
            },
        }
    }
}
