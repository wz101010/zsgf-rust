# \TeamApi

All URIs are relative to *https://api.zashigaofa.cn*

Method | HTTP request | Description
------------- | ------------- | -------------
[**team_delete**](TeamApi.md#team_delete) | **DELETE** /Team/{id} | 删除团队
[**team_post**](TeamApi.md#team_post) | **POST** /Team | 创建团队
[**team_put**](TeamApi.md#team_put) | **PUT** /Team/{id} | 更新团队信息
[**teams**](TeamApi.md#teams) | **GET** /Team | 获取团队列表



## team_delete

> models::BooleanApiResponse team_delete(id)
删除团队

根据团队ID删除团队

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 团队ID | [required] |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_post

> models::BooleanApiResponse team_post(team)
创建团队

创建一个新的团队

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team** | Option<[**Team**](Team.md)> | 团队信息 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_put

> models::BooleanApiResponse team_put(id, team)
更新团队信息

根据团队ID更新团队信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 团队ID | [required] |
**team** | Option<[**Team**](Team.md)> | 团队信息 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams

> models::ListResponseItemListApiResponse teams(channel_code, channel_app_id)
获取团队列表

根据渠道代码和渠道应用ID获取团队列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_code** | Option<**String**> | 渠道代码 |  |
**channel_app_id** | Option<**String**> | 渠道应用ID |  |

### Return type

[**models::ListResponseItemListApiResponse**](ListResponseItemListApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

