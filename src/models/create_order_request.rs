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
pub struct CreateOrderRequest {
    #[serde(rename = "amount")]
    pub amount: f64,
    #[serde(rename = "productName")]
    pub product_name: String,
    #[serde(rename = "productType")]
    pub product_type: String,
    #[serde(rename = "productID")]
    pub product_id: String,
}

impl CreateOrderRequest {
    pub fn new(amount: f64, product_name: String, product_type: String, product_id: String) -> CreateOrderRequest {
        CreateOrderRequest {
            amount,
            product_name,
            product_type,
            product_id,
        }
    }
}

