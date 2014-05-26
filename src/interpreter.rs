#[deriving(Decodable)]
pub struct Interpreter {
    command: String,
    flags: Vec<String>,
}

impl Interpreter {
    pub fn command<'a>(&'a self) -> &'a str {
        self.command.as_slice()
    }

    pub fn flags<'a>(&'a self) -> &'a [String] {
        self.flags.as_slice()
    }
}
