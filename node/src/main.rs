#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!(
        version = env!("CARGO_PKG_VERSION"),
        pid = std::process::id(),
        "scraps-node starting"
    );
}
