## mcp-servers/everything 2.0.0

| ✔ Tools (19) | ✔ Prompts (4) | ✔ Resources (7) | ✔ Logging | ✔ Completions | ~~<span style="opacity:0.6" class="error">✘ Tasks</span>~~ |
| --- | --- | --- | --- | --- | --- |

## 🛠️ Tools (19)

<table style="text-align: left;">
<thead>
    <tr>
        <th style="width: auto;"></th>
        <th style="width: auto;">Icon</th>
        <th style="width: auto;">Tool Name</th>
        <th style="width: auto;">Description</th>
        <th style="width: auto;">Inputs</th>
    </tr>
</thead>
<tbody style="vertical-align: top;">
        <tr>
            <td>1.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>echo</b></code>
            </td>
            <td>Echoes back the input string</td>
            <td>
                <ul>
                    <li> <code>message</code> : string<br /></li>
                </ul>
            </td>
        </tr>
        <tr>
            <td>2.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>get-annotated-message</b></code>
            </td>
            <td>Demonstrates how annotations can be used to provide metadata about content.</td>
            <td>
                <ul>
                    <li> <code>includeImage</code> : boolean<br /></li>
                    <li> <code>messageType</code> : error|success|debug<br /></li>
                </ul>
            </td>
        </tr>
        <tr>
            <td>3.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>get-env</b></code>
            </td>
            <td>Returns all environment variables, helpful for debugging MCP server configuration</td>
            <td>
                <ul>
                </ul>
            </td>
        </tr>
        <tr>
            <td>4.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>get-resource-links</b></code>
            </td>
            <td>Returns up to ten resource links that reference different types of resources</td>
            <td>
                <ul>
                    <li> <code>count</code> : number<br /></li>
                </ul>
            </td>
        </tr>
        <tr>
            <td>5.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>get-resource-reference</b></code>
            </td>
            <td>Returns a resource reference that can be used by MCP clients</td>
            <td>
                <ul>
                    <li> <code>resourceId</code> : number<br /></li>
                    <li> <code>resourceType</code> : Text|Blob<br /></li>
                </ul>
            </td>
        </tr>
        <tr>
            <td>6.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>get-roots-list</b></code>
            </td>
            <td>Lists the current MCP roots provided by the client. Demonstrates the roots protocol capability even though this server doesn't access files.</td>
            <td>
                <ul>
                </ul>
            </td>
        </tr>
        <tr>
            <td>7.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>get-structured-content</b></code>
            </td>
            <td>Returns structured content along with an output schema for client data validation</td>
            <td>
                <ul>
                    <li> <code>location</code> : New York|Chicago|Los Angeles<br /></li>
                </ul>
            </td>
        </tr>
        <tr>
            <td>8.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>get-sum</b></code>
            </td>
            <td>Returns the sum of two numbers</td>
            <td>
                <ul>
                    <li> <code>a</code> : number<br /></li>
                    <li> <code>b</code> : number<br /></li>
                </ul>
            </td>
        </tr>
        <tr>
            <td>9.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>get-tiny-image</b></code>
            </td>
            <td>Returns a tiny MCP logo image.</td>
            <td>
                <ul>
                </ul>
            </td>
        </tr>
        <tr>
            <td>10.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>gzip-file-as-resource</b></code>
            </td>
            <td>Compresses a single file using gzip compression. Depending upon the selected output type, returns either the compressed data as a gzipped resource or a resource link, allowing it to be downloaded in a subsequent request during the current session.</td>
            <td>
                <ul>
                    <li> <code>data</code> : string<br /></li>
                    <li> <code>name</code> : string<br /></li>
                    <li> <code>outputType</code> : resourceLink|resource<br /></li>
                </ul>
            </td>
        </tr>
        <tr>
            <td>11.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>simulate-research-query</b></code>
            </td>
            <td>Simulates a deep research operation that gathers, analyzes, and synthesizes information. Demonstrates MCP task-based operations with progress through multiple stages. If <code>ambiguous</code> is true and client supports elicitation, sends an elicitation request for clarification.</td>
            <td>
                <ul>
                    <li> <code>ambiguous</code> : boolean<br /></li>
                    <li> <code>topic</code> : string<br /></li>
                </ul>
            </td>
        </tr>
        <tr>
            <td>12.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>toggle-simulated-logging</b></code>
            </td>
            <td>Toggles simulated, random-leveled logging on or off.</td>
            <td>
                <ul>
                </ul>
            </td>
        </tr>
        <tr>
            <td>13.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>toggle-subscriber-updates</b></code>
            </td>
            <td>Toggles simulated resource subscription updates on or off.</td>
            <td>
                <ul>
                </ul>
            </td>
        </tr>
        <tr>
            <td>14.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>trigger-elicitation-request</b></code>
            </td>
            <td>Trigger a Request from the Server for User Elicitation</td>
            <td>
                <ul>
                </ul>
            </td>
        </tr>
        <tr>
            <td>15.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>trigger-elicitation-request-async</b></code>
            </td>
            <td>Trigger an async elicitation request that the CLIENT executes as a background task. Demonstrates bidirectional MCP tasks where the server sends an elicitation request and the client handles user input asynchronously, allowing the server to poll for completion.</td>
            <td>
                <ul>
                </ul>
            </td>
        </tr>
        <tr>
            <td>16.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>trigger-long-running-operation</b></code>
            </td>
            <td>Demonstrates a long running operation with progress updates.</td>
            <td>
                <ul>
                    <li> <code>duration</code> : number<br /></li>
                    <li> <code>steps</code> : number<br /></li>
                </ul>
            </td>
        </tr>
        <tr>
            <td>17.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>trigger-sampling-request</b></code>
            </td>
            <td>Trigger a Request from the Server for LLM Sampling</td>
            <td>
                <ul>
                    <li> <code>maxTokens</code> : number<br /></li>
                    <li> <code>prompt</code> : string<br /></li>
                </ul>
            </td>
        </tr>
        <tr>
            <td>18.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>trigger-sampling-request-async</b></code>
            </td>
            <td>Trigger an async sampling request that the CLIENT executes as a background task. Demonstrates bidirectional MCP tasks where the server sends a request and the client executes it asynchronously, allowing the server to poll for progress and results.</td>
            <td>
                <ul>
                    <li> <code>maxTokens</code> : number<br /></li>
                    <li> <code>prompt</code> : string<br /></li>
                </ul>
            </td>
        </tr>
        <tr>
            <td>19.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>trigger-url-elicitation</b></code>
            </td>
            <td>Trigger a URL elicitation so the client can direct the user to a browser flow. Supports two mechanisms: the request path (elicitation/create, default) which awaits the user's response, and the error path (UrlElicitationRequiredError, -32042) which signals the client to handle URL elicitation via the error response. Set errorPath=true to use the error path.</td>
            <td>
                <ul>
                    <li> <code>elicitationId</code> : string<br /></li>
                    <li> <code>errorPath</code> : boolean<br /></li>
                    <li> <code>message</code> : string<br /></li>
                    <li> <code>url</code> : string<br /></li>
                </ul>
            </td>
        </tr>
