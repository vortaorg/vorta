// Custom error types for the mainframe component

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MainframeError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization/Deserialization error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Network error: {0}")]
    NetworkError(String), // Placeholder, could use reqwest::Error, tonic::Status, etc.

    #[error("Enclave operation failed: {0}")]
    EnclaveError(String),

    #[error("Enclave not initialized")]
    EnclaveNotInitialized,

    #[error("Attestation failed: {0}")]
    AttestationError(String),

    #[error("Attestation verification failed: {0}")]
    AttestationVerificationFailed(String),

    #[error("Scheduling error: {0}")]
    SchedulingError(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

// Implement conversion from other error types if needed
// Example:
// impl From<reqwest::Error> for MainframeError {
//     fn from(err: reqwest::Error) -> Self {
//         MainframeError::NetworkError(err.to_string())
//     }
// } 