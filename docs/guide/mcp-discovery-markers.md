# Defining Update Regions with Markers

When using the `update` subcommand, `mcp-discovery` places capabilities between designated markers in the target file, which vary by file format and are typically comment lines.

The update command simplifies the process for developers and maintainers to keep their MCP Server documentation current effortlessly.

You can run the mcp-discovery update command anytime to refresh the file with the latest MCP Server capabilities.

### Marker Annotations

- **Render Block Start** : **`mcp-discovery-render`**
- **Render Block End** : **`mcp-discovery-render-end`**

**👉** The mcp-discovery-render marker supports template and template-file properties as well. Check the examples below for details.

You can optionally include an inline template identifier within the render block, enclosed by:

- **Template Block Start**: **`mcp-discovery-template`**
- **Template Block End**: **`mcp-discovery-template-end`**

If a template annotation is detected within a render block, `mcp-discovery` will use it to render the output. This allows for customized templates without depending on built-in or external template files. Check the examples below for details:

### Sample Markdown file annotated with render block:

```md
# Server Info and Capabilities

<!-- mcp-discovery-render -->

Server Capabilities will be placed here...

<!-- mcp-discovery-render-end -->
```

### Sample Markdown file, annotated with render block and template name:

```md
# Server Info and Capabilities

<!-- mcp-discovery-render template=md-plain -->

Server Capabilities will be placed here...

<!-- mcp-discovery-render-end -->
```

### Sample Markdown file, annotated with render block and custom template file:

```md
# Server Info and Capabilities

<!-- mcp-discovery-render template=my-custom-template.hbs -->

Server Capabilities will be placed here...

<!-- mcp-discovery-render-end -->
```

### Sample HTML file with annotations :

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <title>My MCP Server</title>
  </head>
  <body>
    <h1>MCP Server Details</h1>
    <div>
      <!-- mcp-discovery-render -->

      <!-- mcp-discovery-render-end -->
    </div>
  </body>
</html>
```

### Sample HTML file with inline template :

```html
<h1>MCP Server Details</h1>
<div>
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
</div>
```

Below is a screenshot showing the resulting HTML after the mcp-discovery update command is executed:

<img src="./\_media/example-html-inline.jpg" alt="MCP Discovery HTML Inline Template" width="600" style="border: solid 1px #e4e4e4;">

> You can execute the mcp-discovery update command whenever you need to refresh the file with the latest MCP Server capabilities.
