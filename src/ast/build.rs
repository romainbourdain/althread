use std::collections::HashMap;

use pest::iterators::Pairs;

use crate::{
    error::{AlthreadError, ErrorType},
    parser::Rule,
};

use super::Ast;

impl<'a> Ast<'a> {
    pub fn new() -> Self {
        Self {
            process_bricks: HashMap::new(),
            condition_bricks: Vec::new(),
            global_bricks: Vec::new(),
        }
    }

    // La méthode `build` prend des `Pairs` avec une durée de vie `'a`
    pub fn build(pairs: Pairs<'a, Rule>) -> Result<Self, AlthreadError> {
        let mut ast = Self::new();

        for pair in pairs {
            let rule = pair.as_rule();
            let mut inner_pairs = pair.into_inner(); // `inner_pairs` emprunte la durée de vie de `pair`
            match rule {
                Rule::main_brick => {
                    // Insertion avec une durée de vie explicite
                    ast.process_bricks.insert("main".to_string(), inner_pairs);
                }
                Rule::process_brick => {
                    if let Some(name_pair) = inner_pairs.next() {
                        ast.process_bricks
                            .insert(name_pair.as_str().to_string(), inner_pairs);
                    } else {
                        return Err(AlthreadError::error(
                            ErrorType::SyntaxError,
                            0,
                            0,
                            format!("process_brick name missing"),
                        ));
                    }
                }
                Rule::cond_brick => ast.condition_bricks.push(inner_pairs),
                Rule::global_brick => ast.global_bricks.push(inner_pairs),
                Rule::EOI => (),
                _ => {
                    return Err(AlthreadError::error(
                        ErrorType::SyntaxError,
                        0,
                        0,
                        format!("Unexpected rule: {:?}", rule),
                    ))
                }
            }
        }

        Ok(ast)
    }
}
