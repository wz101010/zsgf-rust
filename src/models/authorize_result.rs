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
pub struct AuthorizeResult {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[serde(rename = "error", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error: Option<Option<String>>,
    #[serde(rename = "access_token", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<Option<String>>,
    #[serde(rename = "token_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub token_type: Option<Option<String>>,
    #[serde(rename = "expires_in", skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i64>,
}

impl AuthorizeResult {
    pub fn new() -> AuthorizeResult {
        AuthorizeResult {
            code: None,
            error: None,
            access_token: None,
            token_type: None,
            expires_in: None,
        }
    }
}

