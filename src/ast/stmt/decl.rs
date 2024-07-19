use pest::iterators::Pair;

use crate::{
    ast::{
        expr::{primary::PrimaryExpr, Expr, ExprKind},
        token::datatype::DataType,
    },
    env::Environment,
    error::{AlthreadError, ErrorType},
    parser::Rule,
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
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            identifier: "".to_string(),
            value: Expr::new(ExprKind::Primary(PrimaryExpr::Null)),
            datatype: DataType::new(),
            mutable: false,
            line,
            column,
        }
    }
}

impl Decl {
    pub fn from_pair(pair: Pair<Rule>, env: &mut Environment) -> Result<Self, AlthreadError> {
        let (line, column) = pair.line_col();
        let mut decl = Decl::new(line, column);

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::IDENTIFIER => decl.identifier = pair.as_str().to_string(),
                Rule::DATATYPE => decl.datatype = DataType::from_pair(pair)?,
                Rule::decl_keyword => decl.mutable = pair.as_str() == "let",
                Rule::expr => decl.value = Expr::from_pair(pair, env)?,
                _ => unreachable!(),
            }
        }

        decl.evaluate_type(env)?;
        env.insert_symbol(
            decl.identifier.clone(),
            decl.datatype.clone(),
            decl.mutable,
            None,
        )
        .map_err(|e| AlthreadError::error(ErrorType::VariableError, decl.line, decl.column, e))?;

        Ok(decl)
    }

    /// # Compare expr datatype and declared datatype
    ///
    /// - Give a value for declarations without value (from datatype)
    /// - Give a datatype for declaration without datatype (from value)
    /// - Throw error for incompatible datatype
    fn evaluate_type(&mut self, env: &Environment) -> Result<(), AlthreadError> {
        let value_type = self.value.get_datatype(env)?;

        match (&self.datatype, &value_type) {
            (_, DataType::Void) => self.value = Expr::from_datatype(&self.datatype),
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

    pub fn eval(&self, env: &mut Environment) -> Result<(), AlthreadError> {
        env.insert_symbol(
            self.identifier.clone(),
            self.datatype.clone(),
            self.mutable,
            Some(self.value.eval(env)?),
        )
        .map_err(|e| AlthreadError::error(ErrorType::VariableError, self.line, self.column, e))?;

        Ok(())
    }
}
