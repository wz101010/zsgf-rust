# \ProjectApi

All URIs are relative to *https://api.zashigaofa.cn*

Method | HTTP request | Description
------------- | ------------- | -------------
[**project**](ProjectApi.md#project) | **GET** /Project/{id} | 项目详情
[**project_delete**](ProjectApi.md#project_delete) | **DELETE** /Project/{id} | 删除项目
[**project_post**](ProjectApi.md#project_post) | **POST** /Project | 创建项目
[**project_put**](ProjectApi.md#project_put) | **PUT** /Project/{id} | 更新项目
[**projects**](ProjectApi.md#projects) | **GET** /Project | 项目列表



## project

> models::ProjectApiResponse project(id)
项目详情

根据项目ID获取项目详情

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |

### Return type

[**models::ProjectApiResponse**](ProjectApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_delete

> models::BooleanApiResponse project_delete(id)
删除项目

根据项目ID删除项目

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_post

> models::PostResultApiResponse project_post(project)
创建项目

创建一个新项目

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | Option<[**Project**](Project.md)> |  |  |

### Return type

[**models::PostResultApiResponse**](PostResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_put

> models::BooleanApiResponse project_put(id, project)
更新项目

根据项目ID更新项目

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**project** | Option<[**Project**](Project.md)> |  |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects

> models::ProjectListResultApiResponse projects(skip, take)
项目列表

获取项目列表，支持分页

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skip** | Option<**i32**> |  |  |
**take** | Option<**i32**> |  |  |

### Return type

[**models::ProjectListResultApiResponse**](ProjectListResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

