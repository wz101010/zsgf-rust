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
pub struct TapPayInfo {
    #[serde(rename = "paymentMediumType", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payment_medium_type: Option<Option<String>>,
}

impl TapPayInfo {
    pub fn new() -> TapPayInfo {
        TapPayInfo {
            payment_medium_type: None,
        }
    }
}

