use crate::{env::Environment, error::AlthreadError, nodes::assign::Assign};

impl Assign {
    pub fn eval(&self, env: &mut Environment) -> Result<(), AlthreadError> {
        // TODO: Implement assignment evaluation
        unimplemented!();
    }
}
