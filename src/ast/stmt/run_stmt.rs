use pest::iterators::Pair;

use crate::{env::Environment, error::AlthreadError, parser::Rule};

#[derive(Debug)]
pub struct RunStmt {
    pub identifier: String,
    pub line: usize,
    pub column: usize,
}

impl RunStmt {
    pub fn from_pair(pair: Pair<Rule>, env: &Environment) -> Result<Self, AlthreadError> {
        let (line, column) = pair.line_col();
        let identifier = pair.into_inner().next().unwrap().as_str().to_string();

        // TODO : vérifier que le process existe

        Ok(Self {
            identifier,
            line,
            column,
        })
    }
}
