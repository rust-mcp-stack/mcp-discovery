use clap::Parser;
use colored::Colorize;
use mcp_discovery::{CommandArguments, DiscoveryCommand, LogLevel, McpDiscovery, PrintOptions};
use tracing_subscriber::{self, EnvFilter};

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
            log_level: args.log_level,
        }));

    let filter = format!(
        "{}={}",
        env!("CARGO_PKG_NAME").to_string().replace("-", "_"),
        command.log_level().as_ref().unwrap_or(&LogLevel::info)
    );

    let tracing_filter = EnvFilter::try_new(filter).unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(tracing_filter)
        .compact()
        .init();

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
