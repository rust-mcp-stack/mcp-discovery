use path_clean::PathClean;

use crate::{OutputTemplate, error::DiscoveryResult, types::Template};
use std::{
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct RenderTemplateInfo {
    pub rendered_template: String,
    pub render_location: (usize, usize),
}

#[derive(Debug)]
pub struct UpdateTemplateInfo {
    /// Content of the file to be updated by mcp-discovery
    pub content: String,
    pub line_ending: String,
    pub render_locations: Vec<RenderTemplateInfo>,
}

pub fn line_ending(content: &str, line_number: Option<usize>) -> &str {
    let line_number = line_number.unwrap_or(1);
    let target = line_number.saturating_sub(1); // 0-based index

    if let Some(line) = content.lines().nth(target) {
        let start = line.as_ptr() as usize - content.as_ptr() as usize + line.len();
        let next_chars = &content[start..];
        if next_chars.starts_with("\r\n") {
            return "\r\n";
        } else if next_chars.starts_with("\n") {
            return "\n";
        }
    }
    "\n" // Default if line not found or no line ending
}

pub fn match_template(
    filename: Option<&PathBuf>,
    template: &Option<Template>,
    template_file: &Option<PathBuf>,
    template_string: &Option<String>,
) -> DiscoveryResult<OutputTemplate> {
    if let Some(template_file) = template_file {
        return OutputTemplate::from_file(template_file, filename);
    } else if let Some(template) = template {
        return Ok(template.into());
    } else if let Some(template_string) = template_string {
        return Ok(OutputTemplate::TemplateString(template_string.to_owned()));
    }

    if let Some(filename) = filename {
        // detect appropriate template based on the file extension, default to txt
        let extension = filename
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        match extension {
            "txt" => Ok(OutputTemplate::Txt),
            "md" | "markdown" | "mdown" | "mkd" | "mdtxt" | "mdtext" => Ok(OutputTemplate::Md),
            "htm" | "html" => Ok(OutputTemplate::Html),
            _ => Ok(OutputTemplate::Txt),
        }
    } else {
        // it is a print command
        Ok(OutputTemplate::None)
    }
}

pub fn boolean_indicator(boolean: bool) -> char {
    match boolean {
        true => '✔',
        false => '✘',
    }
}

pub fn absolute_from_relative(
    base_file_path: &Path,
    relative_file_path: &Path,
) -> io::Result<PathBuf> {
    // Get the parent directory of the base path (since base is a file)
    let base_dir = base_file_path.parent().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "Base path has no parent directory",
        )
    })?;
    // Join the relative path to the base directory
    let joined_path = base_dir.join(relative_file_path);

    let cwd = std::env::current_dir()?;
    Ok(cwd.join(joined_path).clean())
}

pub fn find_template_file(
    template_file: &Path,
    base_file: Option<&PathBuf>,
) -> DiscoveryResult<PathBuf> {
    let relative_path = base_file.and_then(|f| absolute_from_relative(f, template_file).ok());

    let search_paths: Vec<_> = relative_path
        .into_iter()
        .chain([template_file.to_path_buf()])
        .rev()
        .collect();

    search_paths.iter().find(|p| p.exists()).cloned().ok_or(
        io::Error::new(
            ErrorKind::NotFound,
            format!(
                "Template file not found in any of these paths:\n{}",
                search_paths
                    .iter()
                    .map(|p| p.to_string_lossy())
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
        )
        .into(),
    )
}
