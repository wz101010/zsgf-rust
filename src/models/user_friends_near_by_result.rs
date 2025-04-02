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
pub struct UserFriendsNearByResult {
    #[serde(rename = "totalFollowers", skip_serializing_if = "Option::is_none")]
    pub total_followers: Option<i64>,
    #[serde(rename = "data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub data: Option<Option<Vec<models::RecommendFriend>>>,
}

impl UserFriendsNearByResult {
    pub fn new() -> UserFriendsNearByResult {
        UserFriendsNearByResult {
            total_followers: None,
            data: None,
        }
    }
}

