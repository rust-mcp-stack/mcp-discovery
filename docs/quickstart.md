# Quick Start

## Install

<!-- tabs:start -->

#### **Shell script**

<!-- x-release-please-start-version -->

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/rust-mcp-stack/mcp-discovery/releases/download/v0.2.4/mcp-discovery-installer.sh | sh
```

#### **PowerShell script**

```sh
powershell -ExecutionPolicy Bypass -c "irm https://github.com/rust-mcp-stack/mcp-discovery/releases/download/v0.2.4/mcp-discovery-installer.ps1 | iex"
```

<!-- x-release-please-end -->

#### **Homebrew**

```sh
brew install rust-mcp-stack/tap/mcp-discovery
```

#### **Cargo**

```sh
cargo install mcp-discovery --locked
```

#### **NPM**

```sh
npm i -g @rustmcp/mcp-discovery@latest
```

#### **Download Binaries**

<table>
  <thead>
    <tr>
      <th>File</th>
      <th>Platform</th>
      <th>Checksum</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>
      <!-- x-release-please-start-version -->
      <a href="https://github.com/rust-mcp-stack/mcp-discovery/releases/download/v0.2.4/mcp-discovery-aarch64-apple-darwin.tar.gz">mcp-discovery-aarch64-apple-darwin.tar.gz</a>
      <!-- x-release-please-end -->
      </td>
      <td>Apple Silicon macOS</td>
      <td>
      <!-- x-release-please-start-version -->
      <a href="https://github.com/rust-mcp-stack/mcp-discovery/releases/download/v0.2.4/mcp-discovery-aarch64-apple-darwin.tar.gz.sha256">checksum</a>
      <!-- x-release-please-end -->    
      </td>
    </tr>
    <tr>
      <td>
      <!-- x-release-please-start-version -->
      <a href="https://github.com/rust-mcp-stack/mcp-discovery/releases/download/v0.2.4/mcp-discovery-x86_64-apple-darwin.tar.gz">mcp-discovery-x86_64-apple-darwin.tar.gz</a>
      <!-- x-release-please-end -->
      </td>
      <td>Intel macOS</td>
      <td>
      <!-- x-release-please-start-version -->
      <a href="https://github.com/rust-mcp-stack/mcp-discovery/releases/download/v0.2.4/mcp-discovery-x86_64-apple-darwin.tar.gz.sha256">checksum</a>
      <!-- x-release-please-end -->
      </td>
    </tr>
    <tr>
      <td>
      <!-- x-release-please-start-version -->
      <a href="https://github.com/rust-mcp-stack/mcp-discovery/releases/download/v0.2.4/mcp-discovery-x86_64-pc-windows-msvc.zip">mcp-discovery-x86_64-pc-windows-msvc.zip</a>
      <!-- x-release-please-end -->
      </td>
      <td>x64 Windows (zip)</td>
      <td>
      <!-- x-release-please-start-version -->
      <a href="https://github.com/rust-mcp-stack/mcp-discovery/releases/download/v0.2.4/mcp-discovery-x86_64-pc-windows-msvc.zip.sha256">checksum</a>
      <!-- x-release-please-end -->
      </td>
    </tr>
    <tr>
      <td>
      <!-- x-release-please-start-version -->
      <a href="https://github.com/rust-mcp-stack/mcp-discovery/releases/download/v0.2.4/mcp-discovery-x86_64-pc-windows-msvc.msi">mcp-discovery-x86_64-pc-windows-msvc.msi</a>
      <!-- x-release-please-end -->
      </td>
      <td>x64 Windows (msi)</td>
      <td>
      <!-- x-release-please-start-version -->
      <a href="https://github.com/rust-mcp-stack/mcp-discovery/releases/download/v0.2.4/mcp-discovery-x86_64-pc-windows-msvc.msi.sha256">checksum</a>
      <!-- x-release-please-end -->
      </td>
    </tr>
    <tr>
      <td>
      <!-- x-release-please-start-version -->
      <a href="https://github.com/rust-mcp-stack/mcp-discovery/releases/download/v0.2.4/mcp-discovery-aarch64-unknown-linux-gnu.tar.gz">mcp-discovery-aarch64-unknown-linux-gnu.tar.gz</a>
      <!-- x-release-please-end -->
      </td>
      <td>ARM64 Linux</td>
      <td>
      <!-- x-release-please-start-version -->
      <a href="https://github.com/rust-mcp-stack/mcp-discovery/releases/download/v0.2.4/mcp-discovery-aarch64-unknown-linux-gnu.tar.gz.sha256">checksum</a>
      <!-- x-release-please-end -->
      </td>
    </tr>
    <tr>
      <td>
      <!-- x-release-please-start-version -->
      <a href="https://github.com/rust-mcp-stack/mcp-discovery/releases/download/v0.2.4/mcp-discovery-x86_64-unknown-linux-gnu.tar.gz">mcp-discovery-x86_64-unknown-linux-gnu.tar.gz</a>
      <!-- x-release-please-end -->
      </td>
      <td>x64 Linux</td>
      <td>
      <!-- x-release-please-start-version -->
      <a href="https://github.com/rust-mcp-stack/mcp-discovery/releases/download/v0.2.4/mcp-discovery-x86_64-unknown-linux-gnu.tar.gz.sha256">checksum</a>
      <!-- x-release-please-end -->
      </td>
    </tr>
  </tbody>
</table>

<!-- tabs:end -->

### Usage

```sh
Usage: mcp-discovery [OPTIONS] -- <MCP Launch Command>...
       mcp-discovery <COMMAND>

