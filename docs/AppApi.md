# \AppApi

All URIs are relative to *https://api.zashigaofa.cn*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app**](AppApi.md#app) | **GET** /App/{appKey} | 应用详情
[**app2_fa**](AppApi.md#app2_fa) | **GET** /App/{appKey}/2FA | 双因素验证 获取
[**app2_fa_check**](AppApi.md#app2_fa_check) | **GET** /App/{appKey}/2FACheck | 双因素验证 验证
[**app_check_version**](AppApi.md#app_check_version) | **GET** /App/{appKey}/CheckVersion | 更新应用数据库
[**app_delete**](AppApi.md#app_delete) | **DELETE** /App/{appKey} | 删除应用
[**app_info**](AppApi.md#app_info) | **GET** /App/{appKey}/Info | 应用详情
[**app_post**](AppApi.md#app_post) | **POST** /App | 创建应用
[**app_put**](AppApi.md#app_put) | **PUT** /App/{appKey} | 更新应用
[**app_transfer**](AppApi.md#app_transfer) | **GET** /App/{appKey}/Transfer | 转移应用
[**apps**](AppApi.md#apps) | **GET** /App | 应用列表



## app

> models::AppApiResponse app(app_key)
应用详情

根据应用Key获取应用的详细信息。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |

### Return type

[**models::AppApiResponse**](AppApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app2_fa

> models::SetupCodeApiResponse app2_fa(app_key)
双因素验证 获取

获取应用的双因素验证信息。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |

### Return type

[**models::SetupCodeApiResponse**](SetupCodeApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app2_fa_check

> models::BooleanApiResponse app2_fa_check(app_key, code)
双因素验证 验证

验证应用的双因素验证代码。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**code** | Option<**String**> | 双因素验证代码 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_check_version

> models::AppCheckVersionResultApiResponse app_check_version(app_key, check_only)
更新应用数据库

检查应用数据库的版本，如果有未应用的迁移且checkOnly为false，则应用这些迁移。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**check_only** | Option<**bool**> | 是否仅检查版本 |  |[default to true]

### Return type

[**models::AppCheckVersionResultApiResponse**](AppCheckVersionResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_delete

> models::BooleanApiResponse app_delete(app_key)
删除应用

根据应用Key删除应用。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## app_post

> models::AppPostResultApiResponse app_post(app)
创建应用

创建一个新的应用。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app** | Option<[**App**](App.md)> | 应用对象 |  |

### Return type

[**models::AppPostResultApiResponse**](AppPostResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_put

> models::BooleanApiResponse app_put(app_key, app)
更新应用

根据应用Key更新应用信息。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**app** | Option<[**App**](App.md)> | 应用对象 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_transfer

> models::AppApiResponse app_transfer(app_key, project_id)
转移应用

将应用转移到另一个项目。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**project_id** | Option<**i64**> | 目标项目ID |  |

### Return type

[**models::AppApiResponse**](AppApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps

> models::AppListResultApiResponse apps(project_id, skip, take)
应用列表

根据项目ID获取应用列表，支持分页。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | Option<**i64**> | 项目ID |  |
**skip** | Option<**i32**> | 跳过的记录数 |  |
**take** | Option<**i32**> | 获取的记录数 |  |

### Return type

[**models::AppListResultApiResponse**](AppListResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

