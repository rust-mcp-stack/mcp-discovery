use rust_mcp_sdk::schema::{Prompt, Resource, ResourceTemplate};
use std::fmt::Display;

/// Represents the capabilities of an MCP server, indicating which features are supported.
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct McpCapabilities {
    pub tools: bool,
    pub prompts: bool,
    pub resources: bool,
    pub logging: bool,
    pub experimental: bool,
}

impl Display for McpCapabilities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "tools:{}, prompts:{}, resources:{}, logging:{}, experimental:{}",
            self.tools, self.prompts, self.resources, self.logging, self.experimental
        )
    }
}

/// Defines the possible parameter types for tools, which can be primitive (e.g., string, int), objects, or arrays.
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum ParamTypes {
    Primitive(String),
    Object(Vec<McpToolSParams>),
    Array(Vec<ParamTypes>),
    Anyof(Vec<ParamTypes>),      // anyOf
    OneOf(Vec<ParamTypes>),      // oneOf
    AllOf(Vec<ParamTypes>),      // allOf
    EnumValues(Vec<ParamTypes>), // JSON Schema enum
}

impl Display for ParamTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_name = match self {
            ParamTypes::Primitive(type_name) => type_name.to_owned(),
            ParamTypes::Object(items) => {
                format!(
                    "{{{}}}",
                    items
                        .iter()
                        .map(|t| format!("{} : {}", t.param_name, t.param_type))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            ParamTypes::Array(items) => format!("{} [ ]", items[0]),
            ParamTypes::Anyof(types) => types
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<String>>()
                .join(" | "),
            ParamTypes::OneOf(types) => types
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<String>>()
                .join(" | "),
            ParamTypes::AllOf(types) => types
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<String>>()
                .join(" & "),
            ParamTypes::EnumValues(types) => types
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<String>>()
                .join("|"),
        };
        write!(f, "{}", type_name)
    }
}

// impl Serialize for ParamTypes {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         match self {
//             ParamTypes::Primitive(s) => {
//                 let mut map = serializer.serialize_map(Some(1))?;
//                 map.serialize_entry("Primitive", s)?;
//                 map.end()
//             }
//
//             ParamTypes::Object(params) => params.serialize(serializer), // Inline as array
//             ParamTypes::Array(arr) => {
//                 let mut map = serializer.serialize_map(Some(1))?;
//                 map.serialize_entry("Array", arr)?;
//                 map.end()
//             }
//         }
//     }
// }

/// Represents a tool parameter, including its name, type, description, and whether it's required.
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct McpToolSParams {
    pub param_name: String,
    pub param_type: ParamTypes,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub param_description: Option<String>,
    pub required: bool,
}

/// Represents metadata for an MCP tool, including its name, description, and parameters.
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct McpToolMeta {
    pub name: String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: Option<String>,
    pub params: Vec<McpToolSParams>,
}

/// Represents the MCP server's information, including its name, version, capabilities, and supported features.
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct McpServerInfo {
    pub name: String,
    pub version: String,
    pub capabilities: McpCapabilities,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tools: Option<Vec<McpToolMeta>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub prompts: Option<Vec<Prompt>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub resources: Option<Vec<Resource>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub resource_templates: Option<Vec<ResourceTemplate>>,
}
