//! A lightweight CLI tool for discovering and documenting MCP Server capabilities.

pub mod error;
mod handler;
mod render_template;
mod schema;
mod std_output;
mod templates;
mod types;
mod utils;

use rust_mcp_sdk::error::McpSdkError;
use rust_mcp_sdk::mcp_client::McpClientOptions;
use rust_mcp_sdk::ToMcpClientHandler;
use serde_json::{to_value, Map, Value};
pub use templates::OutputTemplate;
pub use types::{
    DiscoveryCommand, LogLevel, McpCapabilities, McpServerInfo, McpToolMeta, ParamTypes,
    PrintOptions, Template, WriteOptions,
};

use crate::types::McpTaskSupport;
use colored::Colorize;
use error::{DiscoveryError, DiscoveryResult};
use handler::MyClientHandler;
use render_template::{detect_render_markers, render_template};
use rust_mcp_sdk::schema::{
    ClientCapabilities, ClientElicitation, ClientRoots, ClientSampling, ClientTaskElicitation,
    ClientTaskSampling, ClientTasks, Implementation, InitializeRequestParams,
    PaginatedRequestParams, Prompt, ProtocolVersion, Resource, ResourceTemplate,
};
use rust_mcp_sdk::{
    error::SdkResult,
    mcp_client::{client_runtime, ClientRuntime},
    McpClient, StdioTransport, TransportOptions,
};
use schema::tool_params;
use std::io::stdout;
use std::sync::Arc;
use std_output::{print_header, print_list, print_summary};

/// Core struct representing the discovery mechanism for the MCP server.
pub struct McpDiscovery {
    /// Discovery action and its options
    options: DiscoveryCommand,
    /// Collected server capabilities and metadata
    pub server_info: Option<McpServerInfo>,
}

impl McpDiscovery {
    pub fn new(options: DiscoveryCommand) -> Self {
        Self {
            options,
            server_info: None,
        }
    }

    /// Entry point to execute the discovery workflow based on the command.
    pub async fn start(&mut self) -> DiscoveryResult<()> {
        // launch mcp server and discover capabilities

        self.discover().await?;

        match &self.options {
            DiscoveryCommand::Create(create_options) => {
                self.create_document(create_options).await?;
            }
            DiscoveryCommand::Update(update_options) => {
                self.update_document(update_options).await?;
            }
            DiscoveryCommand::Print(print_options) => {
                self.print_server_capabilities(print_options).await?;
            }
        };
        Ok(())
    }

    /// Prints MCP server capabilities using a specific template or default view.
    pub async fn print_server_capabilities(
        &self,
        print_options: &PrintOptions,
    ) -> DiscoveryResult<()> {
        let server_info = self
            .server_info
            .as_ref()
            .ok_or(DiscoveryError::NotDiscovered)?;

        let template = print_options.match_template()?;

        match template {
            OutputTemplate::None => {
                self.print_server_details()?;
            }
            _ => {
                let content = template.render_template(server_info)?;
                println!("{content}");
            }
        }

        Ok(())
    }

    /// Creates a new file using a specified template and discovered server info.
    pub async fn create_document(&self, create_options: &WriteOptions) -> DiscoveryResult<()> {
        tracing::trace!("Creating '{}' ", create_options.filename.to_string_lossy());

        let server_info = self
            .server_info
            .as_ref()
            .ok_or(DiscoveryError::NotDiscovered)?;

        let template = create_options.match_template()?;

        let content = template.render_template(server_info)?;

        tokio::fs::write(&create_options.filename, content).await?;

        tracing::info!(
            "File '{}' was created successfully.",
            create_options.filename.to_string_lossy(),
        );
        tracing::info!(
            "Full path: {}",
            create_options
                .filename
                .canonicalize()
                .map(|f| f.to_string_lossy().into_owned())
                .unwrap_or_else(|_| create_options.filename.to_string_lossy().into_owned())
        );

        Ok(())
    }

