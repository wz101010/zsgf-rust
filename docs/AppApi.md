# \AppApi

All URIs are relative to *https://api-dev.zashigaofa.cn*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_info**](AppApi.md#app_info) | **GET** /App/{appKey}/Info | 应用详情



## app_info

> models::AppInfoResultApiResponse app_info(app_key, prop_code)
应用详情

根据应用Key获取应用的详细信息和属性。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**prop_code** | Option<**String**> | 属性代码 |  |

### Return type

[**models::AppInfoResultApiResponse**](AppInfoResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

