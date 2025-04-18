/*
 * 全部  API 文档
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostIndexRequest {
    #[serde(rename = "model")]
    pub model: std::collections::HashMap<String, Vec<serde_json::Value>>,
    #[serde(rename = "options", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub options: Option<Option<std::collections::HashMap<String, Vec<serde_json::Value>>>>,
}

impl PostIndexRequest {
    pub fn new(model: std::collections::HashMap<String, Vec<serde_json::Value>>) -> PostIndexRequest {
        PostIndexRequest {
            model,
            options: None,
        }
    }
}