    /// Updates an existing file by replacing only templated sections.
    pub async fn update_document(&self, update_options: &WriteOptions) -> DiscoveryResult<()> {
        tracing::trace!("Updating '{}' ", update_options.filename.to_string_lossy());

        let server_info = self
            .server_info
            .as_ref()
            .ok_or(DiscoveryError::NotDiscovered)?;

        update_options.validate()?;

        let template_markers = detect_render_markers(update_options, server_info)?;
        let mut content_lines: Vec<String> = template_markers
            .content
            .lines()
            .map(|s| s.to_owned())
            .collect();

        for location in template_markers.render_locations.iter().rev() {
            let new_lines: Vec<String> = location
                .rendered_template
                .lines()
                .map(|s| s.to_owned())
                .collect();

            content_lines.splice(
                location.render_location.0..location.render_location.1 - 1,
                new_lines,
            );
        }

        let updated_content = content_lines.join(&template_markers.line_ending);

        std::fs::write(&update_options.filename, updated_content)?;
        tracing::info!(
            "File '{}' was updated successfully.",
            update_options.filename.to_string_lossy()
        );
        Ok(())
    }

    /// Print a brief summary of the discovered server information.
    fn print_summary(&self) -> DiscoveryResult<usize> {
        let server_info = self
            .server_info
            .as_ref()
            .ok_or(DiscoveryError::NotDiscovered)?;
        Ok(print_summary(&mut stdout(), server_info)?)
    }

    /// Prints summary and then detailed info about tools, prompts, resources, and templates from server.
    fn print_server_details(&self) -> DiscoveryResult<()> {
        let table_size = self.print_summary()?;

        let server_info = self
            .server_info
            .as_ref()
            .ok_or(DiscoveryError::NotDiscovered)?;

        if let Some(tools) = &server_info.tools {
            if !tools.is_empty() {
                print_header(
                    &mut stdout(),
                    &format!("{}({})", "Tools".bold(), tools.len()),
                    table_size,
                )?;
                let mut tool_list: Vec<_> = tools
                    .iter()
                    .map(|item| {
                        (
                            item.name.clone(),
                            item.description.clone().unwrap_or_default(),
                        )
                    })
                    .collect();
                tool_list.sort_by(|a, b| a.0.cmp(&b.0));
                print_list(stdout(), tool_list)?;
            }
        }

        if let Some(prompts) = &server_info.prompts {
            if !prompts.is_empty() {
                print_header(
                    &mut stdout(),
                    &format!("{}({})", "Prompts".bold(), prompts.len()),
                    table_size,
                )?;
                print_list(
                    stdout(),
                    prompts
                        .iter()
                        .map(|item| {
                            (
                                item.name.clone(),
                                item.description.clone().unwrap_or_default(),
                            )
                        })
                        .collect(),
                )?;
            }
        }

        if let Some(resources) = &server_info.resources {
            if !resources.is_empty() {
                print_header(
                    &mut stdout(),
                    &format!("{}({})", "Resources".bold(), resources.len()),
                    table_size,
                )?;
                print_list(
                    stdout(),
                    resources
                        .iter()
                        .map(|item| {
                            (
                                item.name.clone(),
                                format!(
                                    "{}{}{}",
                                    item.uri,
                                    item.mime_type
                                        .as_ref()
                                        .map_or("".to_string(), |mime_type| format!(
                                            " ({mime_type})"
                                        ))
                                        .dimmed(),
                                    item.description.as_ref().map_or(
                                        "".to_string(),
                                        |description| format!("\n{}", description.dimmed())
                                    )
                                ),
                            )
                        })
                        .collect(),
                )?;
            }
        }

        if let Some(resource_templates) = &server_info.resource_templates {
            if !resource_templates.is_empty() {
                print_header(
                    &mut stdout(),
                    &format!(
                        "{}({})",
                        "Resource Templates".bold(),
                        resource_templates.len()
                    ),
                    table_size,
                )?;
                print_list(
                    stdout(),
                    resource_templates
                        .iter()
                        .map(|item| {
                            (
                                item.name.clone(),
                                format!(
                                    "{}{}{}",
                                    item.uri_template,
                                    item.mime_type
                                        .as_ref()
                                        .map_or("".to_string(), |mime_type| format!(
                                            " ({mime_type})"
                                        ))
                                        .dimmed(),
                                    item.description.as_ref().map_or(
                                        "".to_string(),
                                        |description| format!("\n{}", description.dimmed())
                                    )
                                ),
                            )
                        })
                        .collect(),
                )?;
            }
        }

        Ok(())
    }

