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
pub struct AppCheckVersionResult {
    #[serde(rename = "versions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub versions: Option<Option<Vec<String>>>,
    #[serde(rename = "newVersions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub new_versions: Option<Option<Vec<String>>>,
}

impl AppCheckVersionResult {
    pub fn new() -> AppCheckVersionResult {
        AppCheckVersionResult {
            versions: None,
            new_versions: None,
        }
    }
}

