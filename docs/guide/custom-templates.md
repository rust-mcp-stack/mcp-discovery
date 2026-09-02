# Custom Templates

`mcp-discovery` renders MCP Server capabilities with Handlebars templates. Besides the built-in
templates (`md`, `md-plain`, `html`, `txt`), you can supply your own template and bring **your own
partials** alongside it.

## Template sources

| Flag | Source | Own partials? |
| --- | --- | --- |
| `--template <name>` | Built-in template (`md`, `md-plain`, `html`, `txt`) | ✘ (uses built-in partials) |
| `--template-file <path>` | A single `.hbs` **file** or a **folder** | ✔ if a sibling `partials/` directory exists |
| `--template-string <s>` | Inline Handlebars string | ✔ inline (`{{#*inline}}`) |
| `--template-url <url>` | Remote template (`.hbs`, `.zip`, or `.tar.gz`) | ✔ for archives with a `partials/` directory |

The flags are mutually exclusive.

## `--template-file`: a file or a folder

`--template-file` accepts a Handlebars template in two forms:

**1. A single file:**

```bash
mcp-discovery print --template-file ./report.hbs -- npx -y @modelcontextprotocol/server-everything
```

**2. A folder.** When you point it at a directory, `mcp-discovery` uses `template.hbs` inside it, if
that file does not exist, it falls back to the single standalone `.hbs` file in the folder root.
An error is raised when the folder has no `template.hbs` and zero or several `.hbs` files.

```
my-report/
  template.hbs        # entry template (or the only .hbs in the folder)
  partials/           # optional
    header.hbs
    footer.hbs
```

```bash
mcp-discovery create -f capabilities.md --template-file ./my-report -- npx -y @modelcontextprotocol/server-everything
```

## User-defined partials

Any `.hbs` file found in the `partials/` directory **next to your template** is registered
automatically. Partials are addressed by their path **relative to the template base**, without the
`.hbs` extension, so nested directories are preserved:

```
partials/header.hbs        →  {{> partials/header }}
partials/tables/tools.hbs  →  {{> partials/tables/tools }}
```

Example entry template that uses its own partials plus built-in ones:

```handlebars
{{> title-version prefix="## " }}

{{> partials/intro }}

## Tools

{{#each tools}}
- {{name}}: {{description}}
{{/each}}
```

Because custom partials are namespaced under `partials/`, they never collide with the built-in
partials (such as `title-version`, `md-tools`, `html-tools`, ...). The built-in partials and
[helper functions](./helper-functions.md) remain available to your templates.

> `--template-string` templates are a bare string with no `partials/` directory to read from.
> Define partials inline with Handlebars' `{{#*inline "name"}}...{{/inline}}` syntax instead — see
> below.

## `--template-string` with inline partials

Because a `--template-string` has no directory of its own, you can define partials inline using
Handlebars' `{{#*inline "name"}}...{{/inline}}` syntax and reference them with `{{> name}}`:

```bash
mcp-discovery print \
  --template-string '{{#*inline "title"}}## {{name}} {{version}}{{/inline}}{{> title}}

{{#each tools}}- {{name}}: {{description}}
{{/each}}' \
  -- npx -y @modelcontextprotocol/server-everything
```

Inline partials are scoped to the template string they are declared in. The built-in partials
(`title-version`, `md-tools`, ...) and [helper functions](./helper-functions.md) remain available
to `--template-string` templates.

## Markers (`update` command)

The `update` command supports the same partials resolution through the `template-file=` render
block property, e.g.:

```
<!-- mcp-discovery-render template-file=my-report -->

<!-- mcp-discovery-render-end -->
```

Both a file path and a folder path are accepted there.

## Remote templates (`--template-url`)

`--template-url` renders a template fetched over HTTPS from a URL. It accepts either a single
Handlebars file (`.hbs`) or an archive (`.zip` / `.tar.gz`) that contains a template — the same
`template.hbs` + `partials/` layout as local templates. Any `partials/` directory inside an
extracted archive is registered automatically.

```bash
# single remote .hbs
mcp-discovery print --template-url https://example.com/report.hbs -- npx -y @modelcontextprotocol/server-everything

# remote archive that contains template.hbs (and optional partials/)
mcp-discovery create -f capabilities.md \
  --template-url https://example.com/my-template.tar.gz \
  -- npx -y @modelcontextprotocol/server-everything
```

`--template-url` is independent of the transport `--url` flag: it only decides *how the output is
formatted*, so it works with both stdio servers and streamable HTTP servers.

### URL fragments

Fragments in the URL are client-side directives (never sent to the server):

| Fragment | Meaning |
| --- | --- |
| `#sha256=<hex>` | Integrity pin. The downloaded content must hash to this value or rendering is refused. |
| `#entry=<subpath>` | For archives, select a specific file inside the archive instead of `template.hbs`. |

```bash
mcp-discovery print \
  --template-url "https://example.com/pack.zip#sha256=3f9a...&entry=report.hbs" \
  -- npx -y @modelcontextprotocol/server-everything
```

A raw single-file `.hbs` has no directory to read partials from, so it can only use the built-in
partials. Use an archive (or `--template-file`) when you need custom partials.

### Caching and `--cache-dir`

Fetched templates are cached and reused. The cache lives under the OS cache directory
(`~/Library/Caches/mcp-discovery` on macOS, `~/.cache/mcp-discovery` on Linux) unless you point it
elsewhere:

```bash
mcp-discovery print --template-url https://example.com/report.hbs \
  --cache-dir ./tmp/template-cache -- npx -y @modelcontextprotocol/server-everything
```

`--cache-dir` is created if it does not exist. Each cached template carries a `.source.json`
provenance file (source URL, content SHA-256, fetch time, tool version), so it is clear what the
cache holds and where it came from. Cached templates are re-fetched when the `#sha256=` pin does
not match the cached content.

### Security notes

- Only `https://` URLs are accepted.
- Downloaded content and decompressed archives are size-limited.
- Archive extraction rejects entries that attempt path traversal (`../`) or absolute paths, and
  skips symlinks.
- Handlebars templates are inert: they render the discovered MCP Server metadata and cannot access
  the filesystem, environment, or network.
