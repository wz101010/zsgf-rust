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

/// ServiceItem : 服务配置项实体，用于定义和管理服务相关的配置信息。
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceItem {
    /// 服务配置项的唯一标识符。
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// 服务配置项所属的业务代码，用于分类管理。
    #[serde(rename = "bizCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub biz_code: Option<Option<String>>,
    /// 关联的服务商代码，用于标识提供该配置项的服务商。
    #[serde(rename = "providerCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub provider_code: Option<Option<String>>,
    /// 服务配置项所属的功能分组代码，用于组织和管理相关配置项。
    #[serde(rename = "groupCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub group_code: Option<Option<String>>,
    /// 服务配置项的名称，用于描述其功能或用途。
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// 服务配置项的唯一代码，用于系统内部标识。
    #[serde(rename = "code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub code: Option<Option<String>>,
    /// 服务配置项的值类型，例如 'text', 'number', 'boolean' 等。默认为 'text'。
    #[serde(rename = "valueType", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub value_type: Option<Option<String>>,
    /// 服务配置项的图标URL或路径，用于在界面上显示。
    #[serde(rename = "icon", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<String>>,
    /// 服务配置项的默认值，当未设置具体值时使用。
    #[serde(rename = "valueDefaults", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub value_defaults: Option<Option<String>>,
    /// 服务配置项的详细描述信息，用于说明其用途和配置方法。
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// 用于分类或标记服务配置项的标签。
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<String>>,
    /// 指示该配置项是否为系统级别的配置项，默认为 false。
    #[serde(rename = "isSystem", skip_serializing_if = "Option::is_none")]
    pub is_system: Option<bool>,
    /// 指示服务配置项是否在界面上显示，默认为 true。
    #[serde(rename = "show", skip_serializing_if = "Option::is_none")]
    pub show: Option<bool>,
    /// 服务配置项在界面上的显示顺序。
    #[serde(rename = "showIndex", skip_serializing_if = "Option::is_none")]
    pub show_index: Option<i32>,
    /// 服务配置项的创建日期，默认为当前时间。
    #[serde(rename = "createDate", skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    /// 服务配置项的最后更新日期，默认为当前时间。
    #[serde(rename = "lastUpdate", skip_serializing_if = "Option::is_none")]
    pub last_update: Option<String>,
}

impl ServiceItem {
    /// 服务配置项实体，用于定义和管理服务相关的配置信息。
    pub fn new() -> ServiceItem {
        ServiceItem {
            id: None,
            biz_code: None,
            provider_code: None,
            group_code: None,
            name: None,
            code: None,
            value_type: None,
            icon: None,
            value_defaults: None,
            description: None,
            tags: None,
            is_system: None,
            show: None,
            show_index: None,
            create_date: None,
            last_update: None,
        }
    }
}

