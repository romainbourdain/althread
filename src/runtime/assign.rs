use crate::{
    env::Environment,
    error::{AlthreadError, ErrorType},
    nodes::assign::Assign,
};

impl Assign {
    pub fn eval(&self, env: &mut Environment) -> Result<(), AlthreadError> {
        env.update_symbol(&self.identifier, self.value.eval(env)?)
            .map_err(|e| {
                AlthreadError::error(ErrorType::VariableError, self.line, self.column, e)
            })?;

        Ok(())
    }
}
