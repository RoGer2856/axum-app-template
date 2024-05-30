mod app_state;
mod cli;
mod endpoints;
mod error;
mod fn_decorators;
mod messages;
mod model;
mod syn;

use std::net::ToSocketAddrs;

use app_state::AppState;
use axum_helpers::app::AxumApp;

use clap::Parser;
use cli::Cli;
use error::BoxError;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let cli = Cli::parse();

    env_logger::builder()
        // .filter_level(log::LevelFilter::max())
        .filter_level(log::LevelFilter::Info)
        .init();

    log::info!("Starting application!");

    let mut secret = [0; 32];
    getrandom::getrandom(&mut secret)?;
    let state = AppState::new(secret);

    let mut app = AxumApp::new(state.clone());
    for addr in cli.listener_address.to_socket_addrs().unwrap() {
        let _ = app
            .spawn_server(addr)
            .await
            .inspect_err(|e| log::error!("error = {e:?}"));
    }

    // periodic tasks
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(10));
        loop {
            interval.tick().await;

            // tasks to be executed
        }
    });

    app.join().await;

    Ok(())
}
