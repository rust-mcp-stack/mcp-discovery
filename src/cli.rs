use clap::{Parser, Subcommand, ValueEnum};
use mcp_discovery::{
    DiscoveryCommand, Grant, LogLevel, McpAuthOptions, PrintOptions, Template, WriteOptions,
};
use std::path::PathBuf;

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum CliTemplate {
    Md,
    MdPlain,
    Html,
    Txt,
}

impl From<CliTemplate> for Template {
    fn from(value: CliTemplate) -> Self {
        match value {
            CliTemplate::Md => Self::Md,
            CliTemplate::MdPlain => Self::MdPlain,
            CliTemplate::Html => Self::Html,
            CliTemplate::Txt => Self::Txt,
        }
    }
}

#[derive(Debug, Clone, ValueEnum, PartialEq)]
#[allow(non_camel_case_types)]
pub enum CliLogLevel {
    error,
    warn,
    info,
    debug,
    trace,
}

impl From<CliLogLevel> for LogLevel {
    fn from(value: CliLogLevel) -> Self {
        match value {
            CliLogLevel::error => Self::error,
            CliLogLevel::warn => Self::warn,
            CliLogLevel::info => Self::info,
            CliLogLevel::debug => Self::debug,
            CliLogLevel::trace => Self::trace,
        }
    }
}

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum CliGrant {
    ClientCredentials,
    AuthorizationCode,
}

impl From<CliGrant> for Grant {
    fn from(value: CliGrant) -> Self {
        match value {
            CliGrant::ClientCredentials => Self::ClientCredentials,
            CliGrant::AuthorizationCode => Self::AuthorizationCode,
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum CliDiscoveryCommand {
    /// Displays MCP server capability details in the terminal.
    Print(CliPrintOptions),
    /// Creates a file with MCP server capability details.
    Create(CliWriteOptions),
    /// Updates a file by adding MCP server capability information between specified markers.
    Update(CliWriteOptions),
}

#[derive(Parser, Debug)]
pub struct CliWriteOptions {
    #[arg(short, long)]
    pub filename: PathBuf,

    /// Select an output template from the built-in options.
    #[arg(short, long, value_enum, conflicts_with_all = ["template_file", "template_string", "template_url"])]
    pub template: Option<CliTemplate>,

    /// Path to a custom template file written in the Handlebars format.
    #[arg(long, short = 'p',
    conflicts_with_all = ["template", "template_string", "template_url"])]
    pub template_file: Option<PathBuf>,

    /// Template content provided as a string.
    #[arg(
        long,
        short = 's',
        conflicts_with_all = ["template", "template_file", "template_url"]
    )]
    pub template_string: Option<String>,

    /// Fetch a remote Handlebars template ('.hbs', '.zip', or '.tar.gz') from an https:// URL.
    /// Supports fragment directives: '#sha256=<hex>' and '#entry=<subpath>'.
    #[arg(
        long,
        conflicts_with_all = ["template", "template_file", "template_string"]
    )]
    pub template_url: Option<String>,

    /// Cache directory for templates fetched with --template-url (defaults to the OS cache dir).
    #[arg(long, value_name = "PATH")]
    pub cache_dir: Option<PathBuf>,

    /// Specifies the logging level for the application (default: info)
    #[arg(long, short)]
    pub log_level: Option<CliLogLevel>,

    /// URL of a streamable HTTP MCP server. Mutually exclusive with the launch command.
    #[arg(long, conflicts_with = "mcp_server_cmd")]
    pub url: Option<String>,

    /// Static header to send with streamable HTTP requests (repeatable, "Name: Value").
    #[arg(long, value_name = "NAME:VALUE", requires = "url")]
    pub header: Vec<String>,

    /// Pre-registered OAuth client id (omit to use dynamic client registration).
    #[arg(long, requires = "url")]
    pub client_id: Option<String>,

    /// Pre-registered OAuth client secret.
    #[arg(long, requires = "url")]
    pub client_secret: Option<String>,

    /// OAuth scope(s) to request.
    #[arg(long, requires = "url")]
    pub scope: Option<String>,

    /// Redirect URI used by the authorization-code flow.
    #[arg(long, requires = "url")]
    pub redirect_uri: Option<String>,

    /// OAuth grant type (default: client-credentials).
    #[arg(long, value_enum, requires = "url")]
    pub grant: Option<CliGrant>,

    /// Command and arguments to launch the MCP server.
    #[arg(
        value_name = "MCP Launch Command",
        allow_hyphen_values = true,
        last = true,
        required_unless_present = "url"
    )]
    pub mcp_server_cmd: Vec<String>,
}

