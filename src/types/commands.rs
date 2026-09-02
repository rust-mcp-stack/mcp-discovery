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

/// Enum representing the main actions that can be performed for MCP discovery.
#[derive(Debug)]
pub enum DiscoveryCommand {
    /// Displays MCP server capability details in the terminal.
    Print(PrintOptions),
    /// Creates a file with MCP server capability details.
    Create(WriteOptions),
    /// Updates a file by adding MCP server capability information between specified markers.
    Update(WriteOptions),
}

/// Enum defining the types of built-in templates supported for output formatting.
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

/// OAuth grant type used to authenticate against a protected MCP server.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Grant {
    /// Machine-to-machine authentication (RFC 6749 §4.4).
    #[default]
    ClientCredentials,
    /// Interactive user authorization code flow with PKCE (RFC 7636).
    AuthorizationCode,
}

/// Authentication options for connecting to a protected streamable HTTP MCP server.
#[derive(Debug, Clone, Default)]
pub struct McpAuthOptions {
    /// Static headers (e.g. `Authorization: Bearer <token>`), applied to every request.
    pub headers: Vec<String>,
    /// Pre-registered OAuth client id. When omitted, dynamic client registration is used.
    pub client_id: Option<String>,
    /// Pre-registered OAuth client secret.
    pub client_secret: Option<String>,
    /// OAuth scope(s) to request.
    pub scope: Option<String>,
    /// Redirect URI used by the authorization-code flow.
    pub redirect_uri: Option<String>,
    /// Which OAuth grant to use.
    pub grant: Grant,
}

impl McpAuthOptions {
    /// Returns `true` if any authentication option was configured.
    pub fn is_configured(&self) -> bool {
        !self.headers.is_empty()
            || self.client_id.is_some()
            || self.client_secret.is_some()
            || self.scope.is_some()
            || self.redirect_uri.is_some()
            || self.grant != Grant::ClientCredentials
    }
}

/// Enum representing supported log levels for controlling output verbosity.
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

/// Options used when running the `Print` variant of `DiscoveryCommand`.
#[derive(Debug)]
pub struct PrintOptions {
    /// Select an output template from the built-in options.
    pub template: Option<Template>,

    /// Path to a custom template file written in the Handlebars format.
    pub template_file: Option<PathBuf>,

    /// Template content provided as a string.
    pub template_string: Option<String>,

    /// URL of a remote Handlebars template (`.hbs`, `.zip`, or `.tar.gz`) to fetch and render.
    pub template_url: Option<String>,

    /// Cache directory used for fetched remote templates (defaults to the OS cache dir).
    pub cache_dir: Option<PathBuf>,

    /// Specifies the logging level for the application (default: info)
    pub log_level: Option<LogLevel>,

    /// Command and arguments to launch the MCP server.
    pub mcp_server_cmd: Vec<String>,

    /// URL of a streamable HTTP MCP server (mutually exclusive with the launch command).
    pub url: Option<String>,

    /// Authentication options for protected streamable HTTP servers.
    pub auth: McpAuthOptions,
}

impl PrintOptions {
    /// Resolves the output template (built-in, file, string, or remote URL) based on user input.
    pub fn match_template(&self) -> DiscoveryResult<OutputTemplate> {
        match_template(
            None,
            &self.template,
            &self.template_file,
            &self.template_string,
            &self.template_url,
            &self.cache_dir,
        )
    }
}

/// Options used when running the `Create` or `Update` variants of `DiscoveryCommand`.
#[derive(Debug)]
pub struct WriteOptions {
    pub filename: PathBuf,

    /// Select an output template from the built-in options.
    pub template: Option<Template>,

    /// Path to a custom template file written in the Handlebars format.
    pub template_file: Option<PathBuf>,

    /// Template content provided as a string.
    pub template_string: Option<String>,

    /// URL of a remote Handlebars template (`.hbs`, `.zip`, or `.tar.gz`) to fetch and render.
    pub template_url: Option<String>,

    /// Cache directory used for fetched remote templates (defaults to the OS cache dir).
    pub cache_dir: Option<PathBuf>,

    /// Specifies the logging level for the application (default: info)
    pub log_level: Option<LogLevel>,
    /// Command and arguments to launch the MCP server.
    pub mcp_server_cmd: Vec<String>,

    /// URL of a streamable HTTP MCP server (mutually exclusive with the launch command).
    pub url: Option<String>,

    /// Authentication options for protected streamable HTTP servers.
    pub auth: McpAuthOptions,
}

impl WriteOptions {
    /// Resolves the output template (built-in, file, string, or remote URL) based on user input.
    pub fn match_template(&self) -> DiscoveryResult<OutputTemplate> {
        match_template(
            Some(&self.filename),
            &self.template,
            &self.template_file,
            &self.template_string,
            &self.template_url,
            &self.cache_dir,
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
    /// Retrieves the MCP server launch command for the current variant.
    pub fn mcp_launch_command(&self) -> &Vec<String> {
        match self {
            DiscoveryCommand::Create(create_options) => &create_options.mcp_server_cmd,
            DiscoveryCommand::Update(update_options) => &update_options.mcp_server_cmd,
            DiscoveryCommand::Print(print_args) => &print_args.mcp_server_cmd,
        }
    }

    /// Retrieves the streamable HTTP MCP server URL for the current variant, if set.
    pub fn mcp_url(&self) -> Option<&String> {
        match self {
            DiscoveryCommand::Create(create_options) => create_options.url.as_ref(),
            DiscoveryCommand::Update(update_options) => update_options.url.as_ref(),
            DiscoveryCommand::Print(print_args) => print_args.url.as_ref(),
        }
    }

    /// Retrieves the authentication options for the current variant.
    pub fn mcp_auth(&self) -> &McpAuthOptions {
        match self {
            DiscoveryCommand::Create(create_options) => &create_options.auth,
            DiscoveryCommand::Update(update_options) => &update_options.auth,
            DiscoveryCommand::Print(print_args) => &print_args.auth,
        }
    }

    /// Retrieves the configured log level for the current variant.
    pub fn log_level(&self) -> &Option<LogLevel> {
        match self {
            DiscoveryCommand::Create(create_options) => &create_options.log_level,
            DiscoveryCommand::Update(update_options) => &update_options.log_level,
            DiscoveryCommand::Print(print_args) => &print_args.log_level,
        }
    }
}
