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
pub struct EmailSignUpRequest {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "pwd")]
    pub pwd: String,
    /// 邮箱验证码
    #[serde(rename = "emailCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub email_code: Option<Option<String>>,
    /// 手机号
    #[serde(rename = "phone", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub phone: Option<Option<String>>,
    /// 手机验证码（只有启用的手机验证码功能时，才需要传入）
    #[serde(rename = "phoneCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub phone_code: Option<Option<String>>,
    /// 昵称
    #[serde(rename = "nickName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<Option<String>>,
    /// 头像
    #[serde(rename = "avatar", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<Option<String>>,
    /// 自定义数据
    #[serde(rename = "data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub data: Option<Option<String>>,
}

impl EmailSignUpRequest {
    pub fn new(email: String, pwd: String) -> EmailSignUpRequest {
        EmailSignUpRequest {
            email,
            pwd,
            email_code: None,
            phone: None,
            phone_code: None,
            nick_name: None,
            avatar: None,
            data: None,
        }
    }
}

