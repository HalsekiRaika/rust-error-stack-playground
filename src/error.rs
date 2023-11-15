use std::fmt::Display;
use error_stack::Context;

#[derive(Debug)]
pub struct ExecutionError;

impl Display for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to execute")
    }
}

impl Context for ExecutionError {}


#[derive(Debug)]
pub struct InitializationError;

impl Display for InitializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to initialize")
    }
}

impl Context for InitializationError {}