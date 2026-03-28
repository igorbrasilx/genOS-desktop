//! genos-mcp — MCP bus as system primitive
//! Unix socket server at /run/genos/mcp.sock
//! Day 16: full implementation begins

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
