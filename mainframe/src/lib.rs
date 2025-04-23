// Vorta Mainframe Library Crate

// Declare modules
pub mod enclave;
pub mod attestation;
pub mod error;
pub mod config; // Example: For loading configuration
pub mod scheduler; // Example: Scheduler logic
pub mod api; // Example: API handling (RPC, REST)

// Re-export key types or functions if desired
pub use error::MainframeError;

// Initialization function (example)
pub fn initialize() -> Result<(), MainframeError> {
    log::info!("Initializing mainframe library...");
    // Load config, set up resources, etc.
    config::load_config()?;
    Ok(())
} 