impl From<CliWriteOptions> for WriteOptions {
    fn from(value: CliWriteOptions) -> Self {
        Self {
            filename: value.filename,
            template: value.template.map(|t| t.into()),
            template_file: value.template_file,
            template_string: value.template_string,
            template_url: value.template_url,
            cache_dir: value.cache_dir,
            log_level: value.log_level.map(|l| l.into()),
            mcp_server_cmd: value.mcp_server_cmd,
            url: value.url,
            auth: McpAuthOptions {
                headers: value.header,
                client_id: value.client_id,
                client_secret: value.client_secret,
                scope: value.scope,
                redirect_uri: value.redirect_uri,
                grant: value.grant.map(Into::into).unwrap_or_default(),
            },
        }
    }
}

#[derive(Parser, Debug)]
pub struct CliPrintOptions {
    /// Select an output template from the built-in options.
    #[arg(short, long, value_enum, conflicts_with_all = ["template_file", "template_string", "template_url"])]
    pub template: Option<CliTemplate>,

    /// Path to a custom template file written in the Handlebars format.
    #[arg(long, short = 'p',
conflicts_with_all = ["template", "template_string", "template_url"])]
    pub template_file: Option<PathBuf>,

    /// Template content provided as a string.
    #[arg(
    long,
    short = 's',
    conflicts_with_all = ["template", "template_file", "template_url"]
)]
    pub template_string: Option<String>,

    /// Fetch a remote Handlebars template ('.hbs', '.zip', or '.tar.gz') from an https:// URL.
    /// Supports fragment directives: '#sha256=<hex>' and '#entry=<subpath>'.
    #[arg(
     long,
     conflicts_with_all = ["template", "template_file", "template_string"]
 )]
    pub template_url: Option<String>,

    /// Cache directory for templates fetched with --template-url (defaults to the OS cache dir).
    #[arg(long, value_name = "PATH")]
    pub cache_dir: Option<PathBuf>,

    /// Specifies the logging level for the application (default: info)
    #[arg(long, short)]
    pub log_level: Option<CliLogLevel>,

    /// URL of a streamable HTTP MCP server. Mutually exclusive with the launch command.
    #[arg(long, conflicts_with = "mcp_server_cmd")]
    pub url: Option<String>,

    /// Static header to send with streamable HTTP requests (repeatable, "Name: Value").
    #[arg(long, value_name = "NAME:VALUE", requires = "url")]
    pub header: Vec<String>,

    /// Pre-registered OAuth client id (omit to use dynamic client registration).
    #[arg(long, requires = "url")]
    pub client_id: Option<String>,

    /// Pre-registered OAuth client secret.
    #[arg(long, requires = "url")]
    pub client_secret: Option<String>,

    /// OAuth scope(s) to request.
    #[arg(long, requires = "url")]
    pub scope: Option<String>,

    /// Redirect URI used by the authorization-code flow.
    #[arg(long, requires = "url")]
    pub redirect_uri: Option<String>,

    /// OAuth grant type (default: client-credentials).
    #[arg(long, value_enum, requires = "url")]
    pub grant: Option<CliGrant>,

    /// Command and arguments to launch the MCP server.
    #[arg(
        value_name = "MCP Launch Command",
        allow_hyphen_values = true,
        last = true,
        required_unless_present = "url"
    )]
    pub mcp_server_cmd: Vec<String>,
}

impl From<CliPrintOptions> for PrintOptions {
    fn from(value: CliPrintOptions) -> Self {
        Self {
            template: value.template.map(|t| t.into()),
            template_file: value.template_file,
            template_string: value.template_string,
            template_url: value.template_url,
            cache_dir: value.cache_dir,
            log_level: value.log_level.map(|l| l.into()),
            mcp_server_cmd: value.mcp_server_cmd,
            url: value.url,
            auth: McpAuthOptions {
                headers: value.header,
                client_id: value.client_id,
                client_secret: value.client_secret,
                scope: value.scope,
                redirect_uri: value.redirect_uri,
                grant: value.grant.map(Into::into).unwrap_or_default(),
            },
        }
    }
}

