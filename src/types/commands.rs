use std::{
    fmt::Display,
    io::{self, ErrorKind},
    path::PathBuf,
    str::FromStr,
};

use crate::{
    error::{DiscoveryError, DiscoveryResult},
    utils::match_template,
    OutputTemplate,
};

#[derive(Debug)]
pub enum DiscoveryCommand {
    /// Displays MCP server capability details in the terminal.
    Print(PrintOptions),
    /// Creates a file with MCP server capability details.
    Create(WriteOptions),
    /// Updates a file by adding MCP server capability information between specified markers.
    Update(WriteOptions),
}
#[derive(Debug, Clone, PartialEq)]
pub enum Template {
    Md,
    MdPlain,
    Html,
    Txt,
}

impl FromStr for Template {
    type Err = DiscoveryError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "md" => Ok(Template::Md),
            "md-plain" => Ok(Template::MdPlain),
            "html" => Ok(Template::Html),
            "txt" => Ok(Template::Txt),
            _ => Err(DiscoveryError::InvalidTemplate(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum LogLevel {
    error,
    warn,
    info,
    debug,
    trace,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::error => write!(f, "error"),
            LogLevel::warn => write!(f, "warn"),
            LogLevel::info => write!(f, "info"),
            LogLevel::debug => write!(f, "debug"),
            LogLevel::trace => write!(f, "trace"),
        }
    }
}

#[derive(Debug)]
pub struct PrintOptions {
    /// Select an output template from the built-in options.
    pub template: Option<Template>,

    /// Path to a custom template file written in the Handlebars format.
    pub template_file: Option<PathBuf>,

    /// Template content provided as a string.
    pub template_string: Option<String>,

    /// Specifies the logging level for the application (default: info)
    pub log_level: Option<LogLevel>,

    /// Command and arguments to launch the MCP server.
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

#[derive(Debug)]
pub struct WriteOptions {
    pub filename: PathBuf,

    /// Select an output template from the built-in options.
    pub template: Option<Template>,

    /// Path to a custom template file written in the Handlebars format.
    pub template_file: Option<PathBuf>,

    /// Template content provided as a string.
    pub template_string: Option<String>,

    /// Specifies the logging level for the application (default: info)
    pub log_level: Option<LogLevel>,
    /// Command and arguments to launch the MCP server.
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

impl DiscoveryCommand {
    pub fn mcp_launch_command(&self) -> &Vec<String> {
        match self {
            DiscoveryCommand::Create(create_options) => &create_options.mcp_server_cmd,
            DiscoveryCommand::Update(update_options) => &update_options.mcp_server_cmd,
            DiscoveryCommand::Print(print_args) => &print_args.mcp_server_cmd,
        }
    }

    pub fn log_level(&self) -> &Option<LogLevel> {
        match self {
            DiscoveryCommand::Create(create_options) => &create_options.log_level,
            DiscoveryCommand::Update(update_options) => &update_options.log_level,
            DiscoveryCommand::Print(print_args) => &print_args.log_level,
        }
    }
}
