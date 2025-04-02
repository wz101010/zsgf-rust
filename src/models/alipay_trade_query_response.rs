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
pub struct AlipayTradeQueryResponse {
    #[serde(rename = "code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub code: Option<Option<String>>,
    #[serde(rename = "msg", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub msg: Option<Option<String>>,
    #[serde(rename = "subCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sub_code: Option<Option<String>>,
    #[serde(rename = "subMsg", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sub_msg: Option<Option<String>>,
    #[serde(rename = "isError", skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
    #[serde(rename = "additionalStatus", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub additional_status: Option<Option<String>>,
    #[serde(rename = "alipayStoreId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub alipay_store_id: Option<Option<String>>,
    #[serde(rename = "alipaySubMerchantId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub alipay_sub_merchant_id: Option<Option<String>>,
    #[serde(rename = "asyncPayApplyStatus", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub async_pay_apply_status: Option<Option<String>>,
    #[serde(rename = "authTradePayMode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub auth_trade_pay_mode: Option<Option<String>>,
    #[serde(rename = "bizSettleMode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub biz_settle_mode: Option<Option<String>>,
    #[serde(rename = "bkagentRespInfo", skip_serializing_if = "Option::is_none")]
    pub bkagent_resp_info: Option<Box<models::BkAgentRespInfo>>,
    #[serde(rename = "body", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub body: Option<Option<String>>,
    #[serde(rename = "buyerLogonId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub buyer_logon_id: Option<Option<String>>,
    #[serde(rename = "buyerOpenId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub buyer_open_id: Option<Option<String>>,
    #[serde(rename = "buyerPayAmount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub buyer_pay_amount: Option<Option<String>>,
    #[serde(rename = "buyerUserId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub buyer_user_id: Option<Option<String>>,
    #[serde(rename = "buyerUserName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub buyer_user_name: Option<Option<String>>,
    #[serde(rename = "buyerUserType", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub buyer_user_type: Option<Option<String>>,
    #[serde(rename = "cashierType", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cashier_type: Option<Option<String>>,
    #[serde(rename = "chargeAmount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub charge_amount: Option<Option<String>>,
    #[serde(rename = "chargeFlags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub charge_flags: Option<Option<String>>,
    #[serde(rename = "chargeInfoList", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub charge_info_list: Option<Option<Vec<models::ChargeInfo>>>,
    #[serde(rename = "creditBizOrderId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub credit_biz_order_id: Option<Option<String>>,
    #[serde(rename = "creditPayMode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub credit_pay_mode: Option<Option<String>>,
    #[serde(rename = "discountAmount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub discount_amount: Option<Option<String>>,
    #[serde(rename = "discountGoodsDetail", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub discount_goods_detail: Option<Option<String>>,
    #[serde(rename = "enterprisePayInfo", skip_serializing_if = "Option::is_none")]
    pub enterprise_pay_info: Option<Box<models::EnterprisePayInfo>>,
    #[serde(rename = "extInfos", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ext_infos: Option<Option<String>>,
    #[serde(rename = "fulfillmentDetailList", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub fulfillment_detail_list: Option<Option<Vec<models::FulfillmentDetail>>>,
    #[serde(rename = "fundBillList", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub fund_bill_list: Option<Option<Vec<models::TradeFundBill>>>,
    #[serde(rename = "hbFqPayInfo", skip_serializing_if = "Option::is_none")]
    pub hb_fq_pay_info: Option<Box<models::HbFqPayInfo>>,
    #[serde(rename = "hybAmount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub hyb_amount: Option<Option<String>>,
    #[serde(rename = "industrySepcDetail", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub industry_sepc_detail: Option<Option<String>>,
    #[serde(rename = "industrySepcDetailAcc", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub industry_sepc_detail_acc: Option<Option<String>>,
    #[serde(rename = "industrySepcDetailGov", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub industry_sepc_detail_gov: Option<Option<String>>,
    #[serde(rename = "intactChargeInfoList", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub intact_charge_info_list: Option<Option<Vec<models::IntactChargeInfo>>>,
    #[serde(rename = "invoiceAmount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub invoice_amount: Option<Option<String>>,
    #[serde(rename = "mdiscountAmount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mdiscount_amount: Option<Option<String>>,
    #[serde(rename = "medicalInsuranceInfo", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub medical_insurance_info: Option<Option<String>>,
    #[serde(rename = "openId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub open_id: Option<Option<String>>,
    #[serde(rename = "outTradeNo", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub out_trade_no: Option<Option<String>>,
    #[serde(rename = "passbackParams", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub passback_params: Option<Option<String>>,
    #[serde(rename = "payAmount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pay_amount: Option<Option<String>>,
    #[serde(rename = "payCurrency", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pay_currency: Option<Option<String>>,
    #[serde(rename = "paymentInfoWithIdList", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payment_info_with_id_list: Option<Option<Vec<models::PaymentInfoWithId>>>,
    #[serde(rename = "periodScene", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub period_scene: Option<Option<String>>,
    #[serde(rename = "pointAmount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub point_amount: Option<Option<String>>,
    #[serde(rename = "preAuthPayAmount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pre_auth_pay_amount: Option<Option<String>>,
    #[serde(rename = "receiptAmount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub receipt_amount: Option<Option<String>>,
    #[serde(rename = "receiptCurrencyType", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub receipt_currency_type: Option<Option<String>>,
    #[serde(rename = "reqGoodsDetail", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub req_goods_detail: Option<Option<Vec<models::GoodsDetail>>>,
    #[serde(rename = "sendPayDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub send_pay_date: Option<Option<String>>,
    #[serde(rename = "settleAmount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub settle_amount: Option<Option<String>>,
    #[serde(rename = "settleCurrency", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub settle_currency: Option<Option<String>>,
    #[serde(rename = "settleTransRate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub settle_trans_rate: Option<Option<String>>,
    #[serde(rename = "settlementId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub settlement_id: Option<Option<String>>,
    #[serde(rename = "storeId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub store_id: Option<Option<String>>,
    #[serde(rename = "storeName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub store_name: Option<Option<String>>,
    #[serde(rename = "subject", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub subject: Option<Option<String>>,
    #[serde(rename = "tapPayInfo", skip_serializing_if = "Option::is_none")]
    pub tap_pay_info: Option<Box<models::TapPayInfo>>,
    #[serde(rename = "terminalId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub terminal_id: Option<Option<String>>,
    #[serde(rename = "totalAmount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<Option<String>>,
    #[serde(rename = "tradeNo", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub trade_no: Option<Option<String>>,
    #[serde(rename = "tradeSettleInfo", skip_serializing_if = "Option::is_none")]
    pub trade_settle_info: Option<Box<models::TradeSettleInfo>>,
    #[serde(rename = "tradeStatus", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub trade_status: Option<Option<String>>,
    #[serde(rename = "transCurrency", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub trans_currency: Option<Option<String>>,
    #[serde(rename = "transPayRate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub trans_pay_rate: Option<Option<String>>,
    #[serde(rename = "voucherDetailList", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub voucher_detail_list: Option<Option<Vec<models::VoucherDetail>>>,
}

