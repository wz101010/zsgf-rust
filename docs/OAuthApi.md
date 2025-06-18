# \OAuthApi

All URIs are relative to *https://api-dev.zashigaofa.cn*

Method | HTTP request | Description
------------- | ------------- | -------------
[**o_auth_authorize**](OAuthApi.md#o_auth_authorize) | **POST** /OAuth/{appKey}/Authorize | 获取访问令牌
[**o_auth_consents**](OAuthApi.md#o_auth_consents) | **GET** /OAuth/{appKey}/Consents | 获取授权记录
[**o_auth_delete_consent**](OAuthApi.md#o_auth_delete_consent) | **DELETE** /OAuth/{appKey}/Consents/{id} | 删除授权记录
[**o_auth_grant_code**](OAuthApi.md#o_auth_grant_code) | **POST** /OAuth/{appKey}/GrantCode | 获取授权码
[**o_auth_profile**](OAuthApi.md#o_auth_profile) | **GET** /OAuth/{appKey}/Profile | 获取用户资料



## o_auth_authorize

> models::AuthorizeResultApiResponse o_auth_authorize(app_key, scheme, code)
获取访问令牌

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**scheme** | Option<**String**> | 身份源 |  |
**code** | Option<**String**> | 授权码 |  |

### Return type

[**models::AuthorizeResultApiResponse**](AuthorizeResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## o_auth_consents

> models::AppUserConsentModelListApiResponse o_auth_consents(app_key)
获取授权记录

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |

### Return type

[**models::AppUserConsentModelListApiResponse**](AppUserConsentModelListApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## o_auth_delete_consent

> models::BooleanApiResponse o_auth_delete_consent(id, app_key)
删除授权记录

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


## o_auth_grant_code

> models::GrantResultApiResponse o_auth_grant_code(app_key, scheme, grant_request)
获取授权码

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**scheme** | Option<**String**> | 身份源，固定填：app |  |
**grant_request** | Option<[**GrantRequest**](GrantRequest.md)> | 授权详情 |  |

### Return type

[**models::GrantResultApiResponse**](GrantResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## o_auth_profile

> models::ProfileResultApiResponse o_auth_profile(app_key)
获取用户资料

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |

### Return type

[**models::ProfileResultApiResponse**](ProfileResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

