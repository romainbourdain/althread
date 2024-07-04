use lazy_static::lazy_static;
use pest::{
    iterators::{Pair, Pairs},
    pratt_parser::PrattParser,
};

use crate::{error::AlthreadError, parser::Rule};

use super::datatype::DataType;

type ExprResult = Result<Expr, AlthreadError>;

lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc, Op};

        PrattParser::new()
            .op(Op::infix(Rule::or, Assoc::Left))
            .op(Op::infix(Rule::and, Assoc::Left))
            .op(Op::infix(Rule::eq, Assoc::Left) | Op::infix(Rule::ne, Assoc::Left))
            .op(Op::infix(Rule::gt, Assoc::Left)
                | Op::infix(Rule::ge, Assoc::Left)
                | Op::infix(Rule::lt, Assoc::Left)
                | Op::infix(Rule::le, Assoc::Left))
            .op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::sub, Assoc::Left))
            .op(Op::infix(Rule::mul, Assoc::Left) | Op::infix(Rule::div, Assoc::Left))
            .op(Op::prefix(Rule::not))
            .op(Op::prefix(Rule::sub))
    };
}

#[derive(Debug)]
pub enum Expr {
    Null,
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Identifier(String),
    BinOp {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    UnOp {
        op: UnOp,
        lhs: Box<Expr>,
    },
}

impl Expr {
    pub fn build(pairs: Pairs<Rule>) -> ExprResult {
        PRATT_PARSER
            .map_primary(Self::parse_primary)
            .map_infix(Self::parse_infix)
            .map_prefix(Self::parse_prefix)
            .parse(pairs)
    }

    pub fn default(datatype: &DataType) -> Self {
        match datatype {
            DataType::Int => Self::Int(0),
            DataType::Float => Self::Float(0.0),
            DataType::Bool => Self::Bool(false),
            DataType::String => Self::String("".to_string()),
            _ => Self::Null,
        }
    }

    fn parse_primary(pair: Pair<Rule>) -> ExprResult {
        let expr = match pair.as_rule() {
            Rule::BOOLEAN => Self::Bool(pair.as_str() == "true"),
            Rule::INTEGER => Self::Int(pair.as_str().parse::<i64>().unwrap()),
            Rule::FLOAT => Self::Float(pair.as_str().parse::<f64>().unwrap()),
            Rule::STRING => Self::String(pair.as_str().to_string()),
            Rule::IDENTIFIER => Self::Identifier(pair.as_str().to_string()),
            Rule::NULL => Self::Null,
            Rule::expr => Self::build(pair.into_inner())?,
            _ => unreachable!("{:?}", pair),
        };
        Ok(expr)
    }

    fn parse_infix(lhs: ExprResult, op: Pair<Rule>, rhs: ExprResult) -> ExprResult {
        let op = match op.as_rule() {
            Rule::add => BinOp::Add,
            Rule::sub => BinOp::Sub,
            Rule::mul => BinOp::Mul,
            Rule::div => BinOp::Div,
            Rule::eq => BinOp::Eq,
            Rule::ne => BinOp::Ne,
            Rule::gt => BinOp::Gt,
            Rule::ge => BinOp::Ge,
            Rule::lt => BinOp::Lt,
            Rule::le => BinOp::Le,
            Rule::and => BinOp::And,
            Rule::or => BinOp::Or,
            _ => unreachable!("{:?}", op),
        };
        let lhs = lhs?;
        let rhs = rhs?;

        DataType::from_bin_expr(&op, &lhs, &rhs)?;

        Ok(Self::BinOp {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }

    fn parse_prefix(op: Pair<Rule>, lhs: ExprResult) -> ExprResult {
        let op = match op.as_rule() {
            Rule::not => UnOp::Not,
            Rule::sub => UnOp::Neg,
            _ => unreachable!("{:?}", op),
        };
        let lhs = lhs?;

        DataType::from_un_expr(&op, &lhs)?;

        Ok(Self::UnOp {
            op,
            lhs: Box::new(lhs),
        })
    }
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
    And,
    Or,
}

#[derive(Debug)]
pub enum UnOp {
    Not,
    Neg,
}
