use pest::iterators::Pair;

use crate::{
    ast::token::unary_assign_op::UnaryAssignOp,
    env::Environment,
    error::{AlthreadError, ErrorType},
    parser::Rule,
};

#[derive(Debug)]
pub struct AssignUnary {
    pub left: String,
    pub op: UnaryAssignOp,
    pub line: usize,
    pub column: usize,
}

impl AssignUnary {
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            left: "".to_string(),
            op: UnaryAssignOp::Increment,
            line,
            column,
        }
    }

    pub fn build(pair: Pair<Rule>, env: &Environment) -> Result<Self, AlthreadError> {
        let (line, column) = pair.line_col();
        let mut assign = Self::new(line, column);

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::IDENTIFIER => assign.left = pair.as_str().to_string(),
                Rule::assign_unary_op => assign.op = UnaryAssignOp::build(pair)?,
                _ => unreachable!(),
            }
        }

        let symbol = env.get_symbol(&assign.left).map_err(|e| {
            AlthreadError::error(ErrorType::VariableError, assign.line, assign.column, e)
        })?;

        if !symbol.mutable {
            return Err(AlthreadError::error(
                ErrorType::VariableError,
                assign.line,
                assign.column,
                "Cannot change immutable variable value".to_string(),
            ));
        }

        Ok(assign)
    }
}
