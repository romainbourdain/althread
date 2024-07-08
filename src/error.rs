#[derive(Debug)]
pub struct Pos {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct AlthreadError {
    pos: Pos,
    message: String,
    error_type: ErrorType,
}

#[derive(Debug)]
pub enum ErrorType {
    SyntaxError,
    TypeError,
    RuntimeError,
    VariableError,
}

impl AlthreadError {
    pub fn error(error_type: ErrorType, line: usize, col: usize, message: String) -> Self {
        Self {
            pos: Pos { line, col },
            message,
            error_type,
        }
    }

    pub fn print_err_line(&self, input: &str) {
        println!("{:?} {input}", self.pos);
        if self.pos.line == 0 {
            return;
        }
        let line = input.lines().nth(self.pos.line - 1).unwrap().to_string();
        let line_indent = " ".repeat(self.pos.line.to_string().len());
        eprintln!("{} |", line_indent);
        eprintln!("{} | {}", self.pos.line, line);
        eprintln!("{} |{}^---", line_indent, " ".repeat(self.pos.col));
        eprintln!("{} |", line_indent);
    }

    pub fn report(&self, input: &str) {
        eprintln!("Error at {}:{}", self.pos.line, self.pos.col);
        self.print_err_line(input);
        eprintln!("{:?}: {}", self.error_type, self.message);
    }
}
