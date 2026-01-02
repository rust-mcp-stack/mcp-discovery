use async_trait::async_trait;
use rust_mcp_sdk::schema::RpcError;
use rust_mcp_sdk::{mcp_client::ClientHandler, McpClient};

pub struct MyClientHandler;

#[async_trait]
impl ClientHandler for MyClientHandler {
    async fn handle_process_error(
        &self,
        error_message: String,
        runtime: &dyn McpClient,
    ) -> std::result::Result<(), RpcError> {
        if !runtime.is_shut_down().await {
            eprintln!("{error_message}");
        }
        Ok(())
    }
}
