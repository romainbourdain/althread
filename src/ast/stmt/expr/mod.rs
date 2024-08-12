pub mod binary_expr;
pub mod primary_expr;
pub mod unary_expr;

use std::fmt;

use binary_expr::BinaryExpr;
use pest::{iterators::Pairs, pratt_parser::PrattParser};
use primary_expr::PrimaryExpr;
use unary_expr::UnaryExpr;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::{Build, Node},
    },
    error::AlthreadResult,
    parser::Rule,
};

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};

        PrattParser::new()
            .op(Op::infix(Rule::or_priority, Left))
            .op(Op::infix(Rule::and_priority, Left))
            .op(Op::infix(Rule::eq_priority, Left))
            .op(Op::infix(Rule::comp_priority, Left))
            .op(Op::infix(Rule::term_priority, Left))
            .op(Op::infix(Rule::factor_priority, Left))
            .op(Op::prefix(Rule::unary_op))
    };
}

#[derive(Debug)]
pub enum Expr {
    Binary(Node<BinaryExpr>),
    Unary(Node<UnaryExpr>),
    Primary(Node<PrimaryExpr>),
}

pub fn parse_expr(pairs: Pairs<Rule>) -> AlthreadResult<Node<Expr>> {
    PRATT_PARSER
        .map_primary(|primary| {
            Ok(Node {
                line: primary.line_col().0,
                column: primary.line_col().1,
                value: Expr::Primary(PrimaryExpr::build(primary)?),
            })
        })
        .map_infix(|left, op, right| {
            Ok(Node {
                line: op.line_col().0,
                column: op.line_col().1,
                value: Expr::Binary(BinaryExpr::build(left?, op, right?)?),
            })
        })
        .map_prefix(|op, right| {
            Ok(Node {
                line: op.line_col().0,
                column: op.line_col().1,
                value: Expr::Unary(UnaryExpr::build(op, right?)?),
            })
        })
        .parse(pairs)
}

impl Build for Expr {
    fn build(pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        parse_expr(pairs).map(|node| node.value)
    }
}

impl AstDisplay for Expr {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        match self {
            Self::Binary(node) => node.ast_fmt(f, prefix),
            Self::Unary(node) => node.ast_fmt(f, prefix),
            Self::Primary(node) => node.ast_fmt(f, prefix),
        }
    }
}
