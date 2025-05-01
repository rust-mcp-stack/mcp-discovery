use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::{
    error::{DiscoveryError, DiscoveryResult},
    types::{McpToolSParams, ParamTypes},
};

/// Parses an object schema into a vector of `McpToolSParams`.
pub fn get_param_object(object_map: &Map<String, Value>) -> DiscoveryResult<Vec<McpToolSParams>> {
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
            let param_type = get_param_type(param_value)?;
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
pub fn get_param_type(type_info: &Map<String, Value>) -> DiscoveryResult<ParamTypes> {
    let type_name =
        type_info
            .get("type")
            .and_then(|v| v.as_str())
            .ok_or(DiscoveryError::InvalidSchema(
                "Missing or invalid 'type' field".to_string(),
            ))?;

    match type_name {
        "array" => {
            let items_map = type_info.get("items").and_then(|v| v.as_object()).ok_or(
                DiscoveryError::InvalidSchema(
                    "Missing or invalid 'items' field in array type".to_string(),
                ),
            )?;
            Ok(ParamTypes::Array(vec![get_param_type(items_map)?]))
        }
        "object" => Ok(ParamTypes::Object(get_param_object(type_info)?)),
        _ => Ok(ParamTypes::Primitive(type_name.to_string())),
    }
}

pub fn tool_params(
    properties: &Option<HashMap<String, Map<String, Value>>>,
) -> Vec<McpToolSParams> {
    let result = properties.clone().map(|props| {
        let mut params: Vec<_> = props
            .iter()
            .map(|(prop_name, prop_map)| {
                let param_name = prop_name.to_owned();
                let prop_type = get_param_type(prop_map).unwrap();
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
