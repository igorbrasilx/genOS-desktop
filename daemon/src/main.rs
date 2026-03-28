//! genos-daemon — GENOS central daemon
//! Manages AI inference, MCP bus coordination and system state.

use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    info!("GENOS daemon starting...");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));
    // Day 8: Unix socket server
    // Day 10: llama.cpp integration
    // Day 15: MCP bus connection
    Ok(())
}
