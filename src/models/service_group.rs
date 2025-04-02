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

/// ServiceGroup : 服务功能分组实体，用于对服务功能进行分类和管理。
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceGroup {
    /// 服务功能分组的唯一标识符。
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// 关联的服务商的唯一标识符。
    #[serde(rename = "providerID", skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<i64>,
    /// 服务功能分组的名称。
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// 服务功能分组的唯一代码，用于系统内部标识。
    #[serde(rename = "code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub code: Option<Option<String>>,
    /// 服务功能分组的图标URL或路径。
    #[serde(rename = "icon", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<String>>,
    /// 服务功能分组的详细描述信息。
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// 指示服务功能分组是否在界面上显示。
    #[serde(rename = "show", skip_serializing_if = "Option::is_none")]
    pub show: Option<bool>,
    /// 服务功能分组在界面上的显示顺序。
    #[serde(rename = "showIndex", skip_serializing_if = "Option::is_none")]
    pub show_index: Option<i32>,
    /// 服务功能分组的创建日期，默认为当前时间。
    #[serde(rename = "createDate", skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// 服务功能分组的最后更新日期，默认为当前时间。
    #[serde(rename = "lastUpdate", skip_serializing_if = "Option::is_none")]
    pub last_update: Option<String>,
}

impl ServiceGroup {
    /// 服务功能分组实体，用于对服务功能进行分类和管理。
    pub fn new() -> ServiceGroup {
        ServiceGroup {
            id: None,
            provider_id: None,
            name: None,
            code: None,
            icon: None,
            description: None,
            show: None,
            show_index: None,
            create_date: None,
            last_update: None,
        }
    }
}

