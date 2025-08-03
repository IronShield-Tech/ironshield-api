use ironshield::handler::error::ErrorHandler;

/// Type alias for function signatures.
#[allow(dead_code)]
pub type ResultHandler<T> = Result<T, ErrorHandler>;