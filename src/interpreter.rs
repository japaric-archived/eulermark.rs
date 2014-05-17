#[deriving(Decodable)]
pub struct Interpreter {
    command: StrBuf,
    flags: Vec<StrBuf>,
}

impl Interpreter {
    pub fn command<'a>(&'a self) -> &'a str {
        self.command.as_slice()
    }

    pub fn flags<'a>(&'a self) -> &'a [StrBuf] {
        self.flags.as_slice()
    }
}
