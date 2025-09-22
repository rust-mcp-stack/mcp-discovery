use colored::Colorize;
use std::io::{self, Write};
use unicode_width::UnicodeWidthStr;

use crate::{utils::boolean_indicator, McpServerInfo};

const SUMMARY_HEADER_SIZE: usize = 50;

/// Function to print a list of items to the given writer `w`.
/// Each item is printed with an index, key and value.
pub fn print_list<W: Write>(mut w: W, list_items: Vec<(String, String)>) -> io::Result<()> {
    for (index, (key, val)) in list_items.iter().enumerate() {
        writeln!(
            w,
            "{}. {}: {}",
            (index + 1).to_string().bold().cyan(),
            key.bold().cyan(),
            val
        )?;
    }
    Ok(())
}

/// Function to print a header table with a title and table size to the writer `w`.
/// The header includes a top border, the title in the center, and a bottom border.
pub fn print_header<W: Write>(mut w: W, title: &str, table_size: usize) -> io::Result<()> {
    writeln!(w, "{}", table_top(table_size))?;
    writeln!(w, "{}", table_content(table_size, title))?;
    writeln!(w, "{}", table_bottom(table_size))?;
    Ok(())
}

/// Generates the top border for the table based on the given width (`width`).
/// Uses a character `─` for the border.
pub fn table_top(width: usize) -> String {
    format!("┌{:─<w$}┐", "", w = width)
}

/// Generates the bottom border for the table, similar to `table_top`.
pub fn table_bottom(width: usize) -> String {
    format!("└{:─<w$}┘", "", w = width)
}

/// Function to create a table row with centered content.
/// The row will be padded with spaces to fit the required table width.
pub fn table_content(width: usize, content: &str) -> String {
    let title_length = strip_ansi_escapes::strip_str(content).width();
    let l_pad = (((width) as f32 / 2.0) - (title_length as f32 / 2.0)).floor() as usize;
    let r_pad = width - l_pad - title_length;
    format!("│{}{}{}│", " ".repeat(l_pad), content, " ".repeat(r_pad))
}

/// Function to print a formatted summary of the server information.
pub fn print_summary<W: Write>(mut w: W, server_info: &McpServerInfo) -> io::Result<usize> {
    let server_name = format!("{} {}", server_info.name, server_info.version);
    let title_length = server_name.width(); // Use display width for accuracy
    let table_size = SUMMARY_HEADER_SIZE.max(title_length + 4); // 2 padding on each side

    writeln!(w, "{}", table_top(table_size))?;
    writeln!(
        w,
        "{}",
        table_content(table_size, &server_name.bold().cyan().to_string())
    )?;
    writeln!(w, "{}", table_content(table_size, ""))?;

    let caps = &server_info.capabilities;
    let lines = [
        format!(
            "{} Tools   {} Prompts   {} Resources   {} Logging",
            boolean_indicator(caps.tools),
            boolean_indicator(caps.prompts),
            boolean_indicator(caps.resources),
            boolean_indicator(caps.logging),
        ),
        format!(
            "{} Completions   {} Experimental",
            boolean_indicator(caps.completions),
            boolean_indicator(caps.experimental)
        ),
    ];

    let adjust = lines[0].width().saturating_sub(lines[1].width());
    writeln!(w, "{}", table_content(table_size, &lines[0]))?;
    writeln!(
        w,
        "{}",
        table_content(table_size, &format!("{}{}", &lines[1], " ".repeat(adjust)))
    )?;

    writeln!(w, "{}", table_bottom(table_size))?;
    Ok(table_size)
}

#[cfg(test)]
mod tests {
    use crate::McpCapabilities;

    use super::*;

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
                completions: true,
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
                completions: true,
            },
            tools: None,
            prompts: None,
            resources: None,
            resource_templates: None,
        };

        // Print directly to stdout
        print_summary(io::stdout(), &info).unwrap();
    }
}