</tbody>
</table>

## 📝 Prompts (4)

<table style="text-align: left;">
<thead>
    <tr>
        <th style="width: auto;"></th>
        <th style="width: auto;">Prompt Name</th>
        <th style="width: auto;">Description</th>
    </tr>
</thead>
<tbody style="vertical-align: top;">
        <tr>
            <td>1.</td>
            <td>
                <code><b>simple-prompt</b></code>
            </td>
            <td>A prompt with no arguments</td>
        </tr>
        <tr>
            <td>2.</td>
            <td>
                <code><b>args-prompt</b></code>
            </td>
            <td>A prompt with two arguments, one required and one optional</td>
        </tr>
        <tr>
            <td>3.</td>
            <td>
                <code><b>completable-prompt</b></code>
            </td>
            <td>First argument choice narrows values for second argument.</td>
        </tr>
        <tr>
            <td>4.</td>
            <td>
                <code><b>resource-prompt</b></code>
            </td>
            <td>A prompt that includes an embedded resource reference</td>
        </tr>
</tbody>
</table>

## 📄 Resources (7)

<table style="text-align: left;">
<thead>
    <tr>
        <th style="width: auto;"></th>
        <th style="width: auto;">Icon</th>
        <th style="width: auto;">Resource Name</th>
        <th style="width: auto;">Uri</th>
        <th style="width: auto;">Description</th>
    </tr>
