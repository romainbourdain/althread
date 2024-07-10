pub mod binary;
pub mod primary;
pub mod unary;

use lazy_static::lazy_static;
use pest::{iterators::Pair, pratt_parser::PrattParser};

use crate::{
    env::Environment,
    error::AlthreadError,
    nodes::{
        datatype::DataType,
        expr::{binary::BinExpr, primary::PrimaryExpr, unary::UnExpr, Expr, ExprKind},
    },
    parser::Rule,
};

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
            .op(Op::infix(Rule::mul, Assoc::Left)
                | Op::infix(Rule::div, Assoc::Left)
                | Op::infix(Rule::modulo, Assoc::Left))
            .op(Op::prefix(Rule::not))
            .op(Op::prefix(Rule::neg))
    };
}

impl Expr {
    pub fn build(pair: Pair<Rule>, env: &Environment) -> ExprResult {
        PRATT_PARSER
            .map_primary(|pair| PrimaryExpr::build(pair, env))
            .map_infix(|lhs, op, rhs| BinExpr::build(lhs, op, rhs, env))
            .map_prefix(|op, rhs| UnExpr::build(op, rhs, env))
            .parse(pair.into_inner())
    }

    pub fn default(datatype: &DataType) -> Self {
        use DataType::*;

        let primary = match datatype {
            Int => PrimaryExpr::Int(0),
            Float => PrimaryExpr::Float(0.0),
            Bool => PrimaryExpr::Bool(false),
            String => PrimaryExpr::String("".to_string()),
            Void => PrimaryExpr::Null,
        };

        Self::new(ExprKind::Primary(primary))
    }

    pub fn eval(&self, env: &Environment) -> Result<PrimaryExpr, AlthreadError> {
        use ExprKind::*;
        match &self.kind {
            Primary(expr) => expr.eval(env),
            Binary(expr) => expr.eval(env),
            Unary(expr) => expr.eval(env),
        }
    }
}
