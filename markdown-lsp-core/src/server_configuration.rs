use lsp_types::{HoverProviderCapability, ServerCapabilities};

// TODO: get this information from rust cargo workspace
pub const SERVER_NAME: &str = "markdown-lsp-server";
pub const SERVER_VERSION: &str = "0.1";

pub fn server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        ..Default::default()
    }
}
