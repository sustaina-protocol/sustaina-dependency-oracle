use std::fmt;

#[derive(Debug)]
pub enum CliError {
    FileNotFound(String),
    InvalidPercentage(u32),
    NoRegisteredDependencies,
    RegistryResolutionFailed(String),
    InvalidStellarAddress(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::FileNotFound(path) => {
                write!(f, "File not found: {}", path)
            }
            CliError::InvalidPercentage(pct) => {
                write!(f, "Invalid split percentage: {}. Must be between 1 and 100.", pct)
            }
            CliError::NoRegisteredDependencies => {
                write!(f, "No dependencies are registered on the Sustaina Registry.")
            }
            CliError::RegistryResolutionFailed(msg) => {
                write!(f, "Failed to resolve dependencies from registry: {}", msg)
            }
            CliError::InvalidStellarAddress(addr) => {
                write!(f, "Invalid Stellar address: {}", addr)
            }
        }
    }
}

impl std::error::Error for CliError {}
