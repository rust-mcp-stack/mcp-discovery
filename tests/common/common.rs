use clap::Parser;
use mcp_discovery::{CommandArguments, McpCapabilities, McpServerInfo};
use rust_mcp_sdk::macros::JsonSchema;

// Helper function to parse arguments from a vector of strings
pub fn parse_args(args: Vec<&str>) -> CommandArguments {
    CommandArguments::parse_from(args)
}

pub fn default_mcp_server_info() -> McpServerInfo {
    McpServerInfo {
        name: Default::default(),
        version: Default::default(),
        capabilities: McpCapabilities {
            tools: false,
            prompts: false,
            resources: false,
            logging: false,
            experimental: false,
        },
        tools: Default::default(),
        prompts: Default::default(),
        resources: Default::default(),
        resource_templates: Default::default(),
    }
}

#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, JsonSchema)]
/// Represents a text replacement operation.
pub struct EditOperation {
    /// Text to search for - must match exactly.
    #[serde(rename = "oldText")]
    pub old_text: String,
    #[serde(rename = "newText")]
    /// Text to replace the matched text with.
    pub new_text: String,
}
