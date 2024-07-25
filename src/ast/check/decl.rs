use pest::iterators::Pair;

use crate::{
    env::symbol_table::DataType,
    error::{AlthreadError, AlthreadResult, ErrorType},
    no_rule,
    parser::Rule,
};

use super::expr::check_expr;

pub fn check_decl(pair: Pair<Rule>) -> AlthreadResult<()> {
    let mut pairs = pair.into_inner();
    let decl_keyword = pairs.next().unwrap();
    let ident = pairs.next().unwrap();
    let mut datatype = None;
    let mut expr = None;
    for pair in pairs {
        match pair.as_rule() {
            Rule::DATATYPE => datatype = Some(DataType::from_str(pair.as_str())),
            Rule::expr => expr = Some(pair),
            _ => return Err(no_rule!(pair)),
        }
    }

    let datatype = check_decl_type(datatype, expr)?;

    Ok(())
}

fn check_decl_type<'a>(
    datatype: Option<DataType>,
    expr: Option<Pair<'a, Rule>>,
) -> AlthreadResult<DataType> {
    if let Some(expr) = expr {
        let (line, column) = expr.line_col();
        let expr_type = check_expr(expr)?;
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