    /// Retrieves tools metadata from the MCP server.
    pub async fn tools(
        &self,
        client: Arc<ClientRuntime>,
    ) -> DiscoveryResult<Option<Vec<McpToolMeta>>> {
        if !client.server_has_tools().unwrap_or(false) {
            return Ok(None);
        }

        tracing::trace!("retrieving tools...");

        let tools_result = client
            .request_tool_list(Some(PaginatedRequestParams::default()))
            .await?
            .tools;

        let mut tools: Vec<_> = tools_result
            .iter()
            .map(|tool| {
                let root_schema: serde_json::Value =
                    to_value(&tool.input_schema).unwrap_or_else(|_| Value::Object(Map::new()));
                let params = tool_params(&tool.input_schema.properties, &root_schema);

                Ok::<McpToolMeta, DiscoveryError>(McpToolMeta {
                    name: tool.name.to_owned(),
                    title: tool.title.to_owned(),
                    icons: tool.icons.to_owned(),
                    execution: tool.execution.to_owned(),
                    annotations: tool.annotations.to_owned(),
                    description: tool.description.to_owned(),
                    params,
                    input_schema: tool.input_schema.clone(),
                    meta: tool.meta.to_owned(),
                })
            })
            .filter_map(Result::ok)
            .collect();
        tools.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(Some(tools))
    }

    async fn prompts(&self, client: Arc<ClientRuntime>) -> DiscoveryResult<Option<Vec<Prompt>>> {
        if !client.server_has_prompts().unwrap_or(false) {
            return Ok(None);
        }
        tracing::trace!("retrieving prompts...");

        let prompts: Vec<Prompt> = client
            .request_prompt_list(Some(PaginatedRequestParams::default()))
            .await?
            .prompts;

        Ok(Some(prompts))
    }

    /// Retrieves resources from the server.
    async fn resources(
        &self,
        client: Arc<ClientRuntime>,
    ) -> DiscoveryResult<Option<Vec<Resource>>> {
        if !client.server_has_resources().unwrap_or(false) {
            return Ok(None);
        }

        tracing::trace!("retrieving resources...");

        let resources: Vec<Resource> = client
            .request_resource_list(Some(PaginatedRequestParams::default()))
            .await?
            .resources;

        Ok(Some(resources))
    }

    /// Retrieves resource templates from the server.
    async fn resource_templates(
        &self,
        client: Arc<ClientRuntime>,
    ) -> DiscoveryResult<Option<Vec<ResourceTemplate>>> {
        if !client.server_has_resources().unwrap_or(false) {
            return Ok(None);
        }

        tracing::trace!("retrieving resource templates...");

        let result = client
            .request_resource_template_list(Some(PaginatedRequestParams::default()))
            .await;
        match result {
            Ok(data) => Ok(Some(data.resource_templates)),
            Err(err) => {
                tracing::trace!("Unable to retrieve resource templates : {}", err);
                Ok(None)
            }
        }
    }

    /// Discovers all MCP server capabilities and stores them internally.
    pub async fn discover(&mut self) -> DiscoveryResult<&McpServerInfo> {
        let client = self.try_launch_mcp_server().await?;

        let server_version = client
            .server_version()
            .ok_or(DiscoveryError::ServerNotInitialized)?;

        tracing::trace!(
            "Server: {} v{}",
            server_version.name,
            server_version.version,
        );

        let capabilities: McpCapabilities = McpCapabilities {
            tools: client
                .server_has_tools()
                .ok_or(DiscoveryError::ServerNotInitialized)?,
            prompts: client
                .server_has_prompts()
                .ok_or(DiscoveryError::ServerNotInitialized)?,
            resources: client
                .server_has_resources()
                .ok_or(DiscoveryError::ServerNotInitialized)?,
            logging: client
                .server_supports_logging()
                .ok_or(DiscoveryError::ServerNotInitialized)?,
            completions: client
                .server_info()
                .ok_or(DiscoveryError::ServerNotInitialized)?
                .capabilities
                .completions
                .is_some(),
            experimental: client
                .server_has_experimental()
                .ok_or(DiscoveryError::ServerNotInitialized)?,
            task: McpTaskSupport {
                tool_call_task: client
                    .server_capabilities()
                    .ok_or(DiscoveryError::ServerNotInitialized)?
                    .can_run_task_augmented_tools(),
                list_task: client
                    .server_capabilities()
                    .ok_or(DiscoveryError::ServerNotInitialized)?
                    .can_list_tasks(),
                cancel_task: client
                    .server_capabilities()
                    .ok_or(DiscoveryError::ServerNotInitialized)?
                    .can_cancel_tasks(),
            },
        };

        tracing::trace!("Capabilities: {}", capabilities);

        let tools = self.tools(Arc::clone(&client)).await?;
        let prompts = self.prompts(Arc::clone(&client)).await?;
        let resources = self.resources(Arc::clone(&client)).await?;
        let resource_templates = self.resource_templates(Arc::clone(&client)).await?;

        let server_info = McpServerInfo {
            name: server_version.name,
            version: server_version.version,
            title: server_version.title,
            description: server_version.description,
            website_url: server_version.website_url,
            capabilities,
            tools,
            prompts,
            resources,
            resource_templates,
        };

        self.server_info = Some(server_info);

        Ok(self.server_info.as_ref().unwrap())
    }

