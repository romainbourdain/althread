pub mod binary_expression;
pub mod primary_expression;
pub mod unary_expression;

use std::fmt;

use binary_expression::BinaryExpression;
use pest::{iterators::Pairs, pratt_parser::PrattParser};
use primary_expression::PrimaryExpression;
use unary_expression::UnaryExpression;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::{Node, NodeBuilder, NodeExecutor},
        token::literal::Literal,
    },
    env::process_env::ProcessEnv,
    error::AlthreadResult,
    parser::Rule,
};

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};

        PrattParser::new()
            .op(Op::infix(Rule::or_operator, Left))
            .op(Op::infix(Rule::and_operator, Left))
            .op(Op::infix(Rule::equality_operator, Left))
            .op(Op::infix(Rule::comparison_operator, Left))
            .op(Op::infix(Rule::term_operator, Left))
            .op(Op::infix(Rule::factor_operator, Left))
            .op(Op::prefix(Rule::unary_operator))
    };
}

#[derive(Debug)]
pub enum Expression {
    Binary(Node<BinaryExpression>),
    Unary(Node<UnaryExpression>),
    Primary(Node<PrimaryExpression>),
}

pub fn parse_expr(pairs: Pairs<Rule>) -> AlthreadResult<Node<Expression>> {
    PRATT_PARSER
        .map_primary(|primary| {
            Ok(Node {
                line: primary.line_col().0,
                column: primary.line_col().1,
                value: Expression::Primary(PrimaryExpression::build(primary)?),
            })
        })
        .map_infix(|left, op, right| {
            Ok(Node {
                line: op.line_col().0,
                column: op.line_col().1,
                value: Expression::Binary(BinaryExpression::build(left?, op, right?)?),
            })
        })
        .map_prefix(|op, right| {
            Ok(Node {
                line: op.line_col().0,
                column: op.line_col().1,
                value: Expression::Unary(UnaryExpression::build(op, right?)?),
            })
        })
        .parse(pairs)
}

impl NodeBuilder for Expression {
    fn build(pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        parse_expr(pairs).map(|node| node.value)
    }
}

impl NodeExecutor for Expression {
    fn eval(&self, _env: &mut ProcessEnv) -> AlthreadResult<Option<Literal>> {
        match self {
            Self::Binary(node) => node.eval(_env),
            Self::Unary(node) => node.eval(_env),
            Self::Primary(node) => node.eval(_env),
        }
    }
}

impl AstDisplay for Expression {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        match self {
            Self::Binary(node) => node.ast_fmt(f, prefix),
            Self::Unary(node) => node.ast_fmt(f, prefix),
            Self::Primary(node) => node.ast_fmt(f, prefix),
        }
    }
}
