# Command Examples

Below is a list of sample commands showcasing various options available in the mcp-discovery CLI.

_These examples use the `@modelcontextprotocol/server-everything` example server for demonstration._

## Print

#### ▪️ Print MCP Server capabilities to the terminal:

```bash
mcp-discovery -- npx -y @modelcontextprotocol/server-everything
```

<a href="examples/print_terminal.jpg" target="_blank"> 📎 view the output</a>

---

#### ▪️ Print MCP Server capabilities to the terminal, with a custom template:

```bash
mcp-discovery --template-file sample_txt_template.txt -- npx -y @modelcontextprotocol/server-everything
```

<a href="examples/print_terminal_template_file.jpg" target="_blank"> 📎 view the output</a> |
<a href="example_template/sample_txt_template.txt" target="_blank"> 📎 sample_txt_template.txt</a>

---

#### ▪️ Print MCP Server capabilities to the terminal, with a template string:

```bash
mcp-discovery --template-string "Server Name: {{name}}, Server Version: {{version}}" -- npx -y @modelcontextprotocol/server-everything
```

**Output:**

```
Server Name: example-servers/everything, Server Version: 1.0.0
```

---

#### ▪️ Print MCP Server capabilities to the terminal as JSON:

We use the `json` helper function as a template string. It accepts an optional boolean argument—when set to true, the output is pretty-printed:

```bash
mcp-discovery --template-string "{{{json true}}}" -- npx -y @modelcontextprotocol/server-everything
```

<a href="examples/json.txt" target="_blank"> 📎 printed json</a>

---

## Create

#### Create a Markdown file (\*.md) with MCP Server capabilities:

```bash
mcp-discovery create -f create-md.md -- npx -y @modelcontextprotocol/server-everything
```

<a href="https://github.com/rust-mcp-stack/mcp-discovery/blob/main/docs/examples/create-md.md#example-serverseverything-100" target="_blank"> 📎 view generated file</a>

---

#### Create a Markdown file (\*.md) using `md-plain` template:

`md-plain` generates plain markdown using text lists rather than tables.

```bash
mcp-discovery create -f create-md-plain.md --template md-plain -- npx -y @modelcontextprotocol/server-everything
```

<a href="https://github.com/rust-mcp-stack/mcp-discovery/blob/main/docs/examples/create-md-plain.md#example-serverseverything-100" target="_blank"> 📎 view generated file</a>

---

#### Create a HTML file (\*.html) with MCP Server capabilities:

```bash
mcp-discovery create -f server-info.html -- npx -y @modelcontextprotocol/server-everything
```

<a href="examples/server-info.html" target="_blank">📎 view generated file</a>

## Update

To update files, you need to annotate a [render block](./guide/mcp-discovery-markers.md) within the target file where the MCP server capabilities should be inserted.
Refer to the ["Update Regions with Markers"](./guide/mcp-discovery-markers.md) page for details on how to define render blocks and optional inline template sections.

#### Update a Markdown file with MCP Server capabilities:

```bash
mcp-discovery update -f update-md.md -- npx -y @modelcontextprotocol/server-everything
```

Below is a typical md file containing a render block marked with `<!-- mcp-discovery-render -->` and `<!-- mcp-discovery-render-end -->`.  
MCP Discovery will overwrite the content between these markers with the latest generated output.

```md
# Server Info and Capabilities

<!-- mcp-discovery-render -->
<!-- mcp-discovery-render-end -->
```

<a href="https://github.com/rust-mcp-stack/mcp-discovery/blob/main/docs/examples/update-md.md#example-serverseverything-100" target="_blank"> 📎 view updated file</a>

---

#### Update a Markdown file using `md-plain` template:

We can either pass the --template md-plain argument to the CLI or specify md-plain as a property on the <!-- mcp-discovery-render --> line.

```bash
mcp-discovery update -f update-md-plain.md -- npx -y @modelcontextprotocol/server-everything
```

Below is a md file containing a render block with `template` property set to `md-plain`:

```md
# Server Info and Capabilities

<!-- mcp-discovery-render template=md-plain -->
<!-- mcp-discovery-render-end -->
```

<a href="https://github.com/rust-mcp-stack/mcp-discovery/blob/main/docs/examples/update-md-plain.md#example-serverseverything-100" target="_blank"> 📎 view updated file</a>

---

#### Update a Markdown file with an inline template:

Inline templates should appear between a render block. Refer to the ["Update Regions with Markers"](./guide/mcp-discovery-markers.md) page for details on how to define render blocks and optional inline template sections.

```bash
mcp-discovery update -f update-md-inline.md -- npx -y @modelcontextprotocol/server-everything
```

Below is an md file with a `render block` and an `inline template`. The `inline template` will be used and preserved when the `mcp-discovery` CLI updates the file.

```md
# Server Info and Capabilities

<!-- mcp-discovery-render -->
<!-- mcp-discovery-template
    <b>Name: </b>{{name}}
    <br/>
    <b>Version: </b>{{version}}
    <br/>
    <b>Number of tools:</b> {{len tools}}
    <h2>Summary:</h2>
    {{> html-summary }}
    mcp-discovery-template-end -->
<!-- mcp-discovery-render-end -->

---

A Footer
```

<a href="https://github.com/rust-mcp-stack/mcp-discovery/blob/main/docs/examples/update-md-inline.md#example-serverseverything-100" target="_blank"> 📎 view updated file</a>
