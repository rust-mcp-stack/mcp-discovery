use crate::{
    McpServerInfo,
    error::DiscoveryResult,
    render_template,
    types::Template,
    utils::{find_template_file, line_ending},
};
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

// Static string constants for built-in template files
const TEMPLATE_MARKDOWN: &str = include_str!("../templates/markdown/markdown_template.md");
const TEMPLATE_HTML: &str = include_str!("../templates/html/html_template.html");
const TEMPLATE_MARKDOWN_PLAIN: &str =
    include_str!("../templates/markdown/markdown_plain_template.md");
const TEMPLATE_TEXT: &str = include_str!("../templates/text/text_template.txt");

pub const TITLE_VERSION: &str = include_str!("../templates/common/title.hbs");
pub const MD_SUMMARY: &str = include_str!("../templates/markdown/summary.hbs");

// md partials
pub const MD_TOOLS: &str = include_str!("../templates/markdown/md_tools.hbs");
pub const MD_PROMPTS: &str = include_str!("../templates/markdown/md_prompts.hbs");
pub const MD_RESOURCES: &str = include_str!("../templates/markdown/md_resources.hbs");
pub const MD_RESOURCE_TEMPLATES: &str =
    include_str!("../templates/markdown/md_resource_templates.hbs");

// md-plain partials
pub const MD_PLAIN_TOOLS: &str = include_str!("../templates/markdown/md_plain_tools.hbs");
pub const MD_PLAIN_PROMPTS: &str = include_str!("../templates/markdown/md_plain_prompts.hbs");
pub const MD_PLAIN_RESOURCES: &str = include_str!("../templates/markdown/md_plain_resources.hbs");
pub const MD_PLAIN_RESOURCE_TEMPLATES: &str =
    include_str!("../templates/markdown/md_plain_resource_templates.hbs");

// html partials
pub const HTML_SUMMARY: &str = include_str!("../templates/html/html_summary.hbs");
pub const HTML_TOOLS: &str = include_str!("../templates/html/html_tools.hbs");
pub const HTML_PROMPTS: &str = include_str!("../templates/html/html_prompts.hbs");
pub const HTML_RESOURCES: &str = include_str!("../templates/html/html_resources.hbs");
pub const HTML_RESOURCE_TEMPLATES: &str =
    include_str!("../templates/html/html_resource_templates.hbs");

// text partials
pub const TEXT_SUMMARY: &str = include_str!("../templates/text/text_summary.hbs");
pub const TEXT_TOOLS: &str = include_str!("../templates/text/text_tools.hbs");
pub const TEXT_PROMPTS: &str = include_str!("../templates/text/text_prompts.hbs");
pub const TEXT_RESOURCES: &str = include_str!("../templates/text/text_resources.hbs");
pub const TEXT_RESOURCE_TEMPLATES: &str =
    include_str!("../templates/text/text_resource_templates.hbs");

pub static PARTIALS: [(&str, &str); 21] = [
    ("title-version", TITLE_VERSION),
    ("summary", MD_SUMMARY),
    ("md-tools", MD_TOOLS),
    ("md-prompts", MD_PROMPTS),
    ("md-resources", MD_RESOURCES),
    ("md-resource-templates", MD_RESOURCE_TEMPLATES),
    ("md-plain-tools", MD_PLAIN_TOOLS),
    ("md-plain-prompts", MD_PLAIN_PROMPTS),
    ("md-plain-resources", MD_PLAIN_RESOURCES),
    ("md-plain-resource-templates", MD_PLAIN_RESOURCE_TEMPLATES),
    ("html-summary", HTML_SUMMARY),
    ("html-tools", HTML_TOOLS),
    ("html-prompts", HTML_PROMPTS),
    ("html-resources", HTML_RESOURCES),
    ("html-resource-templates", HTML_RESOURCE_TEMPLATES),
    ("txt-summary", TEXT_SUMMARY),
    ("txt-tools", TEXT_TOOLS),
    ("txt-prompts", TEXT_PROMPTS),
    ("txt-resources", TEXT_RESOURCES),
    ("txt-resource-templates", TEXT_RESOURCE_TEMPLATES),
    ("txt-summary", TEXT_SUMMARY),
];

