mod caps;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!(
        version = env!("CARGO_PKG_VERSION"),
        pid = std::process::id(),
        "scraps-node starting"
    );

    match caps::detect() {
        Ok(caps) => tracing::info!(
            total_ram_bytes = caps.total_ram_bytes,
            arch = caps.arch,
            kvm_available = caps.kvm_available,
            "capability detection complete"
        ),
        Err(err) => tracing::error!(error = %err, "capability detection failed"),
    }
}
