# \UserSettingApi

All URIs are relative to *https://api.zashigaofa.cn*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_setting**](UserSettingApi.md#user_setting) | **GET** /UserSetting/{appKey}/{id} | 获取用户配置项详情
[**user_setting_delete**](UserSettingApi.md#user_setting_delete) | **DELETE** /UserSetting/{appKey}/{id} | 删除用户配置项
[**user_setting_post**](UserSettingApi.md#user_setting_post) | **POST** /UserSetting/{appKey} | 添加用户配置项
[**user_setting_put**](UserSettingApi.md#user_setting_put) | **PUT** /UserSetting/{appKey}/{id} | 更新用户配置项
[**user_settings**](UserSettingApi.md#user_settings) | **GET** /UserSetting/{appKey} | 获取用户配置列表



## user_setting

> models::UserSettingApiResponse user_setting(id, app_key)
获取用户配置项详情

根据配置项ID获取用户配置项详情

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 配置项ID | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::UserSettingApiResponse**](UserSettingApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_setting_delete

> models::BooleanApiResponse user_setting_delete(id, app_key)
删除用户配置项

根据配置项ID删除用户配置项

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 配置项ID | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_setting_post

> models::UserSettingPostResultApiResponse user_setting_post(app_key, user_setting)
添加用户配置项

添加新的用户配置项

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**user_setting** | Option<[**UserSetting**](UserSetting.md)> | 配置项内容 |  |

### Return type

[**models::UserSettingPostResultApiResponse**](UserSettingPostResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_setting_put

> models::BooleanApiResponse user_setting_put(id, app_key, user_setting)
更新用户配置项

根据配置项ID更新用户配置项内容

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 配置项ID | [required] |
**app_key** | **String** |  | [required] |
**user_setting** | Option<[**UserSetting**](UserSetting.md)> | 配置项内容 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_settings

> models::UserSettingListApiResponse user_settings(app_key, user_id, code, tag)
获取用户配置列表

根据用户ID、配置项代码和标签获取用户配置列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**user_id** | Option<**i64**> | 用户ID |  |
**code** | Option<**String**> | 配置项代码 |  |
**tag** | Option<**String**> | 配置项标签 |  |

### Return type

[**models::UserSettingListApiResponse**](UserSettingListApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

