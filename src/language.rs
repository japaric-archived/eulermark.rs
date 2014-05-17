use compiler::Compiler;
use interpreter::Interpreter;
use std::io::fs::readdir;
use json::{parse_language_file};

#[deriving(Decodable)]
pub struct Language {
    compiler: Option<Compiler>,
    extension: StrBuf,
    interpreter: Option<Interpreter>,
    name: StrBuf,
}

impl Language {
    pub fn extension<'a>(&'a self) -> &'a str {
        self.extension.as_slice()
    }

    pub fn compiler<'a>(&'a self) -> Option<&'a Compiler> {
        self.compiler.as_ref()
    }

    pub fn interpreter<'a>(&'a self) -> Option<&'a Interpreter> {
        self.interpreter.as_ref()
    }

    pub fn name<'a>(&'a self) -> &'a str {
        self.name.as_slice()
    }
}

pub fn supported_languages() -> Vec<Language> {
    let files = match readdir(&Path::new("languages")) {
        Err(_) => fail!("Couldn't find the languages directory"),
        Ok(files) => files,
    };

    files.move_iter().map(|file| {
        parse_language_file(&file)
    }).collect()
}
