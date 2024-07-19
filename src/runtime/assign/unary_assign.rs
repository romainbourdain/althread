use crate::{
    ast::{
        expr::primary::PrimaryExpr, stmt::assign::assign_unary::AssignUnary,
        token::unary_assign_op::UnaryAssignOp,
    },
    env::Environment,
    error::{AlthreadError, ErrorType},
};

impl AssignUnary {
    pub fn eval(&self, env: &mut Environment) -> Result<(), AlthreadError> {
        let symbol = env.get_symbol(&self.left).map_err(|e| {
            AlthreadError::error(ErrorType::VariableError, self.line, self.column, e)
        })?;
        if let Some(symbol_value) = &symbol.value {
            let value = match self.op {
                UnaryAssignOp::Increment => match symbol_value {
                    PrimaryExpr::Int(value) => PrimaryExpr::Int(value + 1),
                    _ => unreachable!(),
                },
                UnaryAssignOp::Decrement => match symbol_value {
                    PrimaryExpr::Int(value) => PrimaryExpr::Int(value - 1),
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
