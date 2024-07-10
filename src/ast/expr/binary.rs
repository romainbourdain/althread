use pest::iterators::Pair;

use crate::{
    env::Environment,
    error::{AlthreadError, ErrorType},
    nodes::{
        datatype::DataType,
        expr::{
            binary::{BinExpr, BinOp},
            Expr, ExprKind,
        },
    },
    parser::Rule,
};

use super::ExprResult;

impl BinExpr {
    pub fn build(
        lhs: ExprResult,
        op: Pair<Rule>,
        rhs: ExprResult,
        env: &Environment,
    ) -> ExprResult {
        let (line, column) = op.line_col();
        let op = match op.as_rule() {
            Rule::add => BinOp::Add,
            Rule::sub => BinOp::Sub,
            Rule::mul => BinOp::Mul,
            Rule::div => BinOp::Div,
            Rule::modulo => BinOp::Mod,
            Rule::eq => BinOp::Eq,
            Rule::ne => BinOp::Ne,
            Rule::gt => BinOp::Gt,
            Rule::ge => BinOp::Ge,
            Rule::lt => BinOp::Lt,
            Rule::le => BinOp::Le,
            Rule::and => BinOp::And,
            Rule::or => BinOp::Or,
            rule => unreachable!("{:?}", rule),
        };
        let lhs = lhs?;
        let rhs = rhs?;

        DataType::from_bin_expr(&lhs, &op, &rhs, env)
            .map_err(|e| AlthreadError::error(ErrorType::TypeError, line, column, e))?;

        Ok(Expr {
            kind: ExprKind::Binary(Self {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            }),
            line,
            column,
        })
    }
}
