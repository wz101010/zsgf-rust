# \OrderApi

All URIs are relative to *https://api-dev.zashigaofa.cn*

Method | HTTP request | Description
------------- | ------------- | -------------
[**order**](OrderApi.md#order) | **GET** /Order/{appKey}/{id} | 获取订单详情
[**order_create**](OrderApi.md#order_create) | **POST** /Order/{appKey}/Create | 创建订单
[**orders**](OrderApi.md#orders) | **GET** /Order/{appKey} | 获取订单列表



## order

> models::OrderApiResponse order(id, app_key)
获取订单详情

根据订单ID获取订单详情

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 订单ID | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::OrderApiResponse**](OrderApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_create

> models::CreateOrderResultApiResponse order_create(app_key, create_order_request)
创建订单

根据请求参数创建订单

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**create_order_request** | Option<[**CreateOrderRequest**](CreateOrderRequest.md)> | 订单创建请求 |  |

### Return type

[**models::CreateOrderResultApiResponse**](CreateOrderResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orders

> models::OrderListResultApiResponse orders(app_key, status, order_no, trade_no, user_id, pct_type, pct_id, pct_name, skip, take)
获取订单列表

根据查询条件获取订单列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**status** | Option<**String**> | 订单状态 |  |
**order_no** | Option<**String**> | 系统订单号 |  |
**trade_no** | Option<**String**> | 支付平台单号 |  |
**user_id** | Option<**i64**> | 用户ID |  |
**pct_type** | Option<**String**> | 商品类型 |  |
**pct_id** | Option<**String**> | 商品ID |  |
**pct_name** | Option<**String**> | 商品名称 |  |
**skip** | Option<**i32**> | 跳过的条数 |  |
**take** | Option<**i32**> | 拉取的条数 |  |

### Return type

[**models::OrderListResultApiResponse**](OrderListResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

