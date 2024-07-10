use pest::iterators::Pair;

use crate::{
    env::Environment,
    error::{AlthreadError, ErrorType},
    nodes::{
        datatype::DataType,
        expr::{
            binary::{BinExpr, BinOp},
            primary::PrimaryExpr,
            Expr, ExprKind,
        },
    },
    parser::Rule,
};

use super::ExprResult;

macro_rules! match_bin {
    ([$(($variant:ident, $out:ident)),*], $lhs:expr, $rhs:expr, $op:expr) => {
        match ($lhs, $rhs) {
            $(
                (PrimaryExpr::$variant(a), PrimaryExpr::$variant(b)) => Ok(PrimaryExpr::$out($op(a, b))),
            )*
            _ => unreachable!(),
        }
    };
}

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

    pub fn eval(&self, env: &Environment) -> Result<PrimaryExpr, AlthreadError> {
        let lhs = self.lhs.eval(env)?;
        let rhs = self.rhs.eval(env)?;

        match self.op {
            BinOp::Add => match_bin!([(Int, Int), (Float, Float)], lhs, rhs, |a, b| a + b),
            BinOp::Sub => match_bin!([(Int, Int), (Float, Float)], lhs, rhs, |a, b| a - b),
            BinOp::Mul => match_bin!([(Int, Int), (Float, Float)], lhs, rhs, |a, b| a * b),
            BinOp::Div => match_bin!([(Int, Int), (Float, Float)], lhs, rhs, |a, b| a / b),
            BinOp::Mod => match_bin!([(Int, Int), (Float, Float)], lhs, rhs, |a, b| a % b),
            BinOp::Eq => match_bin!([(Int, Bool), (Float, Bool)], lhs, rhs, |a, b| a == b),
            BinOp::Ne => match_bin!([(Int, Bool), (Float, Bool)], lhs, rhs, |a, b| a != b),
            BinOp::Gt => match_bin!([(Int, Bool), (Float, Bool)], lhs, rhs, |a, b| a > b),
            BinOp::Ge => match_bin!([(Int, Bool), (Float, Bool)], lhs, rhs, |a, b| a >= b),
            BinOp::Lt => match_bin!([(Int, Bool), (Float, Bool)], lhs, rhs, |a, b| a < b),
            BinOp::Le => match_bin!([(Int, Bool), (Float, Bool)], lhs, rhs, |a, b| a <= b),
            BinOp::Or => match_bin!([(Bool, Bool)], lhs, rhs, |a, b| a || b),
            BinOp::And => match_bin!([(Bool, Bool)], lhs, rhs, |a, b| a && b),
        }
    }
}
