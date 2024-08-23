use crate::ast::{statement::expression::Expression, token::literal::Literal};

#[derive(Debug)]
pub enum NodeResult {
    Null,
    Literal(Literal),
    Wait(WaitResult),
}

impl NodeResult {
    pub fn get_literal(self) -> Literal {
        match self {
            NodeResult::Literal(literal) => literal,
            _ => panic!("NodeResult is not a Literal"),
        }
    }
}

#[derive(Debug)]
pub struct WaitResult {
    pub process_id: usize,
    pub condition: Expression,
}
