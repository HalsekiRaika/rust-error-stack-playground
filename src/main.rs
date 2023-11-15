use std::fmt::{Display, Formatter};
use error_stack::{Context, ResultExt};
use rust_error_stack_playground::error::{ExecutionError, InitializationError};

#[tokio::main]
async fn main() -> error_stack::Result<(), ExecutionError> {
    println!("Hello, world!");

    let handler = Handler::init()
        .change_context_lazy(|| ExecutionError)
        .attach_printable_lazy(|| "this is execution error.".to_string())?;

    handler.repository()
        .get()
        .change_context_lazy(|| ExecutionError)
        .attach_printable_lazy(|| "cannot get item.".to_string())?;

    Ok(())
}

pub struct Handler;

impl Handler {
    pub fn init() -> error_stack::Result<Handler, InitializationError> {
        Ok(Handler)
    }
}

impl DependOnRepository for Handler {
    type Repository = RepositoryImpl;
    fn repository(&self) -> &Self::Repository {
        &RepositoryImpl
    }
}

#[derive(Debug)]
pub struct KernelError;

impl Display for KernelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error from kernel")
    }
}

impl Context for KernelError {}

#[derive(Debug)]
pub struct DriverError;

impl Display for DriverError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error from driver")
    }
}

impl Context for DriverError {}

pub trait Repository: 'static + Sync + Send {
    fn get(&self) -> error_stack::Result<(), KernelError>;
}

pub trait DependOnRepository: 'static + Sync + Send {
    type Repository: Repository;
    fn repository(&self) -> &Self::Repository;
}

pub struct RepositoryImpl;

impl Repository for RepositoryImpl {
    fn get(&self) -> error_stack::Result<(), KernelError> {
        InternalRepositoryImpl::get().change_context(KernelError)?;
        Ok(())
    }
}

pub struct InternalRepositoryImpl;

impl InternalRepositoryImpl {
    pub fn get() -> error_stack::Result<(), DriverError> {
        Err(DriverError)?
    }
}