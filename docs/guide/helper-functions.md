# Template Helper Functions

MCP Discovery CLI offers a set of built-in helper functions and Handlebars partials that you can utilize within your templates. These functions and partials are designed to enhance template flexibility and functionality, allowing you to easily format output directly within your templates.

## Helper Functions

### `plus_one`

Increments the given number by 1. This function takes a numeric input and returns the value increased by one.

Example:

```hbs
plus_one 19 is : {{plus_one 19}}
```

Output:

```md
plus_one 19 is : 20
```

### `underline`

Takes a string label and outputs the string followed by an underlining made of "─" characters, with the number of dashes equal to the length of the text.

Example:

```hbs
{{underline 'Hello, World!'}}
```

Output:

```md
Hello, World!
─────────────
```

### `capability`

This helper formats a capability with an optional count and a boolean indicator based on whether the capability is supported.

Example:

```hbs
{{capability 'Feature A' true 5}}
{{capability 'Feature B' false 10}}
```

Output:

```md
🟢 Feature A (5)
🔴 Feature B
```

### `capability_tag`

Similar to `capability` , this helper formats a capability tag depending on whether the capability is supported, adding an optional count and formatting it with an indicator.

> Note: The output of this helper contains HTML tags, so it is intended for use in markdown or HTML files.

Example:

```hbs
{{capability_tag 'Feature A' true 5}}
{{capability_tag 'Feature B' false 0}}
{{capability_tag 'Feature C' true null}}
```

Output:

```md
🟢 Feature A (5)
<span style="opacity:0.6">🔴 Feature B</span>
🟢 Feature C
```

### `capability_title`

This helper will format a title with a count if present, and optionally add an underline based on the with_underline flag.

Example:

```hbs
{{capability_title 'Feature A' 5 true}}

{{capability_title 'Feature B' 10 false}}
```

Output:

```md
Feature A(5)
─────────────

Feature B(10)
```

### `tool_param_type`

This helper converts a tool parameter type (ParamTypes) into a string. Here's how you'd use it in your template:

Example:

```hbs
{{#each tools.1.params}}

  {{tools.0.param_name}}
  :
  {{{tool_param_type this.param_type}}}

{{/each}}
```

Output:

```md
includeImage : boolean  
 messageType : string
```

> The output may vary depending on the MCP server that is launched.

### `replace_regex`

Replaces all occurrences in a string that match a given regular expression with a specified replacement string.

This helper is useful for dynamic text transformations within templates, such as cleaning up labels, formatting values, or removing unwanted prefixes.

Example:

_Wraps words enclosed in single quotes with `<code>` and `</code>` tags._

```hbs
{{{replace_regex
  (replace_regex
    "Each entry includes 'name', 'type' ." "'([\\w\\-\\_]+)'" '<code>$1</code>'
  )
}}}
```

Output:

```md
Each entry includes <code>name</code>, <code>type</code> .
```

### `json_helper`

A Handlebars helper that serializes the entire template context into a JSON string. It supports optional pretty-printing when a truthy argument is passed.

Example:

_Wraps words enclosed in single quotes with `<code>` and `</code>` tags._

```hbs
{{json true}}
```

<a href="examples/json.txt" target="_blank"> 📎 output json</a>
