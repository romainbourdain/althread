use crate::{
    env::Environment,
    error::{AlthreadError, ErrorType},
    nodes::expr::primary::PrimaryExpr,
};

impl PrimaryExpr {
    pub fn eval(&self, env: &Environment) -> Result<PrimaryExpr, AlthreadError> {
        match self {
            PrimaryExpr::Identifier(ident) => {
                let symbol = env
                    .get_symbol(ident)
                    .map_err(|err| AlthreadError::error(ErrorType::RuntimeError, 0, 0, err))?;

                if let Some(value) = symbol.value.as_ref() {
                    Ok(value.clone())
                } else {
                    unreachable!("symbol has no value");
                }
            }
            _ => Ok(self.clone()),
        }
    }
}
