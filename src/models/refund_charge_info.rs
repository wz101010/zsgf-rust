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
pub struct RefundChargeInfo {
    #[serde(rename = "chargeType", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<Option<String>>,
    #[serde(rename = "refundChargeFee", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub refund_charge_fee: Option<Option<String>>,
    #[serde(rename = "refundSubFeeDetailList", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub refund_sub_fee_detail_list: Option<Option<Vec<models::RefundSubFee>>>,
    #[serde(rename = "switchFeeRate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub switch_fee_rate: Option<Option<String>>,
}

impl RefundChargeInfo {
    pub fn new() -> RefundChargeInfo {
        RefundChargeInfo {
            charge_type: None,
            refund_charge_fee: None,
            refund_sub_fee_detail_list: None,
            switch_fee_rate: None,
        }
    }
}

