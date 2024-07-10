use pest::iterators::Pair;

use crate::{
    env::Environment,
    error::{AlthreadError, ErrorType},
    nodes::expr::{primary::PrimaryExpr, Expr, ExprKind},
    parser::Rule,
};

use super::ExprResult;

impl PrimaryExpr {
    pub fn build(pair: Pair<Rule>, env: &Environment) -> ExprResult {
        let (line, column) = pair.line_col();
        if pair.as_rule() == Rule::expr {
            return Expr::build(pair, env);
        }
        let expr = match pair.as_rule() {
            Rule::NULL => Self::Null,
            Rule::INTEGER => Self::Int(pair.as_str().parse::<i64>().unwrap()),
            Rule::FLOAT => Self::Float(pair.as_str().parse::<f64>().unwrap()),
            Rule::BOOLEAN => Self::Bool(pair.as_str() == "true"),
            Rule::STRING => Self::String(pair.as_str().to_string()),
            Rule::IDENTIFIER => {
                let identifier = pair.as_str().to_string();
                let symbol = env
                    .get_symbol(&identifier)
                    .map_err(|e| AlthreadError::error(ErrorType::VariableError, line, column, e))?;
                match &symbol.value {
                    Some(value) => Ok(value.clone()),
                    None => Ok(PrimaryExpr::Identifier(identifier)),
                }?
            }
            rule => unreachable!("{:?}", rule),
        };

        Ok(Expr {
            kind: ExprKind::Primary(expr),
            line: pair.as_span().start_pos().line_col().0,
            column: pair.as_span().start_pos().line_col().1,
        })
    }

    pub fn eval(&self, env: &Environment) -> Result<PrimaryExpr, AlthreadError> {
        match self {
            PrimaryExpr::Identifier(ident) => {
                let symbol = env
                    .get_symbol(ident)
                    .map_err(|err| AlthreadError::error(ErrorType::RuntimeError, 0, 0, err))?;

                if let Some(value) = symbol.value.as_ref() {
                    Ok(value.clone())
                } else {
                    unreachable!("symbol has no value");
                }
            }
            _ => Ok(self.clone()),
        }
    }
}
