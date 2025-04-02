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
pub struct FileItem {
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "createDate", skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "lastUpdate", skip_serializing_if = "Option::is_none")]
    pub last_update: Option<String>,
    #[serde(rename = "fileSize", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Option<i64>>,
}

impl FileItem {
    pub fn new() -> FileItem {
        FileItem {
            name: None,
            create_date: None,
            last_update: None,
            file_size: None,
        }
    }
}