impl From<CliDiscoveryCommand> for DiscoveryCommand {
    fn from(value: CliDiscoveryCommand) -> Self {
        match value {
            CliDiscoveryCommand::Print(cli_print_options) => Self::Print(cli_print_options.into()),
            CliDiscoveryCommand::Create(cli_write_options) => {
                Self::Create(cli_write_options.into())
            }
            CliDiscoveryCommand::Update(cli_write_options) => {
                Self::Update(cli_write_options.into())
            }
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
    pub command: Option<CliDiscoveryCommand>,

    /// Select an output template from the built-in options.
    #[arg(short, long, value_enum, conflicts_with_all = ["template_file", "template_string", "template_url"])]
    pub template: Option<CliTemplate>,

    /// Path to a custom template file written in the Handlebars format.
    #[arg(long, short = 'p',
 conflicts_with_all = ["template", "template_string", "template_url"])]
    pub template_file: Option<PathBuf>,

    /// Template content provided as a string.
    #[arg(
     long,
     short = 's',
     conflicts_with_all = ["template", "template_file", "template_url"]
 )]
    pub template_string: Option<String>,

    /// Fetch a remote Handlebars template ('.hbs', '.zip', or '.tar.gz') from an https:// URL.
    /// Supports fragment directives: '#sha256=<hex>' and '#entry=<subpath>'.
    #[arg(
     long,
     conflicts_with_all = ["template", "template_file", "template_string"]
 )]
    pub template_url: Option<String>,

    /// Cache directory for templates fetched with --template-url (defaults to the OS cache dir).
    #[arg(long, value_name = "PATH")]
    pub cache_dir: Option<PathBuf>,

    /// Specifies the logging level for the application (default: info)
    #[arg(long, short)]
    pub log_level: Option<CliLogLevel>,

    /// URL of a streamable HTTP MCP server. Mutually exclusive with the launch command.
    #[arg(long, conflicts_with = "mcp_server_cmd")]
    pub url: Option<String>,

    /// Static header to send with streamable HTTP requests (repeatable, "Name: Value").
    #[arg(long, value_name = "NAME:VALUE", requires = "url")]
    pub header: Vec<String>,

    /// Pre-registered OAuth client id (omit to use dynamic client registration).
    #[arg(long, requires = "url")]
    pub client_id: Option<String>,

    /// Pre-registered OAuth client secret.
    #[arg(long, requires = "url")]
    pub client_secret: Option<String>,

    /// OAuth scope(s) to request.
    #[arg(long, requires = "url")]
    pub scope: Option<String>,

    /// Redirect URI used by the authorization-code flow.
    #[arg(long, requires = "url")]
    pub redirect_uri: Option<String>,

    /// OAuth grant type (default: client-credentials).
    #[arg(long, value_enum, requires = "url")]
    pub grant: Option<CliGrant>,

    /// Command and arguments to launch the MCP server.
    #[arg(
        value_name = "MCP Launch Command",
        allow_hyphen_values = true,
        last = true,
        required_unless_present = "url"
    )]
    pub mcp_server_cmd: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to parse arguments from a vector of strings
    pub fn parse_args(args: Vec<&str>) -> CommandArguments {
        CommandArguments::parse_from(args)
    }

    #[test]
    fn test_version_flag() {
        let args = vec!["mcp-tool", "--version"];
        let result = CommandArguments::try_parse_from(args);
        assert!(result.is_err(), "Expected clap to handle --version flag");
        // Note: clap automatically handles --version and exits, so this test verifies it doesn't parse normally
    }

    #[test]
    fn test_conflicting_template_and_template_file() {
        let args = vec![
            "mcp-tool",
            "create",
            "--filename",
            "output.md",
            "--template",
            "md",
            "--template-file",
            "custom.hbs",
            "--",
            "mcp-server",
        ];
        let result = CommandArguments::try_parse_from(args);
        assert!(
            result.is_err(),
            "Expected error due to conflicting template options"
        );
    }

    #[test]
    fn test_missing_required_mcp_server_cmd() {
        let args = vec!["mcp-tool", "print"];
        let result = CommandArguments::try_parse_from(args);
        assert!(
            result.is_err(),
            "Expected error due to missing mcp_server_cmd"
        );
    }

    #[test]
    fn test_file_options_match_template_custom() {
        let file_options = WriteOptions {
            filename: PathBuf::from("output.html"),
            template: None,
            template_file: Some(PathBuf::from("templates/markdown/markdown_template.md")),
            mcp_server_cmd: vec!["mcp-server".to_string()],
            template_string: None,
            template_url: None,
            cache_dir: None,
            log_level: None,
            url: None,
            auth: Default::default(),
        };

        let result = file_options.match_template();

        assert!(
            result.is_ok(),
            "Expected successful template matching with custom file"
        );
    }

    #[test]
    fn test_create_command_parsing() {
        let args = vec![
            "mcp-tool",
            "create",
            "--filename",
            "output.md",
            "--template",
            "md",
            "--",
            "mcp-server",
            "--some-params",
            "some-values",
        ];
        let parsed = parse_args(args);

        match parsed.command {
            Some(CliDiscoveryCommand::Create(file_options)) => {
                assert_eq!(file_options.filename, PathBuf::from("output.md"));
                assert_eq!(file_options.template, Some(CliTemplate::Md));
                assert_eq!(file_options.template_file, None);
                assert_eq!(
                    file_options.mcp_server_cmd,
                    vec!["mcp-server", "--some-params", "some-values"]
                );
            }
            _ => panic!("Expected Create command"),
        }
    }

    #[test]
    fn test_update_command_with_template_file() {
        let args = vec![
            "mcp-tool",
            "update",
            "--filename",
            "output.html",
            "--template-file",
            "custom.hbs",
            "--",
            "mcp-server",
            "--param",
            "90",
        ];
        let parsed = parse_args(args);

        match parsed.command {
            Some(CliDiscoveryCommand::Update(file_options)) => {
                assert_eq!(file_options.filename, PathBuf::from("output.html"));
                assert_eq!(file_options.template, None);
                assert_eq!(
                    file_options.template_file,
                    Some(PathBuf::from("custom.hbs"))
                );
                assert_eq!(
                    file_options.mcp_server_cmd,
                    vec!["mcp-server", "--param", "90"]
                );
            }
            _ => panic!("Expected Update command"),
        }
    }

    #[test]
    fn test_print_command_with_json() {
        let args = vec!["mcp-tool", "print", "--", "mcp-server", "--verbose"];
        let parsed = parse_args(args);

        match parsed.command {
            Some(CliDiscoveryCommand::Print(print_options)) => {
                assert_eq!(
                    print_options.mcp_server_cmd,
                    vec!["mcp-server", "--verbose"]
                );
            }
            _ => panic!("Expected Print command"),
        }
    }

    #[test]
    fn test_mcp_launch_command_retrieval() {
        let args = vec![
            "mcp-tool",
            "create",
            "--filename",
            "output.txt",
            "--template",
            "txt",
            "--",
            "mcp-server",
            "--port",
            "9090",
        ];
        let command: DiscoveryCommand = parse_args(args).command.unwrap().into();

        let launch_cmd = command.mcp_launch_command();
        assert_eq!(launch_cmd, &vec!["mcp-server", "--port", "9090"]);
        assert_eq!(command.mcp_url(), None);
    }

    #[test]
    fn test_url_command_parsing() {
        let args = vec!["mcp-tool", "print", "--url", "http://localhost:8080/mcp"];
        let command: DiscoveryCommand = parse_args(args).command.unwrap().into();

        assert_eq!(
            command.mcp_url(),
            Some(&"http://localhost:8080/mcp".to_string())
        );
        assert!(command.mcp_launch_command().is_empty());
    }

    #[test]
    fn test_url_conflicts_with_launch_command() {
        let args = vec![
            "mcp-tool",
            "print",
            "--url",
            "http://localhost:8080/mcp",
            "--",
            "mcp-server",
        ];
        let result = CommandArguments::try_parse_from(args);
        assert!(
            result.is_err(),
            "Expected error due to conflicting --url and launch command"
        );
    }

    #[test]
    fn test_template_url_and_cache_dir_parsing() {
        let args = vec![
            "mcp-tool",
            "print",
            "--template-url",
            "https://example.com/t.hbs#sha256=abab",
            "--cache-dir",
            "/tmp/mcp-cache",
            "--",
            "mcp-server",
        ];
        let command: DiscoveryCommand = parse_args(args).command.unwrap().into();
        match command {
            DiscoveryCommand::Print(opts) => {
                assert_eq!(
                    opts.template_url.as_deref(),
                    Some("https://example.com/t.hbs#sha256=abab")
                );
                assert_eq!(
                    opts.cache_dir.as_deref(),
                    Some(PathBuf::from("/tmp/mcp-cache").as_path())
                );
            }
            _ => panic!("expected Print command"),
        }
    }

    #[test]
    fn test_template_url_coexists_with_transport_url() {
        let args = vec![
            "mcp-tool",
            "print",
            "--url",
            "https://mcp.example.com/mcp",
            "--template-url",
            "https://example.com/t.hbs",
            "--cache-dir",
            "/tmp/mcp-cache",
        ];
        let command: DiscoveryCommand = parse_args(args).command.unwrap().into();
        match command {
            DiscoveryCommand::Print(opts) => {
                assert!(
                    opts.template_url.is_some(),
                    "expected --template-url parsed"
                );
                assert!(opts.url.is_some(), "expected transport --url parsed");
            }
            _ => panic!("expected Print command"),
        }
    }

    #[test]
    fn test_template_url_conflicts_with_other_template_flags() {
        for args in [
            vec![
                "mcp-tool",
                "print",
                "--template",
                "md",
                "--template-url",
                "https://example.com/t.hbs",
                "--",
                "srv",
            ],
            vec![
                "mcp-tool",
                "print",
                "--template-file",
                "a.hbs",
                "--template-url",
                "https://example.com/t.hbs",
                "--",
                "srv",
            ],
            vec![
                "mcp-tool",
                "print",
                "--template-string",
                "hi",
                "--template-url",
                "https://example.com/t.hbs",
                "--",
                "srv",
            ],
        ] {
            let result = CommandArguments::try_parse_from(args);
            assert!(result.is_err(), "Expected conflict between template flags");
        }
    }

    #[test]
    fn test_file_options_match_template_builtin() {
        let file_options = WriteOptions {
            filename: PathBuf::from("output.md"),
            template: Some(Template::Md),
            template_file: None,
            mcp_server_cmd: vec!["mcp-server".to_string()],
            template_string: None,
            template_url: None,
            cache_dir: None,
            log_level: None,
            url: None,
            auth: Default::default(),
        };

        let result = file_options.match_template();
        assert!(result.is_ok(), "Expected successful template matching");
        // Note: Cannot assert specific OutputTemplate without knowing its structure
    }

    #[test]
    fn test_oauth_flags_parsing() {
        let args = vec![
            "mcp-tool",
            "print",
            "--url",
            "https://mcp.example.com/mcp",
            "--header",
            "Authorization: Bearer abc",
            "--header",
            "X-Api-Key: 123",
            "--client-id",
            "my-client",
            "--client-secret",
            "my-secret",
            "--scope",
            "mcp tools",
            "--grant",
            "client-credentials",
        ];
        let command: DiscoveryCommand = parse_args(args).command.unwrap().into();

        let auth = command.mcp_auth();
        assert_eq!(
            auth.headers,
            vec![
                "Authorization: Bearer abc".to_string(),
                "X-Api-Key: 123".to_string()
            ]
        );
        assert_eq!(auth.client_id.as_deref(), Some("my-client"));
        assert_eq!(auth.client_secret.as_deref(), Some("my-secret"));
        assert_eq!(auth.scope.as_deref(), Some("mcp tools"));
        assert_eq!(auth.grant, Grant::ClientCredentials);
    }

    #[test]
    fn test_authorization_code_grant() {
        let args = vec![
            "mcp-tool",
            "print",
            "--url",
            "https://mcp.example.com/mcp",
            "--grant",
            "authorization-code",
            "--redirect-uri",
            "http://127.0.0.1:8080/callback",
        ];
        let command: DiscoveryCommand = parse_args(args).command.unwrap().into();

        let auth = command.mcp_auth();
        assert_eq!(auth.grant, Grant::AuthorizationCode);
        assert_eq!(
            auth.redirect_uri.as_deref(),
            Some("http://127.0.0.1:8080/callback")
        );
    }

    #[test]
    fn test_oauth_flags_require_url() {
        let args = vec!["mcp-tool", "print", "--client-id", "my-client"];
        let result = CommandArguments::try_parse_from(args);
        assert!(
            result.is_err(),
            "Expected error: --client-id requires --url"
        );
    }
}
