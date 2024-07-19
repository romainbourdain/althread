use crate::{
    ast::{
        expr::primary::PrimaryExpr, stmt::assign::assign_binary::AssignBinary,
        token::binary_assign_op::BinaryAssignOp,
    },
    env::Environment,
    error::{AlthreadError, ErrorType},
};

impl AssignBinary {
    pub fn eval(&self, env: &mut Environment) -> Result<(), AlthreadError> {
        let symbol = env.get_symbol(&self.left).map_err(|e| {
            AlthreadError::error(ErrorType::VariableError, self.line, self.column, e)
        })?;
        if let Some(symbol_value) = &symbol.value {
            let value = match self.op {
                BinaryAssignOp::Assign => self.right.eval(env)?,
                BinaryAssignOp::AddAssign => match (self.right.eval(env)?, symbol_value) {
                    (PrimaryExpr::Int(value), PrimaryExpr::Int(cur)) => {
                        PrimaryExpr::Int(cur + value)
                    }
                    _ => unreachable!(),
                },
                BinaryAssignOp::SubAssign => match (self.right.eval(env)?, symbol_value) {
                    (PrimaryExpr::Int(value), PrimaryExpr::Int(cur)) => {
                        PrimaryExpr::Int(cur - value)
                    }
                    _ => unreachable!(),
                },
                BinaryAssignOp::MulAssign => match (self.right.eval(env)?, symbol_value) {
                    (PrimaryExpr::Int(value), PrimaryExpr::Int(cur)) => {
                        PrimaryExpr::Int(cur * value)
                    }
                    _ => unreachable!(),
                },
                BinaryAssignOp::DivAssign => match (self.right.eval(env)?, symbol_value) {
                    (PrimaryExpr::Int(value), PrimaryExpr::Int(cur)) => {
                        PrimaryExpr::Int(cur / value)
                    }
                    _ => unreachable!(),
                },
                BinaryAssignOp::ModAssign => match (self.right.eval(env)?, symbol_value) {
                    (PrimaryExpr::Int(value), PrimaryExpr::Int(cur)) => {
                        PrimaryExpr::Int(cur % value)
                    }
                    _ => unreachable!(),
                },
            };

            env.update_symbol(&self.left, value).map_err(|e| {
                AlthreadError::error(ErrorType::VariableError, self.line, self.column, e)
            })?;

            Ok(())
        } else {
            unreachable!()
        }
    }
}
