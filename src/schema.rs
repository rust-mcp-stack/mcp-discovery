use std::collections::{HashMap, HashSet};

use serde_json::{Map, Value};

use crate::{
    error::{DiscoveryError, DiscoveryResult},
    types::{McpToolSParams, ParamTypes},
};

/// Resolves a $ref path to its target value in the schema.
fn resolve_ref<'a>(
    ref_path: &str,
    root_schema: &'a Value,
    visited: &mut HashSet<String>,
) -> DiscoveryResult<&'a Value> {
    if !ref_path.starts_with("#/") {
        return Err(DiscoveryError::InvalidSchema(format!(
            "$ref '{}' must start with '#/'",
            ref_path
        )));
    }

    if !visited.insert(ref_path.to_string()) {
        return Err(DiscoveryError::InvalidSchema(format!(
            "Cycle detected in $ref path '{}'",
            ref_path
        )));
    }

    let path = ref_path.trim_start_matches("#/").split('/');
    let mut current = root_schema;

    for segment in path {
        if segment.is_empty() {
            return Err(DiscoveryError::InvalidSchema(format!(
                "Invalid $ref path '{}': empty segment",
                ref_path
            )));
        }
        current = match current {
            Value::Object(obj) => obj.get(segment).ok_or_else(|| {
                DiscoveryError::InvalidSchema(format!(
                    "Invalid $ref path '{}': segment '{}' not found",
                    ref_path, segment
                ))
            })?,
            Value::Array(arr) => segment
                .parse::<usize>()
                .ok()
                .and_then(|i| arr.get(i))
                .ok_or_else(|| {
                    DiscoveryError::InvalidSchema(format!(
                        "Invalid $ref path '{}': segment '{}' not found in array",
                        ref_path, segment
                    ))
                })?,
            _ => {
                return Err(DiscoveryError::InvalidSchema(format!(
                    "Invalid $ref path '{}': cannot traverse into non-object/array",
                    ref_path
                )))
            }
        };
    }

    Ok(current)
}

