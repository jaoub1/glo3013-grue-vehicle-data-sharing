use std::net::SocketAddr;

use anyhow::Context;
use axum::Router;
use axum_prototype::setup::generate_router;
use tokio::net::TcpListener;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(fmt::layer().event_format(fmt::format().with_target(false)))
        .try_init()
        .context("Failed setuping the logging system")?;

    let custom_server = generate_router();

    let addr = ([127, 0, 0, 1], 8080);

    run(custom_server, addr).await
}

async fn run(router: Router, addr: ([u8; 4], u16)) -> anyhow::Result<()> {
    let address = SocketAddr::from(addr);
    let listener = TcpListener::bind(&address)
        .await
        .context("Failed binding TCP listener")?;

    axum::serve(listener, router.into_make_service())
        .await
        .context("Failed serving the router")
}
