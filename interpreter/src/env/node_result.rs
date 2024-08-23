use crate::ast::{node::Node, statement::expression::Expression, token::literal::Literal};

#[derive(Debug)]
pub enum NodeResult {
    Incomplete,
    Finished(Literal),
    Suspend(Node<Expression>),
}

impl NodeResult {
    pub fn get_return(self) -> Literal {
        match self {
            NodeResult::Finished(literal) => literal,
            _ => panic!("NodeResult is not a Literal"),
        }
    }

    pub fn is_finished(&self) -> bool {
        match self {
            NodeResult::Finished(_) => true,
            _ => false,
        }
    }

    pub fn null() -> Self {
        NodeResult::Finished(Literal::Null)
    }

    pub fn is_null(&self) -> bool {
        match self {
            NodeResult::Finished(Literal::Null) => true,
            _ => false,
        }
    }
}
