# \ExternalAccountApi

All URIs are relative to *https://api-dev.zashigaofa.cn*

Method | HTTP request | Description
------------- | ------------- | -------------
[**external_account_sign_in**](ExternalAccountApi.md#external_account_sign_in) | **POST** /ExternalAccount/{appKey}/SignIn | 外部账号登录
[**user_external_account_bind**](ExternalAccountApi.md#user_external_account_bind) | **POST** /ExternalAccount/{appKey} | 绑定外部账号
[**user_o_auth_accounts**](ExternalAccountApi.md#user_o_auth_accounts) | **GET** /ExternalAccount/{appKey} | 外部账号列表
[**user_o_auth_accounts_put_bind**](ExternalAccountApi.md#user_o_auth_accounts_put_bind) | **PUT** /ExternalAccount/{appKey}/{id} | 更新绑定账号
[**user_o_auth_accounts_un_bind**](ExternalAccountApi.md#user_o_auth_accounts_un_bind) | **DELETE** /ExternalAccount/{appKey}/{id} | 删除绑定账号



## external_account_sign_in

> models::TokenModelApiResponse external_account_sign_in(app_key, external_account_sign_in_request)
外部账号登录

使用外部账号登录应用

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**external_account_sign_in_request** | Option<[**ExternalAccountSignInRequest**](ExternalAccountSignInRequest.md)> | 登录请求参数 |  |

### Return type

[**models::TokenModelApiResponse**](TokenModelApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_external_account_bind

> models::BooleanApiResponse user_external_account_bind(app_key, external_account_bind_request)
绑定外部账号

绑定外部账号，如果已存在绑定则直接返回成功

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**external_account_bind_request** | Option<[**ExternalAccountBindRequest**](ExternalAccountBindRequest.md)> | 绑定请求参数 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_o_auth_accounts

> models::UserLoginsListApiResponse user_o_auth_accounts(app_key)
外部账号列表

获取绑定成功的外部账号列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |

### Return type

[**models::UserLoginsListApiResponse**](UserLoginsListApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_o_auth_accounts_put_bind

> models::BooleanApiResponse user_o_auth_accounts_put_bind(id, app_key, external_account_put_request)
更新绑定账号

更新绑定的账号信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 绑定ID | [required] |
**app_key** | **String** |  | [required] |
**external_account_put_request** | Option<[**ExternalAccountPutRequest**](ExternalAccountPutRequest.md)> | 更新请求参数 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_o_auth_accounts_un_bind

> models::BooleanApiResponse user_o_auth_accounts_un_bind(id, app_key)
删除绑定账号

删除绑定的外部账号

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 绑定ID | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

