use thiserror::Error;

#[derive(Error, Debug)]
pub enum ActuarialError {
    #[error("insufficient data: {0}")]
    InsufficientData(String),
    #[error("model convergence failure after {iterations} iterations")]
    ConvergenceFailure { iterations: usize },
    #[error("invalid expense loading: {0}")]
    InvalidExpenseLoading(String),
    #[error("bounds violation: {0}")]
    BoundsViolation(String),
    #[error("missing required field: {0}")]
    MissingField(String),
}
