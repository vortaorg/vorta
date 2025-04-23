// Vorta Mainframe Entry Point

// Use the library crate
use vorta_mainframe_lib as mainframe;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    log::info!("Starting Vorta Mainframe...");

    // Check if running in SGX mode (example using feature flag)
    if cfg!(feature = "sgx_mode") {
        log::info!("Running in SGX mode.");
        // Perform SGX-specific initialization if needed
        mainframe::enclave::initialize_enclave()?;
    } else {
        log::warn!("Running in non-SGX mode (simulation/development).");
    }

    // Placeholder: Start the main service (e.g., RPC server, scheduler loop)
    log::info!("Initializing services...");
    // let scheduler = mainframe::scheduler::Scheduler::new().await?;
    // let api_server = mainframe::api::ApiServer::new().await?;

    // Example: Perform a sample attestation flow
    match mainframe::attestation::perform_remote_attestation("example_worker_url").await {
        Ok(report) => log::info!("Attestation successful: {:?}", report),
        Err(e) => log::error!("Attestation failed: {}", e),
    }

    // Placeholder: Run the main event loop or server
    log::info!("Vorta Mainframe running.");
    // tokio::select! {
    //     _ = scheduler.run() => { log::info!("Scheduler finished."); }
    //     _ = api_server.run() => { log::info!("API server finished."); }
    // }

    // Keep running indefinitely for a server
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        log::debug!("Mainframe heartbeat...");
    }

    // Ok(())
} 