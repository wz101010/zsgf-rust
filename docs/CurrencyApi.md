# \CurrencyApi

All URIs are relative to *https://api.zashigaofa.cn*

Method | HTTP request | Description
------------- | ------------- | -------------
[**currencies**](CurrencyApi.md#currencies) | **GET** /Currency/{appKey} | 获取货币列表
[**currency**](CurrencyApi.md#currency) | **GET** /Currency/{appKey}/{id} | 获取货币详情
[**currency_delete**](CurrencyApi.md#currency_delete) | **DELETE** /Currency/{appKey}/{id} | 删除货币
[**currency_exchange_rate_delete**](CurrencyApi.md#currency_exchange_rate_delete) | **DELETE** /Currency/{appKey}/ExchangeRates/{id} | 删除汇率
[**currency_exchange_rate_put**](CurrencyApi.md#currency_exchange_rate_put) | **PUT** /Currency/{appKey}/ExchangeRates/{code} | 更新汇率
[**currency_exchange_rates**](CurrencyApi.md#currency_exchange_rates) | **GET** /Currency/{appKey}/ExchangeRates/{code} | 获取汇率列表
[**currency_post**](CurrencyApi.md#currency_post) | **POST** /Currency/{appKey} | 创建新货币
[**currency_put**](CurrencyApi.md#currency_put) | **PUT** /Currency/{appKey}/{id} | 更新货币信息
[**currency_transactions**](CurrencyApi.md#currency_transactions) | **GET** /Currency/{appKey}/Transactions | 获取货币交易记录



## currencies

> models::CurrencyListApiResponse currencies(app_key)
获取货币列表

获取所有货币的列表，按ID降序排列。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |

### Return type

[**models::CurrencyListApiResponse**](CurrencyListApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## currency

> models::CurrencyApiResponse currency(id, app_key)
获取货币详情

根据货币ID获取货币的详细信息。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 货币ID | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::CurrencyApiResponse**](CurrencyApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## currency_delete

> models::BooleanApiResponse currency_delete(id, app_key)
删除货币

根据货币ID删除货币。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 货币ID | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## currency_exchange_rate_delete

> models::BooleanApiResponse currency_exchange_rate_delete(id, app_key)
删除汇率

根据汇率ID删除汇率。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 汇率ID | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## currency_exchange_rate_put

> models::Int64ApiResponse currency_exchange_rate_put(code, app_key, exchange_rate_put_request)
更新汇率

根据货币代码更新汇率信息。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | 货币代码 | [required] |
**app_key** | **String** |  | [required] |
**exchange_rate_put_request** | Option<[**ExchangeRatePutRequest**](ExchangeRatePutRequest.md)> | 汇率信息 |  |

### Return type

[**models::Int64ApiResponse**](Int64ApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## currency_exchange_rates

> models::CurrencyExchangeRateApiResponse currency_exchange_rates(code, app_key)
获取汇率列表

根据货币代码获取该货币的汇率列表。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | 货币代码 | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::CurrencyExchangeRateApiResponse**](CurrencyExchangeRateApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## currency_post

> models::Int64ApiResponse currency_post(app_key, currency)
创建新货币

创建一个新的货币并返回其ID。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**currency** | Option<[**Currency**](Currency.md)> | 货币信息 |  |

### Return type

[**models::Int64ApiResponse**](Int64ApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## currency_put

> models::BooleanApiResponse currency_put(id, app_key, currency)
更新货币信息

根据货币ID更新货币的详细信息。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 货币ID | [required] |
**app_key** | **String** |  | [required] |
**currency** | Option<[**Currency**](Currency.md)> | 货币信息 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## currency_transactions

> models::CurrencyTransactionListApiResponse currency_transactions(app_key, user_id, trans_type, cur_code, start_time, end_time, skip, take)
获取货币交易记录

根据用户ID、交易类型、货币代码、时间范围等条件获取货币交易记录。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**user_id** | Option<**i64**> | 用户ID |  |
**trans_type** | Option<**String**> | 交易类型 |  |
**cur_code** | Option<**String**> | 货币代码 |  |
**start_time** | Option<**String**> | 开始时间 |  |
**end_time** | Option<**String**> | 结束时间 |  |
**skip** | Option<**i32**> | 跳过的条数 |  |
**take** | Option<**i32**> | 拉取的条数 |  |

### Return type

[**models::CurrencyTransactionListApiResponse**](CurrencyTransactionListApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

