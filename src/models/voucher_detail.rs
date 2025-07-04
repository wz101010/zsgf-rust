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
pub struct VoucherDetail {
    #[serde(rename = "amount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub amount: Option<Option<String>>,
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<String>>,
    #[serde(rename = "memo", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub memo: Option<Option<String>>,
    #[serde(rename = "merchantContribute", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub merchant_contribute: Option<Option<String>>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "otherContribute", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub other_contribute: Option<Option<String>>,
    #[serde(rename = "otherContributeDetail", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub other_contribute_detail: Option<Option<Vec<models::ContributeDetail>>>,
    #[serde(rename = "purchaseAntContribute", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub purchase_ant_contribute: Option<Option<String>>,
    #[serde(rename = "purchaseBuyerContribute", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub purchase_buyer_contribute: Option<Option<String>>,
    #[serde(rename = "purchaseMerchantContribute", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub purchase_merchant_contribute: Option<Option<String>>,
    #[serde(rename = "templateId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub template_id: Option<Option<String>>,
    #[serde(rename = "type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Option<String>>,
}

impl VoucherDetail {
    pub fn new() -> VoucherDetail {
        VoucherDetail {
            amount: None,
            id: None,
            memo: None,
            merchant_contribute: None,
            name: None,
            other_contribute: None,
            other_contribute_detail: None,
            purchase_ant_contribute: None,
            purchase_buyer_contribute: None,
            purchase_merchant_contribute: None,
            template_id: None,
            r#type: None,
        }
    }
}