/// Struct to hold information about inline templates
/// Used for templates embedded within other content with specific markers
#[derive(Debug)]
pub struct InlineTemplateInfo {
    pub template: String,
    pub marker_start: String,
    pub marker_end: String,
}

/// Enum representing different types of output templates
/// Used to specify the type of template to render
#[derive(Debug)]
pub enum OutputTemplate {
    /// Markdown template (generates tables)
    Md,
    /// HTML template
    Html,
    /// Plain text template
    Txt,
    /// MD Plain template
    MdPlain,
    /// Custom template from file
    CustomTemplate(PathBuf),
    /// Template from string
    TemplateString(String),
    /// Inline template with markers
    InlineTemplate(InlineTemplateInfo),
    // Print to the terminal
    None,
}

impl OutputTemplate {
    /// Creates an OutputTemplate from a file path
    /// Resolves the template file path relative to a base file if provided
    /// Returns a DiscoveryResult containing the CustomTemplate variant
    pub fn from_file(template_file: &Path, base_file: Option<&PathBuf>) -> DiscoveryResult<Self> {
        let actual_template_file = find_template_file(template_file, base_file)?;
        Ok(OutputTemplate::CustomTemplate(actual_template_file))
    }

    /// Returns the content of the template as a Cow string
    pub fn content(&self) -> Cow<'_, str> {
        match &self {
            // Return borrowed references to static templates
            Self::Md => Cow::Borrowed(TEMPLATE_MARKDOWN),
            Self::MdPlain => Cow::Borrowed(TEMPLATE_MARKDOWN_PLAIN),
            Self::Html => Cow::Borrowed(TEMPLATE_HTML),
            Self::Txt => Cow::Borrowed(TEMPLATE_TEXT),
            // Read custom template from file, return error message instead of the template if reading fails
            Self::CustomTemplate(path_buf) => {
                let content = std::fs::read_to_string(path_buf).unwrap_or(format!(
                    ">> ERROR LOADING TEMPLATE FILE : '{}' <<",
                    path_buf.to_string_lossy()
                ));
                Cow::Owned(content)
            }
            Self::TemplateString(template_str) => Cow::Owned(template_str.to_owned()),
            OutputTemplate::InlineTemplate(inline_template_info) => {
                Cow::Owned(inline_template_info.template.to_owned())
            }
            Self::None => Cow::Owned("".into()),
        }
    }

    /// Generates formatted inline template, including markers and proper line endings
    /// Used for InlineTemplate variant to format the output with start/end markers
    fn inline_template(&self, inline_template_info: &InlineTemplateInfo) -> String {
        let line_ending = line_ending(&inline_template_info.template, None);
        format!(
            "{}{}{}{}{}{}",
            inline_template_info.marker_start,
            line_ending,
            inline_template_info.template,
            line_ending,
            inline_template_info.marker_end,
            line_ending,
        )
    }

    /// Renders the template with provided server information
    /// Returns the rendered output as a `DiscoveryResult<String>`
    pub fn render_template(&self, server_info: &McpServerInfo) -> DiscoveryResult<String> {
        let rendered = render_template(self, server_info)?;
        match self {
            OutputTemplate::InlineTemplate(inline_template_info) => Ok(format!(
                "{}{}",
                self.inline_template(inline_template_info),
                rendered
            )),
            _ => Ok(rendered),
        }
    }
}

impl From<&Template> for OutputTemplate {
    fn from(value: &Template) -> Self {
        match value {
            Template::Md => OutputTemplate::Md,
            Template::Html => OutputTemplate::Html,
            Template::Txt => OutputTemplate::Txt,
            Template::MdPlain => OutputTemplate::MdPlain,
        }
    }
}

impl From<Template> for OutputTemplate {
    fn from(value: Template) -> Self {
        match value {
            Template::Md => OutputTemplate::Md,
            Template::Html => OutputTemplate::Html,
            Template::Txt => OutputTemplate::Txt,
            Template::MdPlain => OutputTemplate::MdPlain,
        }
    }
}
