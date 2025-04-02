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
pub struct UnionIdSignInRequest {
    #[serde(rename = "unionID")]
    pub union_id: String,
    #[serde(rename = "platform")]
    pub platform: String,
    /// 如果启用双因素认证登录，则必填
    #[serde(rename = "twoFactorCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub two_factor_code: Option<Option<String>>,
}

impl UnionIdSignInRequest {
    pub fn new(union_id: String, platform: String) -> UnionIdSignInRequest {
        UnionIdSignInRequest {
            union_id,
            platform,
            two_factor_code: None,
        }
    }
}

