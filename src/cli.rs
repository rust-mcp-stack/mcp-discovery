use std::{
    io::{self, ErrorKind},
    path::PathBuf,
};

use clap::{arg, command, Parser, Subcommand, ValueEnum};

use crate::{error::DiscoveryResult, utils::match_template, OutputTemplate};

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum Template {
    Md,
    MdPlain,
    Html,
    Txt,
}

#[derive(Subcommand, Debug)]
pub enum DiscoveryCommand {
    /// Displays MCP server capability details in the terminal.
    Print(PrintOptions),
    /// Creates a file with MCP server capability details.
    Create(WriteOptions),
    /// Updates a file by adding MCP server capability information between specified markers.
    Update(WriteOptions),
}

#[derive(Parser, Debug)]
pub struct WriteOptions {
    #[arg(short, long)]
    pub filename: PathBuf,

    /// Select an output template from the built-in options.
    #[arg(short, long, value_enum, conflicts_with_all = ["template_file", "template_string"])]
    pub template: Option<Template>,

    /// Path to a custom template file written in the Handlebars format.
    #[arg(long, short = 'p',
    conflicts_with_all = ["template", "template_string"])]
    pub template_file: Option<PathBuf>,

    /// Template content provided as a string.
    #[arg(
        long,
        short = 's',
        conflicts_with_all = ["template", "template_file"]
    )]
    pub template_string: Option<String>,

    /// Command and arguments to launch the MCP server.
    #[arg(
        value_name = "MCP Launch Command",
        allow_hyphen_values = true,
        last = true,
        required = true
    )]
    pub mcp_server_cmd: Vec<String>,
}

impl WriteOptions {
    pub fn match_template(&self) -> DiscoveryResult<OutputTemplate> {
        match_template(
            Some(&self.filename),
            &self.template,
            &self.template_file,
            &self.template_string,
        )
    }

    pub fn validate(&self) -> DiscoveryResult<()> {
        if !self.filename.exists() {
            return Err(io::Error::new(
                ErrorKind::NotFound,
                format!("File '{}' not found", self.filename.to_string_lossy()),
            )
            .into());
        }
        Ok(())
    }
}

#[derive(Parser, Debug)]
pub struct PrintOptions {
    /// Select an output template from the built-in options.
    #[arg(short, long, value_enum, conflicts_with_all = ["template_file", "template_string"])]
    pub template: Option<Template>,

    /// Path to a custom template file written in the Handlebars format.
    #[arg(long, short = 'p',
conflicts_with_all = ["template", "template_string"])]
    pub template_file: Option<PathBuf>,

    /// Template content provided as a string.
    #[arg(
    long,
    short = 's',
    conflicts_with_all = ["template", "template_file"]
)]
    pub template_string: Option<String>,

    /// Command and arguments to launch the MCP server.
    #[arg(
        value_name = "MCP Launch Command",
        allow_hyphen_values = true,
        last = true,
        required = true
    )]
    pub mcp_server_cmd: Vec<String>,
}

impl PrintOptions {
    pub fn match_template(&self) -> DiscoveryResult<OutputTemplate> {
        match_template(
            None,
            &self.template,
            &self.template_file,
            &self.template_string,
        )
    }
}

impl DiscoveryCommand {
    pub fn mcp_launch_command(&self) -> &Vec<String> {
        match self {
            DiscoveryCommand::Create(create_options) => &create_options.mcp_server_cmd,
            DiscoveryCommand::Update(update_options) => &update_options.mcp_server_cmd,
            DiscoveryCommand::Print(print_args) => &print_args.mcp_server_cmd,
        }
    }
}

#[derive(Parser, Debug)]
#[command(name =  env!("CARGO_PKG_NAME"), arg_required_else_help = true)]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "A command-line tool designed to connect to an MCP Server and explore its capabilities. It offers output options in terminal, JSON, or Markdown formats.", 
long_about = None)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct CommandArguments {
    #[command(subcommand)]
    pub command: Option<DiscoveryCommand>,

    /// Select an output template from the built-in options.
    #[arg(short, long, value_enum, conflicts_with_all = ["template_file", "template_string"])]
    pub template: Option<Template>,

    /// Path to a custom template file written in the Handlebars format.
    #[arg(long, short = 'p',
 conflicts_with_all = ["template", "template_string"])]
    pub template_file: Option<PathBuf>,

    /// Template content provided as a string.
    #[arg(
     long,
     short = 's',
     conflicts_with_all = ["template", "template_file"]
 )]
    pub template_string: Option<String>,

    /// Command and arguments to launch the MCP server.
    #[arg(
        value_name = "MCP Launch Command",
        allow_hyphen_values = true,
        last = true,
        required = true
    )]
    pub mcp_server_cmd: Vec<String>,
}

impl CommandArguments {
    pub fn mcp_launch_command(&self) -> &Vec<String> {
        self.command.as_ref().unwrap().mcp_launch_command()
    }
}
