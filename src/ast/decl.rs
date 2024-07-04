use pest::iterators::Pairs;

use crate::{env::Environment, error::AlthreadError, parser::Rule};

use super::{
    datatype::DataType,
    expr::{Expr, PrimaryExpr},
};

#[derive(Debug)]
pub struct Decl {
    pub identifier: String,
    pub value: Expr,
    pub datatype: DataType,
    pub mutable: bool,
}

impl Decl {
    pub fn build(pairs: Pairs<Rule>, env: &mut Environment) -> Result<Self, AlthreadError> {
        let mut decl = Decl {
            identifier: "".to_string(),
            value: Expr::Primary(PrimaryExpr::Null),
            datatype: DataType::Void,
            mutable: false,
        };

        for pair in pairs {
            match pair.as_rule() {
                Rule::IDENTIFIER => decl.identifier = pair.as_str().to_string(),
                Rule::DATATYPE => decl.datatype = DataType::build(pair)?,
                Rule::decl_keyword => decl.mutable = pair.as_str() == "let",
                Rule::expr => decl.value = Expr::build(pair.into_inner(), env)?,
                _ => unreachable!(),
            }
        }

        Self::evaluate_type(&mut decl, env)?;
        env.insert_symbol(
            decl.identifier.clone(),
            decl.datatype.clone(),
            decl.mutable,
            None,
        )?;

        Ok(decl)
    }

    fn evaluate_type(&mut self, env: &Environment) -> Result<(), AlthreadError> {
        let value_type = DataType::from_expr(&self.value, env)?;

        match (&self.datatype, &value_type) {
            (_, DataType::Void) => self.value = Expr::default(&self.datatype),
            (DataType::Void, _) => self.datatype = value_type,
            _ if (self.datatype == value_type) => {}
            _ => return Err(AlthreadError::error(0, 0, "Type Mismatch".to_string())),
        }

        Ok(())
    }
}
