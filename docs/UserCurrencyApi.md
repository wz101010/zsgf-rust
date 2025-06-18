# \UserCurrencyApi

All URIs are relative to *https://api-dev.zashigaofa.cn*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_currencies**](UserCurrencyApi.md#user_currencies) | **GET** /UserCurrency/{appKey}/{id} | 获取用户资产
[**user_currency_consume**](UserCurrencyApi.md#user_currency_consume) | **POST** /UserCurrency/{appKey}/CurrencyConsume | 消费虚拟币
[**user_currency_exchange**](UserCurrencyApi.md#user_currency_exchange) | **POST** /UserCurrency/{appKey}/CurrencyExchange | 兑换虚拟币
[**user_currency_recharge**](UserCurrencyApi.md#user_currency_recharge) | **POST** /UserCurrency/{appKey}/CurrencyRecharge | 充值虚拟币
[**user_currency_transactions**](UserCurrencyApi.md#user_currency_transactions) | **GET** /UserCurrency/{appKey}/CurrencyTransactions | 虚拟币交易记录



## user_currencies

> models::UserCurrencyListApiResponse user_currencies(id, app_key)
获取用户资产

根据用户ID获取用户的资产列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 用户ID | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::UserCurrencyListApiResponse**](UserCurrencyListApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_currency_consume

> models::BooleanApiResponse user_currency_consume(nonce, timestamp, signature, app_key, currency_consume_request)
消费虚拟币

根据提供的参数进行虚拟币消费

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nonce** | **String** | 随机数 | [required] |
**timestamp** | **i64** | 时间戳（允许与服务器时间误差在1分钟内） | [required] |
**signature** | **String** | 签名 | [required] |
**app_key** | **String** |  | [required] |
**currency_consume_request** | Option<[**CurrencyConsumeRequest**](CurrencyConsumeRequest.md)> | 消费请求参数 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_currency_exchange

> models::BooleanApiResponse user_currency_exchange(nonce, timestamp, signature, app_key, exchange_currency_request)
兑换虚拟币

根据提供的参数进行虚拟币兑换

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nonce** | **String** | 随机数 | [required] |
**timestamp** | **i64** | 时间戳（允许与服务器时间误差在1分钟内） | [required] |
**signature** | **String** | 签名 | [required] |
**app_key** | **String** |  | [required] |
**exchange_currency_request** | Option<[**ExchangeCurrencyRequest**](ExchangeCurrencyRequest.md)> | 兑换请求参数 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_currency_recharge

> models::BooleanApiResponse user_currency_recharge(nonce, timestamp, signature, app_key, recharge_point_request)
充值虚拟币

根据提供的参数进行虚拟币充值

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nonce** | **String** | 随机数 | [required] |
**timestamp** | **i64** | 时间戳（允许与服务器时间误差在1分钟内） | [required] |
**signature** | **String** | 签名 | [required] |
**app_key** | **String** |  | [required] |
**recharge_point_request** | Option<[**RechargePointRequest**](RechargePointRequest.md)> | 充值请求参数 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_currency_transactions

> models::UserCurrencyCurrencyTransResultApiResponse user_currency_transactions(app_key, trans_type, cur_code, start_time, end_time, skip, take)
虚拟币交易记录

根据提供的参数获取虚拟币交易记录

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**trans_type** | Option<**String**> | 交易类型 |  |
**cur_code** | Option<**String**> | 货币代码 |  |
**start_time** | Option<**String**> | 开始时间 |  |
**end_time** | Option<**String**> | 结束时间 |  |
**skip** | Option<**i32**> | 跳过的条数 |  |
**take** | Option<**i32**> | 拉取的条数 |  |

### Return type

[**models::UserCurrencyCurrencyTransResultApiResponse**](UserCurrencyCurrencyTransResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

