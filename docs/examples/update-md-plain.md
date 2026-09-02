# Server Info and Capabilities

<!-- mcp-discovery-render template=md-plain -->
## mcp-servers/everything 2.0.0

| ✔ Tools (19) | ✔ Prompts (4) | ✔ Resources (7) | ✔ Logging | ✔ Completions | ~~<span style="opacity:0.6" class="error">✘ Tasks</span>~~ |
| --- | --- | --- | --- | --- | --- |

## 🛠️ Tools (19)


- **echo**
  - Echoes back the input string
  - **Inputs:**
      - <code>message</code> : string<br />

- **get-annotated-message**
  - Demonstrates how annotations can be used to provide metadata about content.
  - **Inputs:**
      - <code>includeImage</code> : boolean<br />
      - <code>messageType</code> : error|success|debug<br />

- **get-env**
  - Returns all environment variables, helpful for debugging MCP server configuration

- **get-resource-links**
  - Returns up to ten resource links that reference different types of resources
  - **Inputs:**
      - <code>count</code> : number<br />

- **get-resource-reference**
  - Returns a resource reference that can be used by MCP clients
  - **Inputs:**
      - <code>resourceId</code> : number<br />
      - <code>resourceType</code> : Text|Blob<br />

- **get-roots-list**
  - Lists the current MCP roots provided by the client. Demonstrates the roots protocol capability even though this server doesn't access files.

- **get-structured-content**
  - Returns structured content along with an output schema for client data validation
  - **Inputs:**
      - <code>location</code> : New York|Chicago|Los Angeles<br />

- **get-sum**
  - Returns the sum of two numbers
  - **Inputs:**
      - <code>a</code> : number<br />
      - <code>b</code> : number<br />

- **get-tiny-image**
  - Returns a tiny MCP logo image.

- **gzip-file-as-resource**
  - Compresses a single file using gzip compression. Depending upon the selected output type, returns either the compressed data as a gzipped resource or a resource link, allowing it to be downloaded in a subsequent request during the current session.
  - **Inputs:**
      - <code>data</code> : string<br />
      - <code>name</code> : string<br />
      - <code>outputType</code> : resourceLink|resource<br />

- **simulate-research-query**
  - Simulates a deep research operation that gathers, analyzes, and synthesizes information. Demonstrates MCP task-based operations with progress through multiple stages. If <code>ambiguous</code> is true and client supports elicitation, sends an elicitation request for clarification.
  - **Inputs:**
      - <code>ambiguous</code> : boolean<br />
      - <code>topic</code> : string<br />

- **toggle-simulated-logging**
  - Toggles simulated, random-leveled logging on or off.

- **toggle-subscriber-updates**
  - Toggles simulated resource subscription updates on or off.

- **trigger-elicitation-request**
  - Trigger a Request from the Server for User Elicitation

- **trigger-elicitation-request-async**
  - Trigger an async elicitation request that the CLIENT executes as a background task. Demonstrates bidirectional MCP tasks where the server sends an elicitation request and the client handles user input asynchronously, allowing the server to poll for completion.

- **trigger-long-running-operation**
  - Demonstrates a long running operation with progress updates.
  - **Inputs:**
      - <code>duration</code> : number<br />
      - <code>steps</code> : number<br />

- **trigger-sampling-request**
  - Trigger a Request from the Server for LLM Sampling
  - **Inputs:**
      - <code>maxTokens</code> : number<br />
      - <code>prompt</code> : string<br />

- **trigger-sampling-request-async**
  - Trigger an async sampling request that the CLIENT executes as a background task. Demonstrates bidirectional MCP tasks where the server sends a request and the client executes it asynchronously, allowing the server to poll for progress and results.
  - **Inputs:**
      - <code>maxTokens</code> : number<br />
      - <code>prompt</code> : string<br />

- **trigger-url-elicitation**
  - Trigger a URL elicitation so the client can direct the user to a browser flow. Supports two mechanisms: the request path (elicitation/create, default) which awaits the user's response, and the error path (UrlElicitationRequiredError, -32042) which signals the client to handle URL elicitation via the error response. Set errorPath=true to use the error path.
  - **Inputs:**
      - <code>elicitationId</code> : string<br />
      - <code>errorPath</code> : boolean<br />
      - <code>message</code> : string<br />
      - <code>url</code> : string<br />


## 📝 Prompts (4)


- **simple-prompt**
  - A prompt with no arguments

- **args-prompt**
  - A prompt with two arguments, one required and one optional

- **completable-prompt**
  - First argument choice narrows values for second argument.

- **resource-prompt**
  - A prompt that includes an embedded resource reference

## 📄 Resources (7)


- **architecture.md**
  - Static document file exposed from /docs: architecture.md
  - URI: <a>demo://resource/static/document/architecture.md</a> <i>(text/markdown)</i>

- **extension.md**
  - Static document file exposed from /docs: extension.md
  - URI: <a>demo://resource/static/document/extension.md</a> <i>(text/markdown)</i>

- **features.md**
  - Static document file exposed from /docs: features.md
  - URI: <a>demo://resource/static/document/features.md</a> <i>(text/markdown)</i>

- **how-it-works.md**
  - Static document file exposed from /docs: how-it-works.md
  - URI: <a>demo://resource/static/document/how-it-works.md</a> <i>(text/markdown)</i>

- **instructions.md**
  - Static document file exposed from /docs: instructions.md
  - URI: <a>demo://resource/static/document/instructions.md</a> <i>(text/markdown)</i>

- **startup.md**
  - Static document file exposed from /docs: startup.md
  - URI: <a>demo://resource/static/document/startup.md</a> <i>(text/markdown)</i>

- **structure.md**
  - Static document file exposed from /docs: structure.md
  - URI: <a>demo://resource/static/document/structure.md</a> <i>(text/markdown)</i>

## 🧩 Resource Templates (2)


- **Dynamic Text Resource**
  - Plaintext dynamic resource fabricated from the {resourceId} variable, which must be an integer.

- **Dynamic Blob Resource**
  - Binary (base64) dynamic resource fabricated from the {resourceId} variable, which must be an integer.

<sup>◾ generated by [mcp-discovery](https://github.com/rust-mcp-stack/mcp-discovery)</sup>
<!-- mcp-discovery-render-end -->