use colored::Colorize;
use std::io::{self, Write};
use unicode_width::UnicodeWidthStr;

use crate::{error::DiscoveryResult, utils::boolean_indicator, McpServerInfo};

const SUMMARY_HEADER_SIZE: usize = 44;

/// Function to print a list of items to the given writer `w`.
/// Each item is printed with an index, key and value.
pub fn print_list<W: Write>(mut w: W, list_items: Vec<(String, String)>) -> io::Result<()> {
    for (index, (key, val)) in list_items.iter().enumerate() {
        writeln!(
            w,
            "{}. {}: {}\n",
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

/// Function to print the server information as JSON.
pub fn print_json<W: Write>(mut w: W, server_info: &McpServerInfo) -> DiscoveryResult<()> {
    let json_content = serde_json::to_string(server_info)?;
    writeln!(w, "{}", json_content)?;
    Ok(())
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
            "{} Tools    {} Prompts    {} Resources",
            boolean_indicator(caps.tools),
            boolean_indicator(caps.prompts),
            boolean_indicator(caps.resources)
        ),
        format!(
            "{} Logging  {} Experimental",
            boolean_indicator(caps.logging),
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
