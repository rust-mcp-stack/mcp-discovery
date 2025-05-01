pub mod error;
mod handler;
pub mod render_template;
pub mod schema;
pub mod std_output;
pub mod templates;
pub mod types;
pub mod utils;

use colored::Colorize;
use error::{DiscoveryError, DiscoveryResult};
use render_template::{detect_render_markers, render_template};
use schema::tool_params;
use std::io::stdout;
use std_output::{print_header, print_list, print_summary};
pub use templates::OutputTemplate;
use types::{
    DiscoveryCommand, McpCapabilities, McpServerInfo, McpToolMeta, PrintOptions, WriteOptions,
};

use std::sync::Arc;

use handler::MyClientHandler;
use rust_mcp_schema::{
    ClientCapabilities, Implementation, InitializeRequestParams, ListPromptsRequestParams,
    ListResourceTemplatesRequestParams, ListResourcesRequestParams, ListToolsRequestParams, Prompt,
    Resource, ResourceTemplate, JSONRPC_VERSION,
};
use rust_mcp_sdk::{
    error::SdkResult,
    mcp_client::{client_runtime, ClientRuntime},
    McpClient, StdioTransport, TransportOptions,
};

pub struct McpDiscovery {
    options: DiscoveryCommand,
    pub server_info: Option<McpServerInfo>,
}

impl McpDiscovery {
    pub fn new(options: DiscoveryCommand) -> Self {
        Self {
            options,
            server_info: None,
        }
    }

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
                self.print_mcp_capabilities(print_options).await?;
            }
        };
        Ok(())
    }

    pub async fn print_mcp_capabilities(
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
                println!("{}", content);
            }
        }

        Ok(())
    }

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

    pub fn print_summary(&self) -> DiscoveryResult<usize> {
        let server_info = self
            .server_info
            .as_ref()
            .ok_or(DiscoveryError::NotDiscovered)?;
        Ok(print_summary(stdout(), server_info)?)
    }

    pub fn render_with_template(&self, template: OutputTemplate) -> DiscoveryResult<()> {
        let server_info = self
            .server_info
            .as_ref()
            .ok_or(DiscoveryError::NotDiscovered)?;

        let content = template.render_template(server_info)?;

        println!("{}", content);
        Ok(())
    }

    pub fn print_server_details(&self) -> DiscoveryResult<()> {
        let table_size = self.print_summary()?;

        let server_info = self
            .server_info
            .as_ref()
            .ok_or(DiscoveryError::NotDiscovered)?;

        if let Some(tools) = &server_info.tools {
            print_header(
                stdout(),
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

        if let Some(prompts) = &server_info.prompts {
            print_header(
                stdout(),
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

        if let Some(resources) = &server_info.resources {
            print_header(
                stdout(),
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
                                    .map_or("".to_string(), |mime_type| format!(" ({})", mime_type))
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

        if let Some(resource_templates) = &server_info.resource_templates {
            print_header(
                stdout(),
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
                                    .map_or("".to_string(), |mime_type| format!(" ({})", mime_type))
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

        Ok(())
    }

    pub async fn tools(
        &self,
        client: Arc<ClientRuntime>,
    ) -> DiscoveryResult<Option<Vec<McpToolMeta>>> {
        if !client.server_has_tools().unwrap_or(false) {
            return Ok(None);
        }

        tracing::trace!("retrieving tools...");

        let tools_result = client
            .list_tools(Some(ListToolsRequestParams::default()))
            .await?
            .tools;

        let mut tools: Vec<_> = tools_result
            .iter()
            .map(|tool| {
                let params = tool_params(&tool.input_schema.properties);

                Ok::<McpToolMeta, DiscoveryError>(McpToolMeta {
                    name: tool.name.to_owned(),
                    description: tool.description.to_owned(),
                    params,
                })
            })
            .filter_map(Result::ok)
            .collect();
        tools.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(Some(tools))
    }

    async fn get_prompts(
        &self,
        client: Arc<ClientRuntime>,
    ) -> DiscoveryResult<Option<Vec<Prompt>>> {
        if !client.server_has_prompts().unwrap_or(false) {
            return Ok(None);
        }
        tracing::trace!("retrieving prompts...");

        let prompts: Vec<Prompt> = client
            .list_prompts(Some(ListPromptsRequestParams::default()))
            .await?
            .prompts;

        Ok(Some(prompts))
    }

    async fn get_resources(
        &self,
        client: Arc<ClientRuntime>,
    ) -> DiscoveryResult<Option<Vec<Resource>>> {
        if !client.server_has_resources().unwrap_or(false) {
            return Ok(None);
        }

        tracing::trace!("retrieving resources...");

        let resources: Vec<Resource> = client
            .list_resources(Some(ListResourcesRequestParams::default()))
            .await?
            .resources;

        Ok(Some(resources))
    }

    async fn get_resource_templates(
        &self,
        client: Arc<ClientRuntime>,
    ) -> DiscoveryResult<Option<Vec<ResourceTemplate>>> {
        if !client.server_has_resources().unwrap_or(false) {
            return Ok(None);
        }

        tracing::trace!("retrieving resource templates...");

        let result = client
            .list_resource_templates(Some(ListResourceTemplatesRequestParams::default()))
            .await;
        match result {
            Ok(data) => Ok(Some(data.resource_templates)),
            Err(err) => {
                tracing::trace!("Unable to retrieve resource templates : {}", err);
                Ok(None)
            }
        }
    }

    pub async fn discover(&mut self) -> DiscoveryResult<&McpServerInfo> {
        let client = self.launch_mcp_server().await?;

        let server_version = client
            .server_version()
            .ok_or(DiscoveryError::ServerNotInitialized)?;

        tracing::trace!(
            "Server: {} v{}",
            server_version.name,
            server_version.version
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
            experimental: client
                .server_has_experimental()
                .ok_or(DiscoveryError::ServerNotInitialized)?,
        };

        tracing::trace!("Capabilities: {}", capabilities);

        let tools = self.tools(Arc::clone(&client)).await?;
        let prompts = self.get_prompts(Arc::clone(&client)).await?;
        let resources = self.get_resources(Arc::clone(&client)).await?;
        let resource_templates = self.get_resource_templates(Arc::clone(&client)).await?;

        let server_info = McpServerInfo {
            name: server_version.name,
            version: server_version.version,
            capabilities,
            tools,
            prompts,
            resources,
            resource_templates,
        };

        self.server_info = Some(server_info);

        Ok(self.server_info.as_ref().unwrap())
    }

    pub async fn launch_mcp_server(&self) -> SdkResult<Arc<ClientRuntime>> {
        let client_details: InitializeRequestParams = InitializeRequestParams {
            capabilities: ClientCapabilities::default(),
            client_info: Implementation {
                name: env!("CARGO_PKG_NAME").to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            protocol_version: JSONRPC_VERSION.into(),
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

        let client = client_runtime::create_client(client_details, transport, handler);

        tracing::trace!("Launching MCP server ...");

        client.clone().start().await?;

        tracing::trace!("MCP server started successfully.");

        Ok(client)
    }
}
