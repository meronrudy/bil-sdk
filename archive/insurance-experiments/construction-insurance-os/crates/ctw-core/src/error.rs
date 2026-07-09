//! Core error types.

/// Errors that can occur in the core type system.
#[derive(Clone, Debug, PartialEq)]
pub enum CoreError {
    /// A value was outside its valid bounds.
    OutOfBounds {
        name: &'static str,
        value: f64,
        min: f64,
        max: f64,
    },
    /// Min bound exceeds max bound.
    InvalidBounds {
        min: f64,
        max: f64,
    },
    /// A required field was missing.
    MissingField {
        name: &'static str,
    },
}

impl core::fmt::Display for CoreError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OutOfBounds { name, value, min, max } => {
                write!(f, "{name} value {value} outside [{min}, {max}]")
            }
            Self::InvalidBounds { min, max } => {
                write!(f, "invalid bounds: min {min} > max {max}")
            }
            Self::MissingField { name } => {
                write!(f, "missing required field: {name}")
            }
        }
    }
}
