use lazy_static::lazy_static;
use pest::{iterators::Pair, pratt_parser::PrattParser};

use crate::{
    env::Environment,
    error::{AlthreadError, ErrorType},
    parser::Rule,
};

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
            .op(Op::infix(Rule::mul, Assoc::Left)
                | Op::infix(Rule::div, Assoc::Left)
                | Op::infix(Rule::modulo, Assoc::Left))
            .op(Op::prefix(Rule::not))
            .op(Op::prefix(Rule::sub))
    };
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub enum ExprKind {
    Primary(PrimaryExpr),
    Binary(BinExpr),
    Unary(UnExpr),
}

impl Expr {
    pub fn new(kind: ExprKind) -> Self {
        Self {
            kind,
            line: 0,
            column: 0,
        }
    }
    pub fn build(pair: Pair<Rule>, env: &Environment) -> ExprResult {
        PRATT_PARSER
            .map_primary(|pair| PrimaryExpr::build(pair, env))
            .map_infix(|lhs, op, rhs| BinExpr::build(lhs, op, rhs, env))
            .map_prefix(|op, rhs| UnExpr::build(op, rhs, env))
            .parse(pair.into_inner())
    }

    pub fn default(datatype: &DataType) -> Self {
        let primary = match datatype {
            DataType::Int => PrimaryExpr::Int(0),
            DataType::Float => PrimaryExpr::Float(0.0),
            DataType::Bool => PrimaryExpr::Bool(false),
            DataType::String => PrimaryExpr::String("".to_string()),
            DataType::Void => PrimaryExpr::Null,
        };

        Self::new(ExprKind::Primary(primary))
    }
}

#[derive(Debug, Clone)]
pub enum PrimaryExpr {
    Null,
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Identifier(String),
}

impl PrimaryExpr {
    pub fn build(pair: Pair<Rule>, env: &Environment) -> ExprResult {
        let (line, column) = pair.line_col();
        let expr = match pair.as_rule() {
            Rule::NULL => Self::Null,
            Rule::INTEGER => Self::Int(pair.as_str().parse::<i64>().unwrap()),
            Rule::FLOAT => Self::Float(pair.as_str().parse::<f64>().unwrap()),
            Rule::BOOLEAN => Self::Bool(pair.as_str() == "true"),
            Rule::STRING => Self::String(pair.as_str().to_string()),
            Rule::IDENTIFIER => {
                let identifier = pair.as_str().to_string();
                let symbol = env
                    .get_symbol(&identifier)
                    .map_err(|e| AlthreadError::error(ErrorType::VariableError, line, column, e))?;
                match &symbol.value {
                    Some(value) => Ok(value.clone()),
                    None => Ok(PrimaryExpr::Identifier(identifier)),
                }?
            }
            _ => unreachable!("{:?}", pair.as_rule()),
        };

        Ok(Expr {
            kind: ExprKind::Primary(expr),
            line: pair.as_span().start_pos().line_col().0,
            column: pair.as_span().start_pos().line_col().1,
        })
    }
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
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
pub struct BinExpr {
    pub lhs: Box<Expr>,
    pub op: BinOp,
    pub rhs: Box<Expr>,
}

impl BinExpr {
    fn build(lhs: ExprResult, op: Pair<Rule>, rhs: ExprResult, env: &Environment) -> ExprResult {
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
            _ => unreachable!("{:?}", op),
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

#[derive(Debug, Clone)]
pub enum UnOp {
    Not,
    Neg,
}

#[derive(Debug)]
pub struct UnExpr {
    pub op: UnOp,
    pub rhs: Box<Expr>,
}

impl UnExpr {
    pub fn build(op: Pair<Rule>, rhs: ExprResult, env: &Environment) -> ExprResult {
        let (line, column) = op.line_col();
        let op = match op.as_rule() {
            Rule::not => UnOp::Not,
            Rule::sub => UnOp::Neg,
            _ => unreachable!("{:?}", op),
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
