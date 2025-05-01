//! Module for rendering templates using Handlebars and handling MCP server template markers.

use std::{path::PathBuf, str::FromStr};

use handlebars::{
    handlebars_helper, Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext,
    RenderError,
};
use regex::Regex;
use serde::Serialize;
use serde_json::Value;
use unicode_width::UnicodeWidthStr;

use crate::{
    error::{DiscoveryError, DiscoveryResult},
    templates::{InlineTemplateInfo, PARTIALS},
    types::{ParamTypes, Template, WriteOptions},
    utils::{
        boolean_indicator, line_ending, match_template, RenderTemplateInfo, UpdateTemplateInfo,
    },
    McpServerInfo, OutputTemplate,
};

/// Properties parsed from the `MCP_DISCOVERY_TEMPLATE_START` marker line in template files.
///
/// This struct captures optional configuration attributes specified alongside the
/// `mcp-discovery-template` marker, used to define the template source for rendering
/// MCP server capabilities in the CLI tool. These properties determine whether a
/// custom template file or a built-in template (e.g., Markdown, HTML) is used.
///
/// # Example
///
/// A marker line in a file might look like:
/// ```text
/// mcp-discovery-template template-file=./custom.hbs
/// ```
/// The `RenderTemplateProps` struct would parse this as:
/// - `template_file`: `Some("./custom.hbs".to_string())`
/// - `template`: `None`
///
/// If a marker line in a file might look like:
/// ```text
/// mcp-discovery-template template=md-plain
/// ```
/// The `RenderTemplateProps` struct would parse this as:
/// - `template_file`: `None`
/// - `template`: `Some(Template::Template::MdPlain)`
///
#[derive(Debug)]
pub struct RenderTemplateProps {
    pub template_file: Option<PathBuf>,
    pub template: Option<Template>,
}

// Constants for template and render marker tags used in files.
const MCP_DISCOVERY_TEMPLATE_START: &str = "mcp-discovery-template";
const MCP_DISCOVERY_TEMPLATE_END: &str = "mcp-discovery-template-end";
const MCP_DISCOVERY_RENDER_START: &str = "mcp-discovery-render";
const MCP_DISCOVERY_RENDER_END: &str = "mcp-discovery-render-end";
const MCP_DISCOVERY_MARKER_REGEX: &str = r"\bmcp-discovery(-template|-render)(-end)?";
const MCP_DISCOVERY_TEMPLATE_FILE_REGEX: &str =
    r"(template-file=)((?:\.|~)*[\.\w\s/-]+)(?:\s|$|-->|\*/)";
const MCP_DISCOVERY_TEMPLATE_REGEX: &str = r"(template=)([\w\-\d\-]+)(\s|$)";

