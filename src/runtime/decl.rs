use crate::{
    env::Environment,
    error::{AlthreadError, ErrorType},
    nodes::decl::Decl,
};

impl Decl {
    pub fn eval(&self, env: &mut Environment) -> Result<(), AlthreadError> {
        env.insert_symbol(
            self.identifier.clone(),
            self.datatype.clone(),
            self.mutable,
            Some(self.value.eval(env)?),
        )
        .map_err(|e| AlthreadError::error(ErrorType::VariableError, self.line, self.column, e))?;

        Ok(())
    }
}
