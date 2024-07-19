use crate::{
    ast::{expr::primary::PrimaryExpr, stmt::assign::Assign, token::assign_op::AssignOp},
    env::Environment,
    error::{AlthreadError, ErrorType},
};

impl Assign {
    pub fn eval(&self, env: &mut Environment) -> Result<(), AlthreadError> {
        let symbol = env.get_symbol(&self.identifier).map_err(|e| {
            AlthreadError::error(ErrorType::VariableError, self.line, self.column, e)
        })?;
        if let Some(symbol_value) = &symbol.value {
            let value = match self.op {
                AssignOp::Assign => self.value.eval(env)?,
                AssignOp::AddAssign => match (self.value.eval(env)?, symbol_value) {
                    (PrimaryExpr::Int(value), PrimaryExpr::Int(cur)) => {
                        PrimaryExpr::Int(cur + value)
                    }
                    _ => unreachable!(),
                },
                AssignOp::SubAssign => match (self.value.eval(env)?, symbol_value) {
                    (PrimaryExpr::Int(value), PrimaryExpr::Int(cur)) => {
                        PrimaryExpr::Int(cur - value)
                    }
                    _ => unreachable!(),
                },
                AssignOp::MulAssign => match (self.value.eval(env)?, symbol_value) {
                    (PrimaryExpr::Int(value), PrimaryExpr::Int(cur)) => {
                        PrimaryExpr::Int(cur * value)
                    }
                    _ => unreachable!(),
                },
                AssignOp::DivAssign => match (self.value.eval(env)?, symbol_value) {
                    (PrimaryExpr::Int(value), PrimaryExpr::Int(cur)) => {
                        PrimaryExpr::Int(cur / value)
                    }
                    _ => unreachable!(),
                },
                AssignOp::ModAssign => match (self.value.eval(env)?, symbol_value) {
                    (PrimaryExpr::Int(value), PrimaryExpr::Int(cur)) => {
                        PrimaryExpr::Int(cur % value)
                    }
                    _ => unreachable!(),
                },
            };

            env.update_symbol(&self.identifier, value).map_err(|e| {
                AlthreadError::error(ErrorType::VariableError, self.line, self.column, e)
            })?;

            Ok(())
        } else {
            unreachable!()
        }
    }
}