/// Parses an object schema into a vector of `McpToolSParams`.
pub fn param_object(
    object_map: &Map<String, Value>,
    root_schema: &Value,
    visited: &mut HashSet<String>,
) -> DiscoveryResult<Vec<McpToolSParams>> {
    let properties = object_map
        .get("properties")
        .and_then(|v| v.as_object())
        .ok_or(DiscoveryError::InvalidSchema(
            "Missing or invalid 'properties' field".to_string(),
        ))?;

    let required = object_map
        .get("required")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
        .unwrap_or_default();

    let params: Vec<McpToolSParams> = properties
        .iter()
        .map(|(param_name, param_value)| {
            let param_value = param_value
                .as_object()
                .ok_or(DiscoveryError::InvalidSchema(format!(
                    "Property '{}' is not an object",
                    param_name
                )))?;
            let param_type = param_type(param_value, root_schema, visited)?;
            let param_description = object_map
                .get("description")
                .and_then(|v| v.as_str())
                .map(String::from);

            Ok::<McpToolSParams, DiscoveryError>(McpToolSParams {
                param_name: param_name.clone(),
                param_type,
                param_description,
                required: required.contains(&param_name.as_str()),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(params)
}

/// Determines the parameter type from a schema definition.
pub fn param_type(
    type_info: &Map<String, Value>,
    root_schema: &Value,
    visited: &mut HashSet<String>,
) -> DiscoveryResult<ParamTypes> {
    // Handle $ref
    if let Some(ref_path) = type_info.get("$ref") {
        let ref_path_str = ref_path.as_str().ok_or(DiscoveryError::InvalidSchema(
            "$ref must be a string".to_string(),
        ))?;
        let ref_value = resolve_ref(ref_path_str, root_schema, visited)?;
        let ref_map = ref_value
            .as_object()
            .ok_or(DiscoveryError::InvalidSchema(format!(
                "$ref '{}' does not point to an object",
                ref_path_str
            )))?;
        return param_type(ref_map, root_schema, visited);
    }

    // Check for 'enum' keyword
    if let Some(enum_values) = type_info.get("enum") {
        let values = enum_values.as_array().ok_or(DiscoveryError::InvalidSchema(
            "'enum' field must be an array".to_string(),
        ))?;
        if values.is_empty() {
            return Err(DiscoveryError::InvalidSchema(
                "'enum' array cannot be empty".to_string(),
            ));
        }
        let mut param_types = Vec::new();
        for value in values {
            let param_type = match value {
                Value::String(s) => ParamTypes::Primitive(s.clone()),
                Value::Number(n) => ParamTypes::Primitive(n.to_string()),
                Value::Bool(b) => ParamTypes::Primitive(b.to_string()),
                Value::Null => ParamTypes::Primitive("null".to_string()),
                _ => {
                    return Err(DiscoveryError::InvalidSchema(format!(
                        "Unsupported enum value type: {}",
                        serde_json::to_string(value).unwrap_or_default()
                    )))
                }
            };
            param_types.push(param_type);
        }
        return Ok(ParamTypes::EnumValues(param_types));
    }

    // Check for 'const' keyword
    if let Some(const_value) = type_info.get("const") {
        return Ok(ParamTypes::Primitive(
            serde_json::to_string(const_value)
                .unwrap_or_default()
                .trim_matches('"')
                .to_string(),
        ));
    }

    // Check for 'anyOf'
    if let Some(any_of) = type_info.get("anyOf") {
        let any_of_array = any_of.as_array().ok_or(DiscoveryError::InvalidSchema(
            "'anyOf' field must be an array".to_string(),
        ))?;
        if any_of_array.is_empty() {
            return Err(DiscoveryError::InvalidSchema(
                "'anyOf' array cannot be empty".to_string(),
            ));
        }
        let mut enum_types = Vec::new();
        for item in any_of_array {
            let item_map = item.as_object().ok_or(DiscoveryError::InvalidSchema(
                "Items in 'anyOf' must be objects".to_string(),
            ))?;
            enum_types.push(param_type(item_map, root_schema, visited)?);
        }
        return Ok(ParamTypes::Anyof(enum_types));
    }

    // Check for 'oneOf'
    if let Some(one_of) = type_info.get("oneOf") {
        let one_of_array = one_of.as_array().ok_or(DiscoveryError::InvalidSchema(
            "'oneOf' field must be an array".to_string(),
        ))?;
        if one_of_array.is_empty() {
            return Err(DiscoveryError::InvalidSchema(
                "'oneOf' array cannot be empty".to_string(),
            ));
        }
        let mut one_of_types = Vec::new();
        for item in one_of_array {
            let item_map = item.as_object().ok_or(DiscoveryError::InvalidSchema(
                "Items in 'oneOf' must be objects".to_string(),
            ))?;
            one_of_types.push(param_type(item_map, root_schema, visited)?);
        }
        return Ok(ParamTypes::OneOf(one_of_types));
    }

    // Check for 'allOf'
    if let Some(all_of) = type_info.get("allOf") {
        let all_of_array = all_of.as_array().ok_or(DiscoveryError::InvalidSchema(
            "'allOf' field must be an array".to_string(),
        ))?;
        if all_of_array.is_empty() {
            return Err(DiscoveryError::InvalidSchema(
                "'allOf' array cannot be empty".to_string(),
            ));
        }
        let mut all_of_types = Vec::new();
        for item in all_of_array {
            let item_map = item.as_object().ok_or(DiscoveryError::InvalidSchema(
                "Items in 'allOf' must be objects".to_string(),
            ))?;
            all_of_types.push(param_type(item_map, root_schema, visited)?);
        }
        return Ok(ParamTypes::AllOf(all_of_types));
    }

    // Other types
    let type_name =
        type_info
            .get("type")
            .and_then(|v| v.as_str())
            .ok_or(DiscoveryError::InvalidSchema(format!(
                "Missing or invalid 'type' field: {}",
                serde_json::to_string(&type_info).unwrap_or_default()
            )))?;

    match type_name {
        "array" => {
            let items_map = type_info.get("items").and_then(|v| v.as_object()).ok_or(
                DiscoveryError::InvalidSchema(
                    "Missing or invalid 'items' field in array type".to_string(),
                ),
            )?;
            Ok(ParamTypes::Array(vec![param_type(
                items_map,
                root_schema,
                visited,
            )?]))
        }
        "object" => Ok(ParamTypes::Object(param_object(
            type_info,
            root_schema,
            visited,
        )?)),
        _ => Ok(ParamTypes::Primitive(type_name.to_string())),
    }
}

/// Processes tool parameters with a given properties map and root schema.
pub fn tool_params(
    properties: &Option<HashMap<String, Map<String, Value>>>,
    root_schema: &Value,
) -> Vec<McpToolSParams> {
    let mut visited = HashSet::new();
    let result = properties.clone().map(|props| {
        let mut params: Vec<_> = props
            .iter()
            .map(|(prop_name, prop_map)| {
                let param_name = prop_name.to_owned();
                let prop_type = param_type(prop_map, root_schema, &mut visited)
                    .unwrap_or_else(|_| ParamTypes::Primitive("unknown".to_string()));
                let prop_description = prop_map
                    .get("description")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                McpToolSParams {
                    param_name,
                    param_type: prop_type,
                    param_description: prop_description,
                    required: true,
                }
            })
            .collect();
        params.sort_by(|a, b| a.param_name.cmp(&b.param_name));
        params
    });
    result.unwrap_or_default()
}