    // Attempt server launch with multiple protocol versions when the latest protocol is not supported.
    async fn try_launch_mcp_server(&self) -> SdkResult<Arc<ClientRuntime>> {
        let protocol_versions = [
            ProtocolVersion::V2025_11_25,
            ProtocolVersion::V2025_06_18,
            ProtocolVersion::V2025_03_26,
        ];
        for version in protocol_versions {
            let current_version = format!("with protocol version: {}", version.to_string().bold(),);
            println!("{}", current_version.bright_green());

            match self.launch_mcp_server(version).await {
                Ok(client) => return Ok(client),
                Err(McpSdkError::Protocol { kind: _ }) => {}
                Err(err) => return Err(err),
            }
        }
        Err(McpSdkError::Internal {
            description: "Failed to launch the server.".into(),
        })
    }

    /// Launches the MCP server as a subprocess and initializes the client.
    async fn launch_mcp_server(
        &self,
        protocol_version: ProtocolVersion,
    ) -> SdkResult<Arc<ClientRuntime>> {
        let client_details: InitializeRequestParams = InitializeRequestParams {
            capabilities: ClientCapabilities{
                elicitation: Some(ClientElicitation{ form: Some(Map::new()), url: Some(Map::new()) }),
                experimental: None,
                roots: Some(ClientRoots{ list_changed:Some(true) }),
                sampling: Some(ClientSampling{ context: Some(Map::new()), tools: Some(Map::new()) }),
                tasks: Some(ClientTasks{ cancel: Some(Map::new()), list:Some(Map::new()), requests: Some(rust_mcp_sdk::schema::ClientTaskRequest { elicitation: Some(ClientTaskElicitation { create: Some(Map::new()) }), sampling:Some(ClientTaskSampling { create_message: Some(Map::new()) }) }) })
            },
            client_info: Implementation {
                title: Some("MCP Discovery - By Rust MCP Stack".to_string()),
                name: env!("CARGO_PKG_NAME").to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                description: Some("A lightweight CLI tool built in Rust for discovering and documenting MCP server capabilities.".into()),
                icons: vec![],
                website_url: Some("https://rust-mcp-stack.github.io/mcp-discovery".into())
            },
            protocol_version: protocol_version.into(),
            meta: None
        };

        tracing::trace!(
            "Client : {} v{}",
            client_details.client_info.name,
            client_details.client_info.version
        );

        let (mcp_command, mcp_args) = self.options.mcp_launch_command().split_at(1);

        tracing::trace!(
            "launching command : {} {}",
            mcp_command.first().map(String::as_ref).unwrap_or(""),
            mcp_args.join(" ")
        );

        let transport = StdioTransport::create_with_server_launch(
            mcp_command.first().unwrap(),
            mcp_args.into(),
            None,
            TransportOptions::default(),
        )?;

        let handler = MyClientHandler {};

        let client = client_runtime::create_client(McpClientOptions {
            client_details,
            transport,
            handler: handler.to_mcp_client_handler(),
            task_store: None,
            server_task_store: None,
        });

        tracing::trace!("Launching MCP server ...");

        client.clone().start().await?;

        tracing::trace!("MCP server started successfully.");

        Ok(client)
    }
}
