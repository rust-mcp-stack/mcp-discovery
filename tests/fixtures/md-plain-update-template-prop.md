<!-- mcp-discovery-render template=txt-->
## example-servers/everything 1.0.0
| 🟢 Tools (8) | 🟢 Prompts (3) | 🟢 Resources (10) | 🟢 Logging | <span style="opacity:0.6">🔴 Experimental</span> |
| --- | --- | --- | --- | --- |
## 🛠️ Tools (8)

<table style="text-align: left;">
<thead>
    <tr>
        <th style="width: auto;"></th>
        <th style="width: auto;">Tool Name</th>
        <th style="width: auto;">Description</th>
        <th style="width: auto;">Inputs</th>
    </tr>
</thead>
<tbody style="vertical-align: top;">
        <tr>
            <td>1.</td>
            <td>
                <code><b>add</b></code>
            </td>
            <td>Adds two numbers</td>
            <td>
                <ul>
                    <li style="white-space: nowrap;"> <code>a</code> : number<br /></li>
                    <li style="white-space: nowrap;"> <code>b</code> : number<br /></li>
                </ul>
            </td>
        </tr>
        <tr>
            <td>2.</td>
            <td>
                <code><b>annotatedMessage</b></code>
            </td>
            <td>Demonstrates how annotations can be used to provide metadata about content</td>
            <td>
                <ul>
                    <li style="white-space: nowrap;"> <code>includeImage</code> : boolean<br /></li>
                    <li style="white-space: nowrap;"> <code>messageType</code> : string<br /></li>
                </ul>
            </td>
        </tr>
        <tr>
            <td>3.</td>
            <td>
                <code><b>echo</b></code>
            </td>
            <td>Echoes back the input</td>
            <td>
                <ul>
                    <li style="white-space: nowrap;"> <code>message</code> : string<br /></li>
                </ul>
            </td>
        </tr>
        <tr>
            <td>4.</td>
            <td>
                <code><b>getResourceReference</b></code>
            </td>
            <td>Returns a resource reference that can be used by MCP clients</td>
            <td>
                <ul>
                    <li style="white-space: nowrap;"> <code>resourceId</code> : number<br /></li>
                </ul>
            </td>
        </tr>
        <tr>
            <td>5.</td>
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
            <td>6.</td>
            <td>
                <code><b>longRunningOperation</b></code>
            </td>
            <td>Demonstrates a long running operation with progress updates</td>
            <td>
                <ul>
                    <li style="white-space: nowrap;"> <code>duration</code> : number<br /></li>
                    <li style="white-space: nowrap;"> <code>steps</code> : number<br /></li>
                </ul>
            </td>
        </tr>
        <tr>
            <td>7.</td>
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
            <td>8.</td>
            <td>
                <code><b>sampleLLM</b></code>
            </td>
            <td>Samples from an LLM using MCP's sampling feature</td>
            <td>
                <ul>
                    <li style="white-space: nowrap;"> <code>maxTokens</code> : number<br /></li>
                    <li style="white-space: nowrap;"> <code>prompt</code> : string<br /></li>
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
        <th style="width: auto;">Resource Name</th>
        <th style="width: auto;">Uri</th>
        <th style="width: auto;">Description</th>
    </tr>
</thead>
<tbody style="vertical-align: top;">
        <tr>
            <td>1.</td>
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
        <th style="width: auto;">Name</th>
        <th style="width: auto;">Uri Template</th>
        <th style="width: auto;">Description</th>
    </tr>
</thead>
<tbody style="vertical-align: top;">
        <tr>
            <td>1.</td>
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
