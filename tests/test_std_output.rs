use mcp_discovery::{
    std_output::{
        print_header, print_json, print_list, print_summary, table_bottom, table_content, table_top,
    },
    types::{McpCapabilities, McpServerInfo},
};
use std::io::{self};
use unicode_width::UnicodeWidthStr;

#[test]
fn test_table_top_bottom() {
    assert_eq!(table_top(5), "┌─────┐");
    assert_eq!(table_bottom(5), "└─────┘");
}

#[test]
fn test_table_content_centering() {
    let result = table_content(10, "Hi");
    assert!(result.starts_with("│"));
    assert!(result.ends_with("│"));
    assert_eq!(strip_ansi_escapes::strip_str(&result).width(), 12); // includes borders
    assert!(result.contains("Hi"));
}

#[test]
fn test_print_list() {
    let items = vec![
        ("Item1".to_string(), "Value1".to_string()),
        ("Item2".to_string(), "Value2".to_string()),
    ];

    let mut buffer = Vec::new();
    print_list(&mut buffer, items).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    let output = strip_ansi_escapes::strip_str(output);

    assert!(output.contains("1. Item1: Value1"));
    assert!(output.contains("2. Item2: Value2"));
}

#[test]
fn test_print_header_structure() {
    let mut buffer = Vec::new();
    print_header(&mut buffer, "My Header", 20).unwrap();

    // Create the String first
    let output = String::from_utf8(buffer).unwrap();

    // Now borrow the String with `lines()`
    let lines: Vec<&str> = output.lines().collect();

    assert_eq!(lines.len(), 3);
    assert!(lines[0].starts_with("┌") && lines[0].ends_with("┐"));
    assert!(lines[2].starts_with("└") && lines[2].ends_with("┘"));
    assert!(lines[1].contains("My Header"));
}

#[test]
fn test_print_json_output() {
    let info = McpServerInfo {
        name: "Test".to_string(),
        version: "1.0".to_string(),
        capabilities: McpCapabilities {
            tools: true,
            prompts: false,
            resources: true,
            logging: false,
            experimental: true,
        },
        tools: None,
        prompts: None,
        resources: None,
        resource_templates: None,
    };

    let mut buffer = Vec::new();
    print_json(&mut buffer, &info).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    let json_value: serde_json::Value = serde_json::from_str(output.trim()).unwrap();
    assert_eq!(json_value["name"], "Test");
    assert_eq!(json_value["version"], "1.0");
}

#[test]
fn test_print_summary_layout() {
    let info = McpServerInfo {
        name: "MyApp".to_string(),
        version: "0.9".to_string(),
        capabilities: McpCapabilities {
            tools: true,
            prompts: false,
            resources: false,
            logging: true,
            experimental: false,
        },
        tools: None,
        prompts: None,
        resources: None,
        resource_templates: None,
    };

    let mut buffer = Vec::new();
    let size = print_summary(&mut buffer, &info).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert!(output.contains("MyApp 0.9"));
    assert!(size >= 44);
}

#[test]
fn test_print_to_stdout() {
    let items = vec![
        ("Tool1".to_string(), "Enabled".to_string()),
        ("Tool2".to_string(), "Disabled".to_string()),
    ];

    // Print directly to stdout
    print_list(io::stdout(), items).unwrap();
}

#[test]
fn test_print_summary_to_stdout() {
    let info = McpServerInfo {
        name: "SampleServer".to_string(),
        version: "1.1".to_string(),
        capabilities: McpCapabilities {
            tools: true,
            prompts: true,
            resources: true,
            logging: false,
            experimental: false,
        },
        tools: None,
        prompts: None,
        resources: None,
        resource_templates: None,
    };

    // Print directly to stdout
    print_summary(io::stdout(), &info).unwrap();
}
