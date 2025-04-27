#[path = "common/common.rs"]
pub mod common;

use clap::Parser;
use common::parse_args;
use mcp_discovery::{CommandArguments, DiscoveryCommand, Template, WriteOptions};
use std::path::PathBuf;

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
        Some(DiscoveryCommand::Create(file_options)) => {
            assert_eq!(file_options.filename, PathBuf::from("output.md"));
            assert_eq!(file_options.template, Some(Template::Md));
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
        Some(DiscoveryCommand::Update(file_options)) => {
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
        Some(DiscoveryCommand::Print(print_options)) => {
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
    let parsed = parse_args(args);

    let launch_cmd = parsed.mcp_launch_command();
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
    };

    let result = file_options.match_template();
    assert!(result.is_ok(), "Expected successful template matching");
    // Note: Cannot assert specific OutputTemplate without knowing its structure
}

#[test]
fn test_file_options_match_template_custom() {
    let file_options = WriteOptions {
        filename: PathBuf::from("output.html"),
        template: None,
        template_file: Some(PathBuf::from("templates/markdown/markdown_template.md")),
        mcp_server_cmd: vec!["mcp-server".to_string()],
        template_string: None,
    };

    let result = file_options.match_template();

    assert!(
        result.is_ok(),
        "Expected successful template matching with custom file"
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
fn test_version_flag() {
    let args = vec!["mcp-tool", "--version"];
    let result = CommandArguments::try_parse_from(args);
    assert!(result.is_err(), "Expected clap to handle --version flag");
    // Note: clap automatically handles --version and exits, so this test verifies it doesn't parse normally
}
