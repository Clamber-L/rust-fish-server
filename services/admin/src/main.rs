use admin::init_router;
use anyhow::Result;
use axum::serve;
use tokio::net::TcpListener;
use tracing::info;
use tracing::metadata::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt::Layer, Layer as _};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::TRACE);
    tracing_subscriber::registry().with(layer).init();

    let addr = format!("0.0.0.0:{}", 8100);
    let tcp_listener = TcpListener::bind(addr).await?;

    info!("Server started on http://{}", tcp_listener.local_addr()?);

    serve(tcp_listener, init_router().await?).await?;
    Ok(())
}
