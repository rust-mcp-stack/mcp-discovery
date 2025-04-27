use clap::Parser;
use colored::Colorize;

use mcp_discovery::{CommandArguments, DiscoveryCommand, McpDiscovery, PrintOptions};

#[tokio::main]
async fn main() {
    let args = CommandArguments::parse();

    let command = args
        .command
        .unwrap_or(DiscoveryCommand::Print(PrintOptions {
            mcp_server_cmd: args.mcp_server_cmd,
            template: args.template,
            template_file: args.template_file,
            template_string: args.template_string,
        }));

    let launch_message = format!(
        "{} {} ...",
        "Launching:".bold(),
        &command.mcp_launch_command().join(" "),
    );

    println!("{}", launch_message.bright_green());

    let mut discovery_agent = McpDiscovery::new(command);

    if let Err(error) = discovery_agent.start().await {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}
