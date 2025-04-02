/*
 * 全部  API 文档
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`alipay_create_order`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AlipayCreateOrderError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`alipay_create_order_page_pay`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AlipayCreateOrderPagePayError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`alipay_create_order_wap_pay`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AlipayCreateOrderWapPayError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`alipay_order_detail`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AlipayOrderDetailError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`alipay_order_refund`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AlipayOrderRefundError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`alipay_return_page_notify`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AlipayReturnPageNotifyError {
    UnknownValue(serde_json::Value),
}


/// 创建一个当面付订单，并返回支付二维码。
pub async fn alipay_create_order(configuration: &configuration::Configuration, app_key: &str, alipay_create_order_request: Option<models::AlipayCreateOrderRequest>) -> Result<models::StringApiResponse, Error<AlipayCreateOrderError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_app_key = app_key;
    let p_alipay_create_order_request = alipay_create_order_request;

    let uri_str = format!("{}/Alipay/{appKey}/CreateOrder", configuration.base_path, appKey=crate::apis::urlencode(p_app_key));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_alipay_create_order_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::StringApiResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::StringApiResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AlipayCreateOrderError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// 创建一个PC支付订单，并返回支付页面。
pub async fn alipay_create_order_page_pay(configuration: &configuration::Configuration, app_key: &str, alipay_create_order_page_pay_request: Option<models::AlipayCreateOrderPagePayRequest>) -> Result<models::StringApiResponse, Error<AlipayCreateOrderPagePayError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_app_key = app_key;
    let p_alipay_create_order_page_pay_request = alipay_create_order_page_pay_request;

    let uri_str = format!("{}/Alipay/{appKey}/CreateOrderPagePay", configuration.base_path, appKey=crate::apis::urlencode(p_app_key));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_alipay_create_order_page_pay_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::StringApiResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::StringApiResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AlipayCreateOrderPagePayError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// 创建一个WAP支付订单，并返回支付页面。
pub async fn alipay_create_order_wap_pay(configuration: &configuration::Configuration, app_key: &str, alipay_create_order_wap_pay_request: Option<models::AlipayCreateOrderWapPayRequest>) -> Result<models::StringApiResponse, Error<AlipayCreateOrderWapPayError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_app_key = app_key;
    let p_alipay_create_order_wap_pay_request = alipay_create_order_wap_pay_request;

    let uri_str = format!("{}/Alipay/{appKey}/CreateOrderWapPay", configuration.base_path, appKey=crate::apis::urlencode(p_app_key));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_alipay_create_order_wap_pay_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::StringApiResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::StringApiResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AlipayCreateOrderWapPayError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// 查询订单详情，包括订单状态和支付信息。
pub async fn alipay_order_detail(configuration: &configuration::Configuration, app_key: &str, order_no: Option<&str>) -> Result<models::AlipayTradeQueryResponseApiResponse, Error<AlipayOrderDetailError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_app_key = app_key;
    let p_order_no = order_no;

    let uri_str = format!("{}/Alipay/{appKey}/OrderDetail", configuration.base_path, appKey=crate::apis::urlencode(p_app_key));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_order_no {
        req_builder = req_builder.query(&[("orderNo", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AlipayTradeQueryResponseApiResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AlipayTradeQueryResponseApiResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AlipayOrderDetailError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// 对指定订单进行退款操作。
pub async fn alipay_order_refund(configuration: &configuration::Configuration, app_key: &str, amount: Option<&str>, order_no: Option<&str>) -> Result<models::AlipayTradeRefundResponseApiResponse, Error<AlipayOrderRefundError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_app_key = app_key;
    let p_amount = amount;
    let p_order_no = order_no;

    let uri_str = format!("{}/Alipay/{appKey}/OrderRefund", configuration.base_path, appKey=crate::apis::urlencode(p_app_key));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref param_value) = p_amount {
        req_builder = req_builder.query(&[("amount", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_order_no {
        req_builder = req_builder.query(&[("orderNo", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AlipayTradeRefundResponseApiResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AlipayTradeRefundResponseApiResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AlipayOrderRefundError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// 处理支付宝支付成功的同步通知。
pub async fn alipay_return_page_notify(configuration: &configuration::Configuration, app_key: &str, return_page_notify_request: Option<models::ReturnPageNotifyRequest>) -> Result<models::BooleanApiResponse, Error<AlipayReturnPageNotifyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_app_key = app_key;
    let p_return_page_notify_request = return_page_notify_request;

    let uri_str = format!("{}/Alipay/{appKey}/ReturnPageNotify", configuration.base_path, appKey=crate::apis::urlencode(p_app_key));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_return_page_notify_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::BooleanApiResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::BooleanApiResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AlipayReturnPageNotifyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

