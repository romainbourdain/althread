use pest::iterators::Pair;

use crate::{
    env::Environment,
    error::{AlthreadError, ErrorType},
    nodes::{
        datatype::DataType,
        expr::{
            unary::{UnExpr, UnOp},
            Expr, ExprKind,
        },
    },
    parser::Rule,
};

use super::ExprResult;

impl UnExpr {
    pub fn build(op: Pair<Rule>, rhs: ExprResult, env: &Environment) -> ExprResult {
        let (line, column) = op.line_col();
        let op = match op.as_rule() {
            Rule::not => UnOp::Not,
            Rule::neg => UnOp::Neg,
            rule => unreachable!("{:?}", rule),
        };
        let rhs = rhs?;

        DataType::from_un_expr(&op, &rhs, env)
            .map_err(|e| AlthreadError::error(ErrorType::TypeError, line, column, e))?;

        Ok(Expr {
            kind: ExprKind::Unary(Self {
                op,
                rhs: Box::new(rhs),
            }),
            line,
            column,
        })
    }
}
