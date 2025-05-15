# \AuthorizePolicyApi

All URIs are relative to *https://api.zashigaofa.cn*

Method | HTTP request | Description
------------- | ------------- | -------------
[**authorize_policies**](AuthorizePolicyApi.md#authorize_policies) | **GET** /AuthorizePolicy/{appKey} | 获取鉴权策略列表
[**authorize_policy**](AuthorizePolicyApi.md#authorize_policy) | **GET** /AuthorizePolicy/{appKey}/{id} | 获取鉴权策略详情
[**authorize_policy_delete**](AuthorizePolicyApi.md#authorize_policy_delete) | **DELETE** /AuthorizePolicy/{appKey}/{id} | 删除鉴权策略
[**authorize_policy_post**](AuthorizePolicyApi.md#authorize_policy_post) | **POST** /AuthorizePolicy/{appKey} | 添加鉴权策略
[**authorize_policy_put**](AuthorizePolicyApi.md#authorize_policy_put) | **PUT** /AuthorizePolicy/{appKey}/{id} | 更新鉴权策略



## authorize_policies

> models::AuthorizePolicyListApiResponse authorize_policies(app_key, auth_type, policy_name)
获取鉴权策略列表

根据鉴权类型和策略名称获取鉴权策略列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**auth_type** | Option<**String**> | 鉴权类型（access_token、user、role） |  |
**policy_name** | Option<**String**> | 策略名称 |  |

### Return type

[**models::AuthorizePolicyListApiResponse**](AuthorizePolicyListApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## authorize_policy

> models::AuthorizePolicyApiResponse authorize_policy(id, app_key)
获取鉴权策略详情

根据鉴权策略ID获取鉴权策略详情

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 鉴权策略ID | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::AuthorizePolicyApiResponse**](AuthorizePolicyApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## authorize_policy_delete

> models::BooleanApiResponse authorize_policy_delete(id, app_key)
删除鉴权策略

根据鉴权策略ID删除鉴权策略

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 鉴权策略ID | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## authorize_policy_post

> models::CreatePostResultApiResponse authorize_policy_post(app_key, authorize_policy)
添加鉴权策略

添加新的鉴权策略

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**authorize_policy** | Option<[**AuthorizePolicy**](AuthorizePolicy.md)> | 鉴权策略对象 |  |

### Return type

[**models::CreatePostResultApiResponse**](CreatePostResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## authorize_policy_put

> models::BooleanApiResponse authorize_policy_put(id, app_key, authorize_policy)
更新鉴权策略

根据鉴权策略ID更新鉴权策略

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 鉴权策略ID | [required] |
**app_key** | **String** |  | [required] |
**authorize_policy** | Option<[**AuthorizePolicy**](AuthorizePolicy.md)> | 鉴权策略对象 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

