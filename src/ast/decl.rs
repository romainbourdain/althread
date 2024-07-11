use pest::iterators::Pair;

use crate::{
    env::Environment,
    error::{AlthreadError, ErrorType},
    nodes::{datatype::DataType, decl::Decl, expr::Expr},
    parser::Rule,
};

impl Decl {
    pub fn build(pair: Pair<Rule>, env: &mut Environment) -> Result<Self, AlthreadError> {
        let (line, column) = pair.line_col();
        let mut decl = Decl::new(line, column);

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::IDENTIFIER => decl.identifier = pair.as_str().to_string(),
                Rule::DATATYPE => decl.datatype = DataType::build(pair)?,
                Rule::decl_keyword => decl.mutable = pair.as_str() == "let",
                Rule::expr => decl.value = Expr::build(pair, env)?,
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
}
