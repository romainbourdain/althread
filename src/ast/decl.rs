use pest::iterators::Pair;

use crate::{
    env::Environment,
    error::{AlthreadError, ErrorType},
    parser::Rule,
};

use super::{
    datatype::DataType,
    expr::{Expr, ExprKind, PrimaryExpr},
};

#[derive(Debug)]
pub struct Decl {
    pub identifier: String,
    pub value: Expr,
    pub datatype: DataType,
    pub mutable: bool,
    pub line: usize,
    pub column: usize,
}

impl Decl {
    pub fn build(pair: Pair<Rule>, env: &mut Environment) -> Result<Self, AlthreadError> {
        let (line, column) = pair.line_col();
        let mut decl = Decl {
            identifier: "".to_string(),
            value: Expr::new(ExprKind::Primary(PrimaryExpr::Null)),
            datatype: DataType::Void,
            mutable: false,
            line,
            column,
        };

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::IDENTIFIER => decl.identifier = pair.as_str().to_string(),
                Rule::DATATYPE => decl.datatype = DataType::build(pair)?,
                Rule::decl_keyword => decl.mutable = pair.as_str() == "let",
                Rule::expr => decl.value = Expr::build(pair, env)?,
                _ => unreachable!(),
            }
        }

        Self::evaluate_type(&mut decl, env)?;
        env.insert_symbol(
            decl.identifier.clone(),
            decl.datatype.clone(),
            decl.mutable,
            None,
        )
        .map_err(|e| AlthreadError::error(ErrorType::VariableError, decl.line, decl.column, e))?;

        Ok(decl)
    }

    fn evaluate_type(&mut self, env: &Environment) -> Result<(), AlthreadError> {
        let value_type = DataType::from_expr(&self.value.kind, env).map_err(|e| {
            AlthreadError::error(ErrorType::TypeError, self.value.line, self.value.column, e)
        })?;

        match (&self.datatype, &value_type) {
            (_, DataType::Void) => self.value = Expr::default(&self.datatype),
            (DataType::Void, _) => self.datatype = value_type,
            _ if (self.datatype == value_type) => {}
            _ => {
                return Err(AlthreadError::error(
                    ErrorType::TypeError,
                    self.value.line,
                    self.value.column,
                    format!("Cannot convert {} to {}", value_type, self.datatype),
                ))
            }
        }

        Ok(())
    }

    pub fn eval(&self) -> Result<(), AlthreadError> {
        // TODO: Implement evaluation
        unimplemented!();
    }
}
