use pest::iterators::{Pair, Pairs};

use crate::{
    env::{symbol_table::DataType, Environment},
    error::{AlthreadError, AlthreadResult, ErrorType},
    parser::Rule,
};

use super::Ast;

impl<'a> Ast<'a> {
    pub fn check(&self, env: &mut Environment) -> AlthreadResult<()> {
        for (_, pairs) in &self.process_bricks {
            Self::check_pair(pairs.clone(), env)?;
        }
        Ok(())
    }

    fn check_pair(pairs: Pairs<'a, Rule>, env: &mut Environment) -> AlthreadResult<()> {
        for pair in pairs {
            match pair.as_rule() {
                Rule::decl => Self::check_decl(pair, env)?,
                Rule::expr => {
                    Self::check_expr(pair)?;
                }
                rule => unreachable!("Unexpected rule: {:?}", rule),
            }
        }

        Ok(())
    }

    fn check_decl(pair: Pair<'a, Rule>, env: &mut Environment) -> AlthreadResult<()> {
        let mut inner_pairs = pair.into_inner();
        let mut data_type = DataType::Void;
        let mut expr_type = DataType::Void;

        while let Some(inner_pair) = inner_pairs.next() {
            match inner_pair.as_rule() {
                Rule::DATATYPE => data_type = DataType::from_str(inner_pair.as_str()),
                Rule::expr => expr_type = Self::check_expr(inner_pair)?,
                _ => unreachable!(),
            }
        }

        Ok(())
    }

    fn check_expr(pair: Pair<'a, Rule>) -> AlthreadResult<DataType> {
        let (line, column) = pair.line_col();
        match pair.as_rule() {
            Rule::primary => Ok(Self::check_primary(pair.into_inner().next().unwrap())?),
            Rule::unary => Ok(Self::check_unary(pair.into_inner())?),
            Rule::expr
            | Rule::logical_or
            | Rule::logical_and
            | Rule::equality
            | Rule::comparison
            | Rule::term
            | Rule::factor => Ok(Self::check_binary(pair.into_inner())?),
            rule => {
                return Err(AlthreadError::error(
                    ErrorType::SyntaxError,
                    line,
                    column,
                    format!("Unexpected rule: {:?}", rule),
                ))
            }
        }
    }

    pub fn check_primary(pair: Pair<Rule>) -> AlthreadResult<DataType> {
        use Rule::*;

        Ok(match pair.as_rule() {
            NULL => DataType::Void,
            BOOLEAN => DataType::Bool,
            INTEGER => DataType::Int,
            FLOAT => DataType::Float,
            STRING => DataType::String,
            rule => Err(AlthreadError::error(
                ErrorType::SyntaxError,
                pair.line_col().0,
                pair.line_col().1,
                format!("Unexpected rule: {:?}", rule),
            ))?,
        })
    }

    pub fn check_unary(mut pairs: Pairs<'a, Rule>) -> AlthreadResult<DataType> {
        use Rule::*;
        let pair = pairs.next().unwrap();
        match pair.as_rule() {
            unary_op => {
                let data_type = Self::check_unary(pairs)?;
                let error_message = |op: &str, data_type: DataType| {
                    AlthreadError::error(
                        ErrorType::TypeError,
                        pair.line_col().0,
                        pair.line_col().1,
                        format!("Wrong type for {} unary operator: {}", op, data_type),
                    )
                };

                match pair.as_str() {
                    "-" if !data_type.is_numeric() => Err(error_message(pair.as_str(), data_type)),
                    "!" if data_type != DataType::Bool => {
                        Err(error_message(pair.as_str(), data_type))
                    }
                    _ => Ok(data_type),
                }
            }
            primary => Ok(Self::check_expr(pair)?),
            rule => unimplemented!("Unexpected rule: {:?}", rule),
        }
    }

    pub fn check_binary(mut pairs: Pairs<'a, Rule>) -> AlthreadResult<DataType> {
        let error_message = |op: Pair<Rule>, data_type: DataType| {
            AlthreadError::error(
                ErrorType::TypeError,
                op.line_col().0,
                op.line_col().1,
                format!("Wrong type for {} operator: {}", op.as_str(), data_type),
            )
        };

        let left_type = Self::check_expr(pairs.next().unwrap())?;
        if let Some(op) = pairs.next() {
            let right_type = Self::check_binary(pairs)?;
            match op.as_str() {
                "+" | "-" | "*" | "/" | "%" if !left_type.is_numeric() => {
                    Err(error_message(op, left_type))
                }
                "<" | ">" | "<=" | ">=" if !left_type.is_numeric() => {
                    Err(error_message(op, left_type))
                }
                "&&" | "||" if left_type != DataType::Bool => Err(error_message(op, left_type)),
                "==" | "!=" | "<" | ">" | "<=" | ">=" => Ok(DataType::Bool),
                _ if right_type != left_type => Err(AlthreadError::error(
                    ErrorType::TypeError,
                    op.line_col().0,
                    op.line_col().1,
                    format!(
                        "{} operation between {} and {} is not allowed",
                        op.as_str(),
                        left_type,
                        right_type
                    ),
                )),
                _ => Ok(left_type),
            }
        } else {
            Ok(left_type)
        }
    }
}
