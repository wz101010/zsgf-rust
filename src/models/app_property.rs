/*
 * 用户全部 API 文档
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
pub struct AppProperty {
    #[serde(rename = "code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub code: Option<Option<String>>,
    #[serde(rename = "value", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub value: Option<Option<String>>,
    #[serde(rename = "desc", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Option<String>>,
}

impl AppProperty {
    pub fn new() -> AppProperty {
        AppProperty {
            code: None,
            value: None,
            desc: None,
        }
    }
}

