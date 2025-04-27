use async_trait::async_trait;
use rust_mcp_schema::RpcError;
use rust_mcp_sdk::{mcp_client::ClientHandler, McpClient};

pub struct MyClientHandler;

#[async_trait]
impl ClientHandler for MyClientHandler {
    async fn handle_process_error(
        &self,
        _error_message: String,
        _: &dyn McpClient,
    ) -> std::result::Result<(), RpcError> {
        Ok(())
    }
}
