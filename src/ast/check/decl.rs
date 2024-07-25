use pest::iterators::Pair;

use crate::{
    env::{symbol_table::DataType, Environment},
    error::AlthreadResult,
    no_rule,
    parser::Rule,
};

pub fn check_decl<'a>(pair: Pair<'a, Rule>, env: &mut Environment) -> AlthreadResult<()> {
    let mut pairs = pair.into_inner();
    let mut data_type = DataType::Void;
    let mut expr_type = DataType::Void;

    for pair in pairs {
        match pair.as_rule() {
            Rule::IDENTIFIER => {
                let name = pair.as_str();
            }
            _ => return Err(no_rule!(pair)),
        }
    }

    Ok(())
}