</thead>
<tbody style="vertical-align: top;">
        <tr>
            <td>1.</td>
            <td>
              <!--- no icon -->
            </td>
            <td>
                <code><b>architecture.md</b></code>
            </td>
            <td>
                <a>demo://resource/static/document/architecture.md</a> <i>(text/markdown)</i>
            </td>
            <td>Static document file exposed from /docs: architecture.md</td>
        </tr>
        <tr>
            <td>2.</td>
            <td>
              <!--- no icon -->
            </td>
            <td>
                <code><b>extension.md</b></code>
            </td>
            <td>
                <a>demo://resource/static/document/extension.md</a> <i>(text/markdown)</i>
            </td>
            <td>Static document file exposed from /docs: extension.md</td>
        </tr>
        <tr>
            <td>3.</td>
            <td>
              <!--- no icon -->
            </td>
            <td>
                <code><b>features.md</b></code>
            </td>
            <td>
                <a>demo://resource/static/document/features.md</a> <i>(text/markdown)</i>
            </td>
            <td>Static document file exposed from /docs: features.md</td>
        </tr>
        <tr>
            <td>4.</td>
            <td>
              <!--- no icon -->
            </td>
            <td>
                <code><b>how-it-works.md</b></code>
            </td>
            <td>
                <a>demo://resource/static/document/how-it-works.md</a> <i>(text/markdown)</i>
            </td>
            <td>Static document file exposed from /docs: how-it-works.md</td>
        </tr>
        <tr>
            <td>5.</td>
            <td>
              <!--- no icon -->
            </td>
            <td>
                <code><b>instructions.md</b></code>
            </td>
            <td>
                <a>demo://resource/static/document/instructions.md</a> <i>(text/markdown)</i>
            </td>
            <td>Static document file exposed from /docs: instructions.md</td>
        </tr>
        <tr>
            <td>6.</td>
            <td>
              <!--- no icon -->
            </td>
            <td>
                <code><b>startup.md</b></code>
            </td>
            <td>
                <a>demo://resource/static/document/startup.md</a> <i>(text/markdown)</i>
            </td>
            <td>Static document file exposed from /docs: startup.md</td>
        </tr>
        <tr>
            <td>7.</td>
            <td>
              <!--- no icon -->
            </td>
            <td>
                <code><b>structure.md</b></code>
            </td>
            <td>
                <a>demo://resource/static/document/structure.md</a> <i>(text/markdown)</i>
            </td>
            <td>Static document file exposed from /docs: structure.md</td>
        </tr>
</tbody>
</table>

## 🧩 Resource Templates (2)

<table style="text-align: left;">
<thead>
    <tr>
        <th style="width: auto;"></th>
        <th style="width: auto;">Icon</th>
        <th style="width: auto;">Name</th>
        <th style="width: auto;">Uri Template</th>
        <th style="width: auto;">Description</th>
    </tr>
</thead>
<tbody style="vertical-align: top;">
        <tr>
            <td>1.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>Dynamic Text Resource</b></code>
            </td>
            <td>
                <a>demo://resource/dynamic/text/{resourceId}</a> <i>(text/plain)</i>
            </td>
            <td>Plaintext dynamic resource fabricated from the {resourceId} variable, which must be an integer.</td>
        </tr>
        <tr>
            <td>2.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>Dynamic Blob Resource</b></code>
            </td>
            <td>
                <a>demo://resource/dynamic/blob/{resourceId}</a> <i>(application/octet-stream)</i>
            </td>
            <td>Binary (base64) dynamic resource fabricated from the {resourceId} variable, which must be an integer.</td>
        </tr>
</tbody>
</table>

<sup>◾ generated by [mcp-discovery](https://github.com/rust-mcp-stack/mcp-discovery)</sup>
