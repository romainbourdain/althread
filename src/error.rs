#[derive(Debug)]
pub struct Pos {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct AlthreadError {
    pos: Pos,
    message: String,
}

impl AlthreadError {
    pub fn error(line: usize, col: usize, message: String) -> Self {
        Self {
            pos: Pos { line, col },
            message,
        }
    }

    pub fn report(&self, loc: String) {
        eprintln!(
            "[{}:{}]\n{}: {}",
            self.pos.line, self.pos.col, loc, self.message
        );
    }
}
