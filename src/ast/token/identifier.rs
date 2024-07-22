use std::fmt;

use pest::iterators::Pair;

use crate::parser::Rule;

#[derive(Debug, Clone)]
pub struct Identifier {
    pub value: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Identifier {
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            value: String::new(),
            line,
            column,
        }
    }

    pub fn from_pair(pair: Pair<Rule>) -> Self {
        let (line, column) = pair.line_col();
        Self {
            value: pair.as_str().to_string(),
            line,
            column,
        }
    }
}
