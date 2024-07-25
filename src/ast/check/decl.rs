use pest::iterators::Pair;

use crate::{
    env::{symbol_table::DataType, Environment},
    error::{AlthreadError, AlthreadResult, ErrorType},
    no_rule,
    parser::Rule,
};

use super::expr::check_expr;

pub fn check_decl(pair: Pair<Rule>, env: &mut Environment) -> AlthreadResult<()> {
    let mut pairs = pair.into_inner();
    let mutable = pairs.next().unwrap().as_str() == "let";
    let identifier = pairs.next().unwrap();
    let mut datatype = None;
    let mut expr = None;
    for pair in pairs {
        match pair.as_rule() {
            Rule::DATATYPE => datatype = Some(DataType::from_str(pair.as_str())),
            Rule::expr => expr = Some(pair),
            _ => return Err(no_rule!(pair)),
        }
    }

    let datatype = check_decl_type(datatype, expr, env)?;

    env.insert_symbol(identifier, datatype, mutable, None)?;

    Ok(())
}

fn check_decl_type<'a>(
    datatype: Option<DataType>,
    expr: Option<Pair<'a, Rule>>,
    env: &mut Environment,
) -> AlthreadResult<DataType> {
    if let Some(expr) = expr {
        let (line, column) = expr.line_col();
        let expr_type = check_expr(expr, env)?;
        if let Some(datatype) = datatype {
            if datatype != expr_type {
                return Err(AlthreadError::new(
                    ErrorType::TypeError,
                    line,
                    column,
                    format!(
                        "Wrong type for variable: expected {:?}, found {:?}",
                        datatype, expr_type
                    ),
                ));
            }
        }
        Ok(expr_type)
    } else {
        Ok(datatype.unwrap_or(DataType::Void))
    }
}
