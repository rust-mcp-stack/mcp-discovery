# MCP Discovery

A command-line tool written in Rust for discovering and documenting MCP Server capabilities.

`mcp-discovery` launches an MCP Server using provided commands, queries its capabilities, tools, resources etc.
It supports outputting the results in the terminal or saving them to files in Markdown, HTML, or plain text formats.

🌐 Check out the **rust-mcp-filesystem** [capabilities](https://rust-mcp-stack.github.io/rust-mcp-filesystem/#/capabilities) page for sample output.

This project is open-source and leverages the [rust-mcp-schema](https://github.com/rust-mcp-stack/rust-mcp-schema) and [rust-mcp-sdk](https://github.com/rust-mcp-stack/rust-mcp-sdk) crates for seamless interaction with MCP Servers.

## Features 💡

- **Display MCP Details**: Output MCP Server information, including tools, resources, and capabilities, directly to the terminal.
- **Generate Files**: Create files in Markdown (`.md`), HTML (`.html`), or plain text (`.txt`) formats with MCP Server details and capabilities.
- **Update Files**: Modify existing Markdown, HTML, or text files by adding MCP Server capabilities within specified markers, enabling MCP Server developers to automatically maintain up-to-date documentation and repository README files.
- **Flexible Output Customization**: Choose from built-in templates (`md`, `md-plain`, `html`, `txt`) or supply custom Handlebars templates for personalized output.
- **MCP Discovery GitHub Action**: Integrate the mcp-discovery CLI as a GitHub Action to automate and maintain up-to-date MCP Server documentation in your development workflow.

## Installation ⬇️

### Running as CLI

Refer to the [quickstart](quickstart.md) guide for installation and command line options.

### GitHub Action

The easiest way to automate and maintain up-to-date MCP Server documentation , is to use mcp-discovery as a GitHub action.  
Please see [rust-mcp-stack/mcp-discovery-action](https://github.com/rust-mcp-stack/mcp-discovery-action) for installation and configuration instructions.

## Example

- Print MCP Server capabilities to the terminal:

```sh
mcp-discovery -- npx -y @modelcontextprotocol/server-everything
```

- Running the following command will start the `@modelcontextprotocol/server-everything` server and generate an HTML file listing the available tools and capabilities provided by the example server:

```sh
mcp-discovery create -f server-info.html -- npx -y @modelcontextprotocol/server-everything
```

<b>📄</b> <a href="examples/server-info.html" target="_blank"> Click here to view generated html file</a>
<br/><br/>

?> 💡 See [Example Commands](guide/command-examples.md) for more CLI usage examples across different configurations and scenarios.
