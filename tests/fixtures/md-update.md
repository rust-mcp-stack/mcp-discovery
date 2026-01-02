<!-- mcp-discovery-render -->
## example-servers/everything 1.0.0
| ✔ Tools (13) | ✔ Prompts (3) | ✔ Resources (10) | ✔ Logging | ✔ Completions | ~~<span style="opacity:0.6">✘ Tasks</span>~~ |
| --- | --- | --- | --- | --- | --- |

## 🛠️ Tools (13)

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
                <code><b>add</b></code>
            </td>
            <td>Adds two numbers</td>
            <td>
                <ul>
                    <li> <code>a</code> : number<br /></li>
                    <li> <code>b</code> : number<br /></li>
                </ul>
            </td>
        </tr>
        <tr>
            <td>2.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>annotatedMessage</b></code>
            </td>
            <td>Demonstrates how annotations can be used to provide metadata about content</td>
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
                <code><b>echo</b></code>
            </td>
            <td>Echoes back the input</td>
            <td>
                <ul>
                    <li> <code>message</code> : string<br /></li>
                </ul>
            </td>
        </tr>
        <tr>
            <td>4.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>getResourceLinks</b></code>
            </td>
            <td>Returns multiple resource links that reference different types of resources</td>
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
                <code><b>getResourceReference</b></code>
            </td>
            <td>Returns a resource reference that can be used by MCP clients</td>
            <td>
                <ul>
                    <li> <code>resourceId</code> : number<br /></li>
                </ul>
            </td>
        </tr>
        <tr>
            <td>6.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>getTinyImage</b></code>
            </td>
            <td>Returns the MCP_TINY_IMAGE</td>
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
                <code><b>listRoots</b></code>
            </td>
            <td>Lists the current MCP roots provided by the client. Demonstrates the roots protocol capability even though this server doesn't access files.</td>
            <td>
                <ul>
                </ul>
            </td>
        </tr>
        <tr>
            <td>8.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>longRunningOperation</b></code>
            </td>
            <td>Demonstrates a long running operation with progress updates</td>
            <td>
                <ul>
                    <li> <code>duration</code> : number<br /></li>
                    <li> <code>steps</code> : number<br /></li>
                </ul>
            </td>
        </tr>
        <tr>
            <td>9.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>printEnv</b></code>
            </td>
            <td>Prints all environment variables, helpful for debugging MCP server configuration</td>
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
                <code><b>sampleLLM</b></code>
            </td>
            <td>Samples from an LLM using MCP's sampling feature</td>
            <td>
                <ul>
                    <li> <code>maxTokens</code> : number<br /></li>
                    <li> <code>prompt</code> : string<br /></li>
                </ul>
            </td>
        </tr>
        <tr>
            <td>11.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>startElicitation</b></code>
            </td>
            <td>Elicitation test tool that demonstrates how to request user input with various field types (string, boolean, email, uri, date, integer, number, enum)</td>
            <td>
                <ul>
                </ul>
            </td>
        </tr>
        <tr>
            <td>12.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>structuredContent</b></code>
            </td>
            <td>Returns structured content along with an output schema for client data validation</td>
            <td>
                <ul>
                    <li> <code>location</code> : string<br /></li>
                </ul>
            </td>
        </tr>
        <tr>
            <td>13.</td>
            <td>
                <!--- no icon -->
            </td>
            <td>
                <code><b>zip</b></code>
            </td>
            <td>Compresses the provided resource files (mapping of name to URI, which can be a data URI) to a zip file, which it returns as a data URI resource link.</td>
            <td>
                <ul>
                    <li> <code>files</code> : unknown<br /></li>
                </ul>
            </td>
        </tr>
</tbody>
</table>

## 📝 Prompts (3)

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
                <code><b>simple_prompt</b></code>
            </td>
            <td>A prompt without arguments</td>
        </tr>
        <tr>
            <td>2.</td>
            <td>
                <code><b>complex_prompt</b></code>
            </td>
            <td>A prompt with arguments</td>
        </tr>
        <tr>
            <td>3.</td>
            <td>
                <code><b>resource_prompt</b></code>
            </td>
            <td>A prompt that includes an embedded resource reference</td>
        </tr>
</tbody>
</table>

## 📄 Resources (10)

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
                <code><b>Resource 1</b></code>
            </td>
            <td>
                <a>test://static/resource/1</a> <i>(text/plain)</i>
            </td>
            <td></td>
        </tr>
        <tr>
            <td>2.</td>
            <td>
              <!--- no icon -->
            </td>
            <td>
                <code><b>Resource 2</b></code>
            </td>
            <td>
                <a>test://static/resource/2</a> <i>(application/octet-stream)</i>
            </td>
            <td></td>
        </tr>
        <tr>
            <td>3.</td>
            <td>
              <!--- no icon -->
            </td>
            <td>
                <code><b>Resource 3</b></code>
            </td>
            <td>
                <a>test://static/resource/3</a> <i>(text/plain)</i>
            </td>
            <td></td>
        </tr>
        <tr>
            <td>4.</td>
            <td>
              <!--- no icon -->
            </td>
            <td>
                <code><b>Resource 4</b></code>
            </td>
            <td>
                <a>test://static/resource/4</a> <i>(application/octet-stream)</i>
            </td>
            <td></td>
        </tr>
        <tr>
            <td>5.</td>
            <td>
              <!--- no icon -->
            </td>
            <td>
                <code><b>Resource 5</b></code>
            </td>
            <td>
                <a>test://static/resource/5</a> <i>(text/plain)</i>
            </td>
            <td></td>
        </tr>
        <tr>
            <td>6.</td>
            <td>
              <!--- no icon -->
            </td>
            <td>
                <code><b>Resource 6</b></code>
            </td>
            <td>
                <a>test://static/resource/6</a> <i>(application/octet-stream)</i>
            </td>
            <td></td>
        </tr>
        <tr>
            <td>7.</td>
            <td>
              <!--- no icon -->
            </td>
            <td>
                <code><b>Resource 7</b></code>
            </td>
            <td>
                <a>test://static/resource/7</a> <i>(text/plain)</i>
            </td>
            <td></td>
        </tr>
        <tr>
            <td>8.</td>
            <td>
              <!--- no icon -->
            </td>
            <td>
                <code><b>Resource 8</b></code>
            </td>
            <td>
                <a>test://static/resource/8</a> <i>(application/octet-stream)</i>
            </td>
            <td></td>
        </tr>
        <tr>
            <td>9.</td>
            <td>
              <!--- no icon -->
            </td>
            <td>
                <code><b>Resource 9</b></code>
            </td>
            <td>
                <a>test://static/resource/9</a> <i>(text/plain)</i>
            </td>
            <td></td>
        </tr>
        <tr>
            <td>10.</td>
            <td>
              <!--- no icon -->
            </td>
            <td>
                <code><b>Resource 10</b></code>
            </td>
            <td>
                <a>test://static/resource/10</a> <i>(application/octet-stream)</i>
            </td>
            <td></td>
        </tr>
</tbody>
</table>

## 🧩 Resource Templates (1)

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
                <code><b>Static Resource</b></code>
            </td>
            <td>
                <a>test://static/resource/{id}</a> 
            </td>
            <td>A static resource with a numeric ID</td>
        </tr>
</tbody>
</table>

<sup>◾ generated by [mcp-discovery](https://github.com/rust-mcp-stack/mcp-discovery)</sup>
<!-- mcp-discovery-render-end -->