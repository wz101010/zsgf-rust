# \AlipayApi

All URIs are relative to *https://api-dev.zashigaofa.cn*

Method | HTTP request | Description
------------- | ------------- | -------------
[**alipay_create_order**](AlipayApi.md#alipay_create_order) | **POST** /Alipay/{appKey}/CreateOrder | 创建当面付订单
[**alipay_create_order_page_pay**](AlipayApi.md#alipay_create_order_page_pay) | **POST** /Alipay/{appKey}/CreateOrderPagePay | 创建PC支付订单
[**alipay_create_order_wap_pay**](AlipayApi.md#alipay_create_order_wap_pay) | **POST** /Alipay/{appKey}/CreateOrderWapPay | 创建WAP支付订单
[**alipay_order_detail**](AlipayApi.md#alipay_order_detail) | **GET** /Alipay/{appKey}/OrderDetail | 获取订单详情
[**alipay_order_refund**](AlipayApi.md#alipay_order_refund) | **POST** /Alipay/{appKey}/OrderRefund | 发起订单退款
[**alipay_return_page_notify**](AlipayApi.md#alipay_return_page_notify) | **POST** /Alipay/{appKey}/ReturnPageNotify | 支付成功回调通知



## alipay_create_order

> models::StringApiResponse alipay_create_order(app_key, alipay_create_order_request)
创建当面付订单

创建一个当面付订单，并返回支付二维码。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**alipay_create_order_request** | Option<[**AlipayCreateOrderRequest**](AlipayCreateOrderRequest.md)> |  |  |

### Return type

[**models::StringApiResponse**](StringApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alipay_create_order_page_pay

> models::StringApiResponse alipay_create_order_page_pay(app_key, alipay_create_order_page_pay_request)
创建PC支付订单

创建一个PC支付订单，并返回支付页面。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**alipay_create_order_page_pay_request** | Option<[**AlipayCreateOrderPagePayRequest**](AlipayCreateOrderPagePayRequest.md)> |  |  |

### Return type

[**models::StringApiResponse**](StringApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alipay_create_order_wap_pay

> models::StringApiResponse alipay_create_order_wap_pay(app_key, alipay_create_order_wap_pay_request)
创建WAP支付订单

创建一个WAP支付订单，并返回支付页面。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**alipay_create_order_wap_pay_request** | Option<[**AlipayCreateOrderWapPayRequest**](AlipayCreateOrderWapPayRequest.md)> |  |  |

### Return type

[**models::StringApiResponse**](StringApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alipay_order_detail

> models::AlipayTradeQueryResponseApiResponse alipay_order_detail(app_key, order_no)
获取订单详情

查询订单详情，包括订单状态和支付信息。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**order_no** | Option<**String**> | 订单号 |  |

### Return type

[**models::AlipayTradeQueryResponseApiResponse**](AlipayTradeQueryResponseApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alipay_order_refund

> models::AlipayTradeRefundResponseApiResponse alipay_order_refund(app_key, amount, order_no)
发起订单退款

对指定订单进行退款操作。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**amount** | Option<**String**> | 退款金额 |  |
**order_no** | Option<**String**> | 订单号 |  |

### Return type

[**models::AlipayTradeRefundResponseApiResponse**](AlipayTradeRefundResponseApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alipay_return_page_notify

> models::BooleanApiResponse alipay_return_page_notify(app_key, return_page_notify_request)
支付成功回调通知

处理支付宝支付成功的同步通知。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**return_page_notify_request** | Option<[**ReturnPageNotifyRequest**](ReturnPageNotifyRequest.md)> |  |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