/// Registers custom Handlebars helpers for template rendering.
pub fn register_helpers(handlebar: &mut Handlebars) {
    // Helper: Adds 1 to an integer value.
    handlebars_helper!(plus_one: |v: i64| format!("{}", v+1));

    // Helper: replaces new line characters with provided `new_line` and optionally wraps tokens enclosed in specified delimiter pairs (like `` or '') with <code> tags.
    // This can be used to format plain text for HTML or MD display, enabling readable line breaks and inline code styling.
    handlebars_helper!(format_text: |text: Option<String>, new_line: Option<String>, code_wrap_chars:Option<String> | {

        let text = text.unwrap_or_default();

        let new_line =  new_line.unwrap_or(line_ending(&text, None).to_string());

        let mut replacers = vec![("\\n".to_string(),new_line)];

        if let Some(code_wrap_chars) = code_wrap_chars {
            // Proceed only if `code_wrap_chars` contains an even number of characters,
            // so we can pair each starting wrap character with a corresponding ending one.
            if code_wrap_chars.len() % 2 == 0 {
                for (left_char, right_char) in code_wrap_chars
                    .chars()
                    .take(code_wrap_chars.len() / 2)
                    .zip(code_wrap_chars.chars().rev())
                {
                    replacers.push((
                        format!(
                            "{}([\\w\\-\\_]+){}",
                            regex::escape(&left_char.to_string()),
                            regex::escape(&right_char.to_string()),
                        ),
                        "<code>$1</code>".to_string(),
                    ));
                }
            }
        }

        let mut result = text;
        for (regex_str, replace_str) in replacers {
            let re = Regex::new(&regex_str).unwrap();
            result = re.replace_all(&result, replace_str).to_string();
        }

        result
    });

    // Helper: Formats a capability tag with a boolean indicator and optional count.
    handlebars_helper!(capability_tag: |label:Value, supported: Value, count: Option<i64>| {
        let count_str = count.map_or("".to_string(), |count| format!(" ({})", count));
        if supported.as_bool().unwrap_or(false) {
        format!("{} {}{}", boolean_indicator(true), label.as_str().unwrap(), count_str)
        }
        else{
        format!(r#"<span style="opacity:0.6">{} {}</span>"#, boolean_indicator(false), label.as_str().unwrap())
        }
    });

    // Helper: Formats a capability with a boolean indicator and optional count.
    handlebars_helper!(capability: |label:String, supported: Option<bool>, count: Option<i64>| {
        let supported = supported.unwrap_or(false);
        let count_str = if supported && count.is_some() {
            format!(" ({})", count.unwrap())
        }
        else {
            "".to_string()
        };
        format!("{} {}{}", boolean_indicator(supported), label, count_str)
    });

    // Helper: Underlines a label with Unicode-aware width calculation.
    handlebars_helper!(underline: |label:Value| {
       let text =  label.as_str().unwrap_or_default();
        format!("{}\n{}", text, "─".repeat(text.width()))
    });

    // Helper: Formats a capability title with optional count and underline.
    handlebars_helper!(capability_title: |label:Option<String>, count: Option<i64>, with_underline:Option<bool>| {
    let label = label.unwrap_or("".to_string());
    let count_str = count.map(|c| format!("({})", c)).unwrap_or("".to_string());
    let text = format!("{}{}", label, count_str);
    let underline_str = with_underline.unwrap_or(false).then(|| format!("\n{}", "─".repeat(text.width())));
    format!("{}{}",text,underline_str.unwrap_or("".to_string()))
    });

    // Helper: Replaces text in a label using a regex pattern.
    handlebars_helper!(replace_regex: |label:Option<String>, regex:String, replacer:String| {
       let label = label.unwrap_or("".to_string());
       let re = Regex::new(&regex.to_string()).unwrap();
       let result = re.replace_all(&label, replacer.to_string());
       format!("{}", result)
    });
    // Helper: Converts a ParamTypes enum to its string representation.
    handlebars_helper!(tool_param_type: |param_type:ParamTypes| {
        param_type.to_string()
    });

    let helpers: Vec<(&str, Box<dyn HelperDef + Send + Sync>)> = vec![
        ("plus_one", Box::new(plus_one)),
        ("underline", Box::new(underline)),
        ("format_text", Box::new(format_text)),
        ("capability_tag", Box::new(capability_tag)),
        ("capability", Box::new(capability)),
        ("capability_title", Box::new(capability_title)),
        ("replace_regex", Box::new(replace_regex)),
        ("tool_param_type", Box::new(tool_param_type)),
        ("json", Box::new(json_helper)),
    ];
    // Register each helper with the Handlebars instance.
    for (name, helper) in helpers {
        handlebar.register_helper(name, helper);
    }
}

/// Handlebars helper to serialize context to JSON, with optional pretty printing.
fn json_helper(
    h: &Helper,
    _: &Handlebars,
    ctx: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let data: &Value = ctx.data();

    // Check if the first param (pretty) is passed and true
    let pretty = h
        .param(0)
        .and_then(|v| v.value().as_bool())
        .unwrap_or(false);

    let json_output = if pretty {
        serde_json::to_string_pretty(data)
    } else {
        serde_json::to_string(data)
    }
    .unwrap_or_else(|_| String::from("/* failed to serialize */"));

    out.write(&json_output)?;
    Ok(())
}

// Registers Handlebars partials from the PARTIALS constant.
pub fn register_partials(handlebar: &mut Handlebars) {
    for (name, template) in PARTIALS {
        handlebar.register_partial(name, template).unwrap();
    }
}

/// Renders a template with the provided data using Handlebars.
pub fn render_template<T>(template: &OutputTemplate, data: &T) -> Result<String, RenderError>
where
    T: Serialize,
{
    let mut handlebar: Handlebars = Handlebars::new();

    register_helpers(&mut handlebar);
    register_partials(&mut handlebar);

    let template_content = template.content();

    handlebar.render_template(&template_content, &data)
}

/// Select the template to be used, considering template and template-file and inline templates
/// that are passed via CLI or set as properties of the markers
fn select_template(
    update_options: &WriteOptions,
    rendering_props: &RenderTemplateProps,
    inline_template: Option<OutputTemplate>,
) -> DiscoveryResult<OutputTemplate> {
    // template argument or template property of the render block
    let template_name = update_options
        .template
        .as_ref()
        .or(rendering_props.template.as_ref())
        .map(OutputTemplate::from);

    // template-file from argument or from render block
    let template_file = update_options
        .template_file
        .as_ref()
        .or(rendering_props.template_file.as_ref())
        .map(|t| OutputTemplate::from_file(t, Some(&update_options.filename)))
        .transpose()?;

    let no_template: Option<Template> = None;
    let no_template_file: Option<PathBuf> = None;
    let no_template_string: Option<String> = None;

    Ok(template_file
        .or(template_name)
        .or(inline_template)
        .unwrap_or(match_template(
            Some(&update_options.filename),
            &no_template,
            &no_template_file,
            &no_template_string,
        )?))
}

/// Detects and processes template and render markers in a file for updating.
pub fn detect_render_markers(
    update_options: &WriteOptions,
    server_info: &McpServerInfo,
) -> DiscoveryResult<UpdateTemplateInfo> {
    let content = std::fs::read_to_string(update_options.filename.as_path())
        .expect("Failed to read test data");

    let line_ending = line_ending(content.as_str(), None).to_owned();

    let re = Regex::new(MCP_DISCOVERY_MARKER_REGEX)?;

    let mut inside_template = false;
    let mut inside_render = false;

    let mut template_markers_start: Option<usize> = None;
    let mut render_markers_start: Option<usize> = None;
    let mut render_locations: Vec<RenderTemplateInfo> = vec![];

    let mut last_template: Option<OutputTemplate> = None;
    let mut rendering_props = RenderTemplateProps {
        template_file: None,
        template: None,
    };

    for mat in re.captures_iter(&content) {
        let tag = mat.get(0).unwrap().as_str();
        let pos = mat.get(0).unwrap().start();

        let slice = &content[..pos];
        let line_number = slice.matches('\n').count() + 1;

        match tag {
            MCP_DISCOVERY_TEMPLATE_START => {
                if inside_template {
                    return Err(DiscoveryError::ParseTemplate(format!(
                        "Duplicate template start marker '{}' found at line {} in '{}'. Ensure each template section has a single start marker.",
                        MCP_DISCOVERY_TEMPLATE_START,
                        line_number,
                        update_options.filename.display()
                    )));
                }

                if !inside_render {
                    return Err(DiscoveryError::ParseTemplate(format!(
                        "Template start marker '{}' at line {} in '{}' is outside a render section. Ensure it is enclosed within '{}' and '{}' markers.",
                        MCP_DISCOVERY_TEMPLATE_START,
                        line_number,
                        update_options.filename.display(),
                        MCP_DISCOVERY_RENDER_START,
                        MCP_DISCOVERY_RENDER_END
                    )));
                }

                inside_template = true;
                template_markers_start = Some(line_number);
            }
            MCP_DISCOVERY_TEMPLATE_END => {
                if !inside_template {
                    return Err(DiscoveryError::ParseTemplate(format!(
                        "Template end marker '{}' at line {} in '{}' has no matching start marker '{}'. Add a corresponding start marker before this line.",
                        MCP_DISCOVERY_TEMPLATE_END,
                        MCP_DISCOVERY_TEMPLATE_START,
                        line_number,
                        update_options.filename.display()
                    )));
                }

                if !inside_render {
                    return Err(DiscoveryError::ParseTemplate(format!(
                        "Template end marker '{}' at line {} in '{}' is outside a render section. Ensure it is enclosed within '{}' and '{}' markers.",
                        MCP_DISCOVERY_TEMPLATE_END,
                        line_number,
                        update_options.filename.display(),
                        MCP_DISCOVERY_RENDER_START,
                        MCP_DISCOVERY_RENDER_END
                    )));
                }

                inside_template = false;

                if last_template.is_some() {
                    eprintln!(
                        "WARNING: Template section starting at line {} in '{}' was ignored because it was not followed by a render section. Ensure it is within a valid render block.",
                        template_markers_start.unwrap(),
                        update_options.filename.display()
                    );
                }

                let start_line = template_markers_start.unwrap();

                let template_content = &content
                    .lines()
                    .skip(start_line)
                    .take(line_number - start_line - 1)
                    .collect::<Vec<_>>()
                    .join(&line_ending);

                last_template = Some(OutputTemplate::InlineTemplate(InlineTemplateInfo {
                    template: template_content.to_owned(),
                    marker_start: content.lines().nth(start_line - 1).unwrap_or("").to_owned(),
                    marker_end: content
                        .lines()
                        .nth(line_number - 1)
                        .unwrap_or("")
                        .to_owned(),
                }));

                template_markers_start = None;
            }
            MCP_DISCOVERY_RENDER_START => {
                if inside_render {
                    return Err(DiscoveryError::ParseTemplate(format!(
                        "Duplicate render start marker '{}' found at line {} in '{}'. Remove the extra marker to define a single render section.",
                        MCP_DISCOVERY_RENDER_START,
                        line_number,
                        update_options.filename.display()
                    )));
                }
                if inside_template {
                    return Err(DiscoveryError::ParseTemplate(format!(
                        "Render start marker '{}' at line {} in '{}' is inside a template section. Move it outside the '{}' and '{}' markers.",
                        MCP_DISCOVERY_RENDER_START,
                        line_number,
                        update_options.filename.display(),
                        MCP_DISCOVERY_TEMPLATE_START,
                        MCP_DISCOVERY_TEMPLATE_END
                    )));
                }

                rendering_props =
                    extract_render_props(content.lines().nth(line_number - 1).unwrap());

                inside_render = true;
                render_markers_start = Some(line_number);
            }
            MCP_DISCOVERY_RENDER_END => {
                if !inside_render {
                    return Err(DiscoveryError::ParseTemplate(format!(
                        "Render end marker '{}' at line {} in '{}' has no matching start marker '{}'. Add a corresponding start marker before this line.",
                        MCP_DISCOVERY_RENDER_END,
                        MCP_DISCOVERY_RENDER_START,
                        line_number,
                        update_options.filename.display()
                    )));
                }

                if inside_template {
                    return Err(DiscoveryError::ParseTemplate(format!(
                        "Render end marker '{}' at line {} in '{}' is inside a template section. Close the template section with '{}' before this marker.",
                        MCP_DISCOVERY_RENDER_END,
                        line_number,
                        update_options.filename.display(),
                        MCP_DISCOVERY_TEMPLATE_END
                    )));
                }

                if rendering_props.template_file.is_some() && last_template.is_some() {
                    return Err(DiscoveryError::ParseTemplate(format!(
                        "Render section ending at line {} in '{}' specifies both a 'template-file' and an inline template. Choose one template source for this render block.",
                        line_number,
                        update_options.filename.display()
                    )));
                }

                if rendering_props.template.is_some() && last_template.is_some() {
                    return Err(DiscoveryError::ParseTemplate(format!(
                        "Render section ending at line {} in '{}' specifies both a 'template' and an inline template. Choose one template source for this render block.",
                        line_number,
                        update_options.filename.display()
                    )));
                }

                if rendering_props.template_file.is_some() && rendering_props.template.is_some() {
                    return Err(DiscoveryError::ParseTemplate(format!(
                        "Render section ending at line {} in '{}' specifies both a 'template-file' and 'template'. Choose one template source for this render block.",
                        line_number,
                        update_options.filename.display()
                    )));
                }

                inside_render = false;

                let template =
                    select_template(update_options, &rendering_props, last_template.take())?;

                // prepend the inline template before the rendered template, to preserve the inline template
                let rendered_template = template.render_template(server_info)?;

                render_locations.push(RenderTemplateInfo {
                    render_location: (render_markers_start.unwrap(), line_number),
                    rendered_template,
                });
            }
            _ => {}
        }
    }

    Ok(UpdateTemplateInfo {
        content,
        render_locations,
        line_ending,
    })
}

/// Extracts a template file path from a marker line using a regex.
pub fn extract_template_file(line: &str) -> Option<String> {
    let re = Regex::new(MCP_DISCOVERY_TEMPLATE_FILE_REGEX).unwrap();

    re.captures(line)
        .and_then(|cap| cap.get(2).map(|m| m.as_str().trim().to_string()))
}

/// Extracts a template file path from a marker line using a regex.
pub fn extract_template_prop(line: &str) -> Option<Template> {
    let re = Regex::new(MCP_DISCOVERY_TEMPLATE_REGEX).unwrap();

    let prop_value = re
        .captures(line)
        .and_then(|cap| cap.get(2).map(|m| m.as_str().trim().to_string()));

    let template = Template::from_str(&prop_value.unwrap_or_default());
    template.ok()
}

pub fn extract_render_props(line: &str) -> RenderTemplateProps {
    RenderTemplateProps {
        template: extract_template_prop(line),
        template_file: extract_template_file(line).map(PathBuf::from),
    }
}
