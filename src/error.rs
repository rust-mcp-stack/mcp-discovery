use rust_mcp_sdk::error::McpSdkError;
use thiserror::Error;

pub type DiscoveryResult<T> = core::result::Result<T, DiscoveryError>;

#[derive(Debug, Error)]
pub enum DiscoveryError {
    #[error("The MCP Server failed to initialize successfully.")]
    ServerNotInitialized,
    #[error("{0}")]
    InvalidSchema(String),
    #[error("{0}")]
    ParseTemplate(String),
    #[error(
        "Server details are not available. please ensure the discover() method is called first."
    )]
    NotDiscovered,
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    McpSdkError(#[from] McpSdkError),
    #[error("{0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("{0}")]
    RenderError(#[from] handlebars::RenderError),
    #[error("{0}")]
    RegexError(#[from] regex::Error),
}
