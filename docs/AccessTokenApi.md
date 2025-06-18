# \AccessTokenApi

All URIs are relative to *https://api-dev.zashigaofa.cn*

Method | HTTP request | Description
------------- | ------------- | -------------
[**access_token_delete**](AccessTokenApi.md#access_token_delete) | **DELETE** /AccessToken/{appKey}/{id} | 删除令牌
[**access_token_post**](AccessTokenApi.md#access_token_post) | **POST** /AccessToken/{appKey} | 创建令牌
[**access_token_put**](AccessTokenApi.md#access_token_put) | **PUT** /AccessToken/{appKey}/{id} | 更新令牌
[**access_tokens**](AccessTokenApi.md#access_tokens) | **GET** /AccessToken/{appKey} | 令牌列表



## access_token_delete

> models::BooleanApiResponse access_token_delete(id, app_key)
删除令牌

删除用户令牌

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## access_token_post

> models::TokenModelApiResponse access_token_post(app_key, access_token_post_request)
创建令牌

创建新的用户令牌

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**access_token_post_request** | Option<[**AccessTokenPostRequest**](AccessTokenPostRequest.md)> |  |  |

### Return type

[**models::TokenModelApiResponse**](TokenModelApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## access_token_put

> models::BooleanApiResponse access_token_put(id, app_key, access_token_put_request)
更新令牌

更新现有的用户令牌

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |
**app_key** | **String** |  | [required] |
**access_token_put_request** | Option<[**AccessTokenPutRequest**](AccessTokenPutRequest.md)> |  |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## access_tokens

> models::AccessTokenListResultApiResponse access_tokens(app_key, user_id, skip, take)
令牌列表

获取用户令牌列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**user_id** | Option<**i64**> |  |  |
**skip** | Option<**i32**> |  |  |
**take** | Option<**i32**> |  |  |

### Return type

[**models::AccessTokenListResultApiResponse**](AccessTokenListResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

