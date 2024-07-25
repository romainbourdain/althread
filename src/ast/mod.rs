pub mod build;
pub mod check;
pub mod display;
pub mod eval;

use crate::parser::Rule;
use pest::iterators::Pairs;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Ast<'a> {
    pub process_bricks: HashMap<String, Pairs<'a, Rule>>,
    pub condition_bricks: Vec<Pairs<'a, Rule>>,
    pub global_bricks: Vec<Pairs<'a, Rule>>,
}
