use language::Language;
use problem::Problem;

pub struct Solution<'l, 'p> {
    file: Path,
    language: &'l Language,
    problem: &'p Problem,
}

impl<'l, 'p> Solution<'l, 'p> {
    pub fn new_opt(language: &'l Language, problem: &'p Problem)
        -> Option<Solution<'l, 'p>>
    {
        let file = problem.answer().with_extension(language.extension());

        if file.exists() {
            Some(Solution {
                file: file,
                language: language,
                problem: problem,
            })
        } else {
            None
        }
    }

    pub fn language(&self) -> &'l Language {
        self.language
    }

    pub fn problem(&self) -> &'p Problem {
        self.problem
    }

    pub fn file<'a>(&'a self) -> &'a Path {
        &self.file
    }
}
