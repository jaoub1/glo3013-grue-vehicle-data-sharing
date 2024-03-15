use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Context;
use axum::Router;
use axum_prototype::setup::generate_router;
use clap::Parser;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Address of the TCP connection
    #[arg(short, long, default_value_t = [0; 4].into())]
    address: Ipv4Addr,
    /// TCP port number
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
    /// Specific lock UUIDv4
    #[arg(short, long, default_value = None)]
    lock_uuid: Option<Uuid>,
}

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
    info!("Hello World !");

    let args = Args::parse();
    match args.lock_uuid {
        Some(uuid) => info!("Reset allowed with UUID: {}", uuid),
        None => info!("Reset always allowed because no UUID supplied"),
    }

    let custom_server = generate_router(args.lock_uuid);

    run(custom_server, (args.address, args.port)).await
}

async fn run(router: Router, addr: (Ipv4Addr, u16)) -> anyhow::Result<()> {
    let address = SocketAddr::from(addr);
    let listener = TcpListener::bind(&address)
        .await
        .context("Failed binding TCP listener")?;

    info!("Listening on {}", address);
    axum::serve(listener, router.into_make_service())
        .await
        .context("Failed serving the router")
}
