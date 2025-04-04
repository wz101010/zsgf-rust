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

/// Project : 项目
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    /// 状态码
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// 用户 ID
    #[serde(rename = "userID", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// 名称
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// Logo
    #[serde(rename = "logo", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub logo: Option<Option<String>>,
    /// 描述
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// 是否显示
    #[serde(rename = "show", skip_serializing_if = "Option::is_none")]
    pub show: Option<bool>,
    /// 显示索引
    #[serde(rename = "showIndex", skip_serializing_if = "Option::is_none")]
    pub show_index: Option<i64>,
    /// 创建时间
    #[serde(rename = "createDate", skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// 最后更新时间
    #[serde(rename = "lastUpdate", skip_serializing_if = "Option::is_none")]
    pub last_update: Option<String>,
}

impl Project {
    /// 项目
    pub fn new() -> Project {
        Project {
            id: None,
            user_id: None,
            name: None,
            logo: None,
            description: None,
            show: None,
            show_index: None,
            create_date: None,
            last_update: None,
        }
    }
}