impl AlipayTradeQueryResponse {
    pub fn new() -> AlipayTradeQueryResponse {
        AlipayTradeQueryResponse {
            code: None,
            msg: None,
            sub_code: None,
            sub_msg: None,
            is_error: None,
            additional_status: None,
            alipay_store_id: None,
            alipay_sub_merchant_id: None,
            async_pay_apply_status: None,
            auth_trade_pay_mode: None,
            biz_settle_mode: None,
            bkagent_resp_info: None,
            body: None,
            buyer_logon_id: None,
            buyer_open_id: None,
            buyer_pay_amount: None,
            buyer_user_id: None,
            buyer_user_name: None,
            buyer_user_type: None,
            cashier_type: None,
            charge_amount: None,
            charge_flags: None,
            charge_info_list: None,
            credit_biz_order_id: None,
            credit_pay_mode: None,
            discount_amount: None,
            discount_goods_detail: None,
            enterprise_pay_info: None,
            ext_infos: None,
            fulfillment_detail_list: None,
            fund_bill_list: None,
            hb_fq_pay_info: None,
            hyb_amount: None,
            industry_sepc_detail: None,
            industry_sepc_detail_acc: None,
            industry_sepc_detail_gov: None,
            intact_charge_info_list: None,
            invoice_amount: None,
            mdiscount_amount: None,
            medical_insurance_info: None,
            open_id: None,
            out_trade_no: None,
            passback_params: None,
            pay_amount: None,
            pay_currency: None,
            payment_info_with_id_list: None,
            period_scene: None,
            point_amount: None,
            pre_auth_pay_amount: None,
            receipt_amount: None,
            receipt_currency_type: None,
            req_goods_detail: None,
            send_pay_date: None,
            settle_amount: None,
            settle_currency: None,
            settle_trans_rate: None,
            settlement_id: None,
            store_id: None,
            store_name: None,
            subject: None,
            tap_pay_info: None,
            terminal_id: None,
            total_amount: None,
            trade_no: None,
            trade_settle_info: None,
            trade_status: None,
            trans_currency: None,
            trans_pay_rate: None,
            voucher_detail_list: None,
        }
    }
}