Commands:
  print   Displays MCP server capability details in the terminal
  create  Creates a file with MCP server capability details
  update  Updates a file by adding MCP server capability information between specified markers
  help    Print this message or the help of the given subcommand(s)

Arguments:
  <MCP Launch Command>...  Command and arguments to launch the MCP server

Options:
  -t, --template <TEMPLATE>
          Select an output template from the built-in options [possible values: md, md-plain, html, txt]
  -p, --template-file <TEMPLATE_FILE>
          Path to a custom template file written in the Handlebars format
  -s, --template-string <TEMPLATE_STRING>
          Template content provided as a string
  -h, --help
          Print help
  -V, --version
          Print version
```

### Example

Print MCP Server capabilities to the terminal:

```sh
mcp-discovery -- npx -y @modelcontextprotocol/server-everything
```

## Subcommands

- **`print`**: Displays MCP Server capabilities in the terminal.
- **`create`**: Creates a new file with MCP Server capability details.
- **`update`**: Updates an existing file by inserting MCP Server capabilities between specified
  markers.

👉 Note: If no subcommand is provided, the `print` subcommand will be used by default.

### Options ⚙️

- `-f, --filename <FILENAME>`: Used with `create` and `update` commands to specify the output file to generate or modify.
- `-t, --template <TEMPLATE>`: Choose a built-in output template. Options: `md`, `md-plain`, `html`, `txt`.
- `-p, --template-file <TEMPLATE_FILE>`: Path to a custom Handlebars template file.
- `-s, --template-string <TEMPLATE_STRING>`: Inline Handlebars template provided as a string.
- `-h, --help`: Display help information.
- `-V, --version`: Display the version of `mcp-discovery`.

👉 Note: If no template is provided, `mcp-discovery` will automatically select the most suitable built-in template based on the file extension.

## Built-in Templates 🧬

The CLI supports the following built-in output templates:

- **`md`**: Formatted Markdown that presents MCP Server capabilities in a table format.
- **`md-plain`**: Minimalist Markdown for straightforward output, using plain text instead of tables.
- **`html`**: Structured HTML with basic styling.
- **`txt`**: Plain text for raw, unformatted output.

## Custom Templates 🧩

You can provide custom Handlebars templates in different ways:

1.  Use the `--template-file` flag to provide a custom template file.
2.  Use the `--template-string` flag to provide a raw Handlebars template directly as a string.
3.  To use an inline template, define it in a file for the `update` command only — <i>this will not function with print or create.</i>

> Inline templates must be enclosed within designated marker annotations.

?> 💡 See [Example Commands](guide/command-examples.md) for CLI usage examples across different configurations and scenarios.
