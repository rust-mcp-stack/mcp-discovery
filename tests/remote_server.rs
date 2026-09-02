use mcp_discovery::{DiscoveryCommand, McpDiscovery, PrintOptions};

#[tokio::test]
#[ignore = "requires network access to a live remote MCP server"]
async fn discover_remote_streamable_http_server() {
    let options = DiscoveryCommand::Print(PrintOptions {
        template: None,
        template_file: None,
        template_string: None,
        template_url: None,
        cache_dir: None,
        log_level: None,
        mcp_server_cmd: Vec::new(),
        url: Some("https://gateway.mcpservers.org/yahoo-finance/mcp".to_string()),
        auth: Default::default(),
    });

    let mut discovery = McpDiscovery::new(options);
    let info = discovery
        .discover()
        .await
        .expect("failed to discover remote streamable HTTP server");

    assert_eq!(info.name, "yahoo-finance-mcp");
    assert!(
        info.tools.as_ref().is_some_and(|tools| !tools.is_empty()),
        "expected at least one tool from the remote server"
    );
}
