// Placeholder module for configuration management

use serde::Deserialize;
use config::{Config, ConfigError, File, Environment};
use std::sync::OnceLock;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub log_level: String,
    pub enclave_path: Option<String>, // Path to the enclave binary
    pub attestation_service_url: Option<String>,
    pub scheduler_interval_secs: u64,
    pub api_listen_address: String,
    // Add other configuration parameters as needed
}

static SETTINGS: OnceLock<Settings> = OnceLock::new();

/// Loads configuration from file and environment variables.
pub fn load_config() -> Result<(), super::error::MainframeError> {
    let default_config_path = "config/mainframe.yaml"; // Default path relative to CWD
    log::info!("Attempting to load configuration from {}", default_config_path);

    let s = Config::builder()
        // Start off by merging in the default configuration file
        .add_source(File::with_name(default_config_path).required(false))
        // Add in settings from the environment (with a prefix of VORTA_)
        // Eg.. `VORTA_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(Environment::with_prefix("VORTA_MAINFRAME").separator("__"))
        .build()
        .map_err(|e| super::error::MainframeError::ConfigError(e.to_string()))?;

    let settings: Settings = s.try_deserialize()
        .map_err(|e| super::error::MainframeError::ConfigError(e.to_string()))?;

    if SETTINGS.set(settings).is_err() {
        log::error!("Configuration already initialized");
        return Err(super::error::MainframeError::ConfigError("Configuration already initialized".to_string()));
    }

    log::info!("Configuration loaded successfully.");
    Ok(())
}

/// Get a reference to the global settings.
/// Panics if `load_config` has not been called successfully.
pub fn get_settings() -> &'static Settings {
    SETTINGS.get().expect("Configuration not initialized. Call load_config first.")
} 