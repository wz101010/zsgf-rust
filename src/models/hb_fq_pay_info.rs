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
pub struct HbFqPayInfo {
    #[serde(rename = "fqAmount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub fq_amount: Option<Option<String>>,
    #[serde(rename = "userInstallNum", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user_install_num: Option<Option<String>>,
}

impl HbFqPayInfo {
    pub fn new() -> HbFqPayInfo {
        HbFqPayInfo {
            fq_amount: None,
            user_install_num: None,
        }
    }
}

