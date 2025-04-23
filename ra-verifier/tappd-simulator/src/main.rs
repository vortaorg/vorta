#[cfg(unix)]
use std::{fs::Permissions, os::unix::fs::PermissionsExt};

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use rocket::{
    figment::Figment,
    listener::{Bind, DefaultListener},
};
use rpc_service::AppState;

mod http_routes;
mod rocket_helper;
mod rpc_call;
mod rpc_service;
mod ra_tls;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("./app.crt"))]
    cert_file: String,

    #[arg(long, default_value_t = String::from("./app.key"))]
    key_file: String,

    // The listen address for the server, defaults to unix:/var/run/tappd.sock under Linux,
    // and tcp:0.0.0.0:8090 under Windows & Mac
    #[arg(short, long)]
    listen: Option<String>,

    // The port to listen on, defaults to 8090. It only applies to listening on IP addresses,
    // or once it specifies, it will auto switch to the listen mode.
    #[arg(short, long, default_value_t = 8090)]
    port: u16,
}

async fn run_internal(state: AppState, figment: Figment) -> Result<()> {
    let rocket = rocket::custom(figment)
        .mount("/", http_routes::internal_routes())
        .manage(state);
    let ignite = rocket
        .ignite()
        .await
        .map_err(|err| anyhow!("Failed to ignite rocket: {err}"))?;
    let endpoint = DefaultListener::bind_endpoint(&ignite)
        .map_err(|err| anyhow!("Failed to get endpoint: {err}"))?;
    let listener = DefaultListener::bind(&ignite)
        .await
        .map_err(|err| anyhow!("Failed to bind on {endpoint}: {err}"))?;
    #[cfg(unix)]
    if let Some(path) = endpoint.unix() {
        // Allow any user to connect to the socket
        fs_err::set_permissions(path, Permissions::from_mode(0o777))?;
    }
    ignite
        .launch_on(listener)
        .await
        .map_err(|err| anyhow!(err.to_string()))?;
    Ok(())
}

#[rocket::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let config = rocket::Config::figment()
        .merge(("workers", 8))
        .merge(("max_blocking", 64))
        .merge(("ident", "Tappd Server"))
        .merge(("temp_dir", "/tmp"))
        .merge(("keep_alive", 10))
        .merge(("log_level", "debug"))
        .merge(("address", args.listen.unwrap_or_else(|| {
            #[cfg(all(unix, not(target_os = "macos")))]
            {
                String::from("unix:tappd.sock")
            }
            #[cfg(any(windows, target_os = "macos"))]
            {
                String::from("0.0.0.0")
            }
        })))
        .merge(("port", args.port))
        .merge(("reuse", false))
        ;

    let state = AppState::new(args.cert_file, args.key_file).context("Failed to create app state")?;

    let _ = reqwest::get("https://wapo-testnet.phala.network/_/beacon").await;

    tokio::select!(
        res = run_internal(state.clone(), config) => res?,
    );
    Ok(())
}
