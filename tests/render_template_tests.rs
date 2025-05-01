#[path = "common/common.rs"]
pub mod common;

use common::default_mcp_server_info;
use handlebars::Handlebars;
use mcp_discovery::{
    render_template::{detect_render_markers, extract_template_file, register_helpers},
    types::{ParamTypes, WriteOptions},
    OutputTemplate,
};
use serde_json::json;
use std::fs::write;
use tempfile::NamedTempFile;

#[test]
fn test_register_helpers() {
    let mut handlebar = Handlebars::new();
    register_helpers(&mut handlebar);

    // Test plus_one helper
    let result = handlebar
        .render_template("{{plus_one 5}}", &json!({}))
        .expect("Failed to render plus_one");
    assert_eq!(result, "6");

    // Test capability_tag helper (supported)
    let result = handlebar
        .render_template("{{capability_tag \"Feature\" true 42}}", &json!({}))
        .expect("Failed to render capability_tag");
    assert_eq!(result, "🟢 Feature (42)");

    // Test capability_tag helper (not supported)
    let result = handlebar
        .render_template("{{{capability_tag \"Feature\" false 0}}}", &json!({}))
        .expect("Failed to render capability_tag");
    assert_eq!(result, r#"<span style="opacity:0.6">🔴 Feature</span>"#);

    // Test capability helper
    let result = handlebar
        .render_template("{{{capability \"Feature\" true 42}}}", &json!({}))
        .expect("Failed to render capability");
    assert_eq!(result, "🟢 Feature (42)");

    // Test underline helper
    let result = handlebar
        .render_template("{{underline \"Title\"}}", &json!({}))
        .expect("Failed to render underline");
    assert_eq!(result, "Title\n─────");

    // Test capability_title helper
    let result = handlebar
        .render_template("{{capability_title \"Title\" 10 true}}", &json!({}))
        .expect("Failed to render capability_title");
    assert_eq!(result, "Title(10)\n─────────");

    // Test replace_regex helper
    let result = handlebar
        .render_template(
            "{{replace_regex \"Hello World\" \"World\" \"Rust\"}}",
            &json!({}),
        )
        .expect("Failed to render replace_regex");
    assert_eq!(result, "Hello Rust");

    // Test json helper (pretty)
    let result = handlebar
        .render_template("{{json true}}", &json!({"key": "value"}))
        .expect("Failed to render json");
    assert_eq!(result, "{\n  \"key\": \"value\"\n}");

    // Test tool_param_type helper
    let result = handlebar
        .render_template(
            "{{tool_param_type param}}",
            &json!({"param": ParamTypes::Primitive("string".to_string())}),
        )
        .expect("Failed to render tool_param_type");
    assert_eq!(result, "string");
}

#[test]
fn test_render_template() {
    let template = OutputTemplate::TemplateString("Hello, {{name}}!".to_string());
    let data = json!({"name": "World"});
    let result = mcp_discovery::render_template::render_template(&template, &data)
        .expect("Failed to render template");
    assert_eq!(result, "Hello, World!");

    // Test with helper
    let template = OutputTemplate::TemplateString("{{plus_one 5}}".to_string());
    let result = mcp_discovery::render_template::render_template(&template, &json!({}))
        .expect("Failed to render template");
    assert_eq!(result, "6");

    // Test invalid template
    let template = OutputTemplate::TemplateString("{{#invalid}}".to_string());
    let result = mcp_discovery::render_template::render_template(&template, &json!({}));
    assert!(result.is_err(), "Expected error for invalid template");
}

#[test]
fn test_detect_render_markers_valid() {
    let file = NamedTempFile::new().unwrap();
    let content = r#"mcp-discovery-render
    mcp-discovery-template
    Template content
    mcp-discovery-template-end
    mcp-discovery-render-end"#;
    write(&file, content).unwrap();
    let options = WriteOptions {
        filename: file.path().to_path_buf(),
        template: None,
        template_file: None,
        mcp_server_cmd: vec!["mcp-server".to_string()],
        template_string: None,
        log_level: None,
    };
    let server_info = default_mcp_server_info();
    let result = detect_render_markers(&options, &server_info);
    assert!(result.is_ok(), "Expected valid marker detection");
    let update_info = result.unwrap();
    assert_eq!(update_info.render_locations.len(), 1);
    assert_eq!(update_info.line_ending, "\n");
    assert_eq!(update_info.render_locations[0].render_location, (1, 5));
}

#[test]
fn test_detect_render_markers_duplicate_template_start() {
    let file = NamedTempFile::new().unwrap();
    let content = r#"mcp-discovery-render
    mcp-discovery-template
    mcp-discovery-template
    Template content
    mcp-discovery-template-end
    mcp-discovery-render-end"#;
    write(&file, content).unwrap();
    let options = WriteOptions {
        filename: file.path().to_path_buf(),
        template: None,
        template_file: None,
        mcp_server_cmd: vec!["mcp-server".to_string()],
        template_string: None,
        log_level: None,
    };
    let server_info = default_mcp_server_info();
    let result = detect_render_markers(&options, &server_info);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Duplicate template start marker"));
}

#[test]
fn test_detect_render_markers_template_outside_render() {
    let file = NamedTempFile::new().unwrap();
    let content = r#"
    mcp-discovery-template
    Template content
    mcp-discovery-template-end
    "#;
    write(&file, content).unwrap();
    let options = WriteOptions {
        filename: file.path().to_path_buf(),
        template: None,
        template_file: None,
        mcp_server_cmd: vec!["mcp-server".to_string()],
        template_string: None,
        log_level: None,
    };
    let server_info = default_mcp_server_info();
    let result = detect_render_markers(&options, &server_info);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("outside a render section"));
}

#[test]
fn test_detect_render_markers_unmatched_template_end() {
    let file = NamedTempFile::new().unwrap();
    let content = r#"
    mcp-discovery-render
    mcp-discovery-template-end
    mcp-discovery-render-end
    "#;
    write(&file, content).unwrap();
    let options = WriteOptions {
        filename: file.path().to_path_buf(),
        template: None,
        template_file: None,
        mcp_server_cmd: vec!["mcp-server".to_string()],
        template_string: None,
        log_level: None,
    };
    let server_info = default_mcp_server_info();
    let result = detect_render_markers(&options, &server_info);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("no matching start marker"));
}

#[test]
fn test_detect_render_markers_conflicting_templates() {
    let file = NamedTempFile::new().unwrap();
    let content = r#"
    mcp-discovery-render template-file=./template.hbs
    mcp-discovery-template
    Template content
    mcp-discovery-template-end
    mcp-discovery-render-end"#;
    write(&file, content).unwrap();
    let options = WriteOptions {
        filename: file.path().to_path_buf(),
        template: None,
        template_file: None,
        mcp_server_cmd: vec!["mcp-server".to_string()],
        template_string: None,
        log_level: None,
    };
    let server_info = default_mcp_server_info();
    let result = detect_render_markers(&options, &server_info);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("both a 'template-file' and an inline template"));
}

#[test]
fn test_extract_template_file() {
    let line = "mcp-discovery-render template-file=./template.hbs";
    let result = extract_template_file(line);
    assert_eq!(result, Some("./template.hbs".to_string()));

    let line = "mcp-discovery-render";
    let result = extract_template_file(line);
    assert_eq!(result, None);
}
