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
pub struct AccessTokenPostRequest {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<String>>,
    #[serde(rename = "userID", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "permissions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Option<String>>,
    #[serde(rename = "expireInDays", skip_serializing_if = "Option::is_none")]
    pub expire_in_days: Option<i32>,
}

impl AccessTokenPostRequest {
    pub fn new(title: String) -> AccessTokenPostRequest {
        AccessTokenPostRequest {
            title,
            tags: None,
            user_id: None,
            description: None,
            permissions: None,
            expire_in_days: None,
        }
    }
}

