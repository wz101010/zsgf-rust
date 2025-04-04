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

/// Settings : 公共配置存储实体，用于集中管理系统的配置项。
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    /// 配置项的唯一标识符。
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// 配置项所属的业务代码，用于分类管理。
    #[serde(rename = "bizCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub biz_code: Option<Option<String>>,
    /// 配置项所属的业务标识，用于唯一标识业务。
    #[serde(rename = "bizIdentity", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub biz_identity: Option<Option<String>>,
    /// 配置项的提供者代码，用于标识配置来源。
    #[serde(rename = "providerCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub provider_code: Option<Option<String>>,
    /// 配置项的分组代码，用于组织和管理相关配置。
    #[serde(rename = "groupCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub group_code: Option<Option<String>>,
    /// 配置项的唯一代码，用于标识具体的配置项。
    #[serde(rename = "code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub code: Option<Option<String>>,
    /// 配置项的具体值，存储配置内容。
    #[serde(rename = "value", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub value: Option<Option<String>>,
    /// 用于分类或标记配置项的标签。
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<String>>,
    /// 配置项的详细描述，说明其用途和作用。
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// 指示该配置项是否可供前端使用。
    #[serde(rename = "frontendUsable", skip_serializing_if = "Option::is_none")]
    pub frontend_usable: Option<bool>,
    /// 配置项的创建日期，默认为当前时间。
    #[serde(rename = "createDate", skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// 配置项的最后更新日期，默认为当前时间。
    #[serde(rename = "lastUpdate", skip_serializing_if = "Option::is_none")]
    pub last_update: Option<String>,
}

impl Settings {
    /// 公共配置存储实体，用于集中管理系统的配置项。
    pub fn new() -> Settings {
        Settings {
            id: None,
            biz_code: None,
            biz_identity: None,
            provider_code: None,
            group_code: None,
            code: None,
            value: None,
            tags: None,
            description: None,
            frontend_usable: None,
            create_date: None,
            last_update: None,
        }
    }
}

