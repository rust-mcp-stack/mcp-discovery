use std::path::PathBuf;

use clap::{arg, command, Parser, Subcommand, ValueEnum};
use mcp_discovery::{DiscoveryCommand, LogLevel, PrintOptions, Template, WriteOptions};

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
    #[arg(short, long, value_enum, conflicts_with_all = ["template_file", "template_string"])]
    pub template: Option<CliTemplate>,

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

    /// Specifies the logging level for the application (default: info)
    #[arg(long, short)]
    pub log_level: Option<CliLogLevel>,
    /// Command and arguments to launch the MCP server.
    #[arg(
        value_name = "MCP Launch Command",
        allow_hyphen_values = true,
        last = true,
        required = true
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
            log_level: value.log_level.map(|l| l.into()),
            mcp_server_cmd: value.mcp_server_cmd,
        }
    }
}

#[derive(Parser, Debug)]
pub struct CliPrintOptions {
    /// Select an output template from the built-in options.
    #[arg(short, long, value_enum, conflicts_with_all = ["template_file", "template_string"])]
    pub template: Option<CliTemplate>,

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

    /// Specifies the logging level for the application (default: info)
    #[arg(long, short)]
    pub log_level: Option<CliLogLevel>,

    /// Command and arguments to launch the MCP server.
    #[arg(
        value_name = "MCP Launch Command",
        allow_hyphen_values = true,
        last = true,
        required = true
    )]
    pub mcp_server_cmd: Vec<String>,
}

impl From<CliPrintOptions> for PrintOptions {
    fn from(value: CliPrintOptions) -> Self {
        Self {
            template: value.template.map(|t| t.into()),
            template_file: value.template_file,
            template_string: value.template_string,
            log_level: value.log_level.map(|l| l.into()),
            mcp_server_cmd: value.mcp_server_cmd,
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
    #[arg(short, long, value_enum, conflicts_with_all = ["template_file", "template_string"])]
    pub template: Option<CliTemplate>,

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

    /// Specifies the logging level for the application (default: info)
    #[arg(long, short)]
    pub log_level: Option<CliLogLevel>,

    /// Command and arguments to launch the MCP server.
    #[arg(
        value_name = "MCP Launch Command",
        allow_hyphen_values = true,
        last = true,
        required = true
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
            log_level: None,
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
    }

    #[test]
    fn test_file_options_match_template_builtin() {
        let file_options = WriteOptions {
            filename: PathBuf::from("output.md"),
            template: Some(Template::Md),
            template_file: None,
            mcp_server_cmd: vec!["mcp-server".to_string()],
            template_string: None,
            log_level: None,
        };

        let result = file_options.match_template();
        assert!(result.is_ok(), "Expected successful template matching");
        // Note: Cannot assert specific OutputTemplate without knowing its structure
    }
}
