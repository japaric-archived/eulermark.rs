pub struct Problem {
    answer: Path,
    directory: Path,
    id: String,
}

impl Problem {
    pub fn new_opt(id: uint) -> Option<Problem> {
        let id = format!("{:03}", id);
        let directory = Path::new(format!("problems/{}", id));
        let answer = directory.join(format!("{}.ans", id));

        if answer.exists() {
            Some(Problem {
                answer: answer,
                directory: directory,
                id: id,
            })
        } else {
            None
        }
    }

    pub fn answer<'a>(&'a self) -> &'a Path {
        &self.answer
    }

    pub fn id<'a>(&'a self) -> &'a str {
        self.id.as_slice()
    }
}
