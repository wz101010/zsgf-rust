# \FileApi

All URIs are relative to *https://api-dev.zashigaofa.cn*

Method | HTTP request | Description
------------- | ------------- | -------------
[**file_create_folder**](FileApi.md#file_create_folder) | **POST** /File/{appKey}/CreateFolder | 创建文件夹
[**file_delete**](FileApi.md#file_delete) | **DELETE** /File/{appKey} | 删除文件 / 文件夹
[**file_rename**](FileApi.md#file_rename) | **POST** /File/{appKey}/Rename | 重命名文件 / 文件夹
[**file_upload**](FileApi.md#file_upload) | **POST** /File/{appKey}/Upload | 上传文件
[**files**](FileApi.md#files) | **GET** /File/{appKey} | 获取文件列表



## file_create_folder

> models::BooleanApiResponse file_create_folder(app_key, path)
创建文件夹

在指定路径创建文件夹

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**path** | Option<**String**> | 文件夹路径 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## file_delete

> models::BooleanApiResponse file_delete(app_key, path)
删除文件 / 文件夹

根据指定路径删除文件或文件夹

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**path** | Option<**String**> | 文件 / 文件夹路径 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## file_rename

> models::BooleanApiResponse file_rename(app_key, source_name, dest_name)
重命名文件 / 文件夹

将指定的文件或文件夹重命名

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**source_name** | Option<**String**> | 原文件 / 文件夹名称 |  |
**dest_name** | Option<**String**> | 新文件 / 文件夹名称 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## file_upload

> models::StringApiResponse file_upload(app_key, path, file)
上传文件

将文件上传到指定路径

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**path** | Option<**String**> | 文件夹路径 |  |
**file** | Option<**std::path::PathBuf**> | 上传的文件 |  |

### Return type

[**models::StringApiResponse**](StringApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files

> models::FileListResultApiResponse files(app_key, path, skip, take)
获取文件列表

根据指定路径获取文件和文件夹列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**path** | Option<**String**> | 文件路径 |  |
**skip** | Option<**i32**> | 跳过的记录数 |  |[default to 0]
**take** | Option<**i32**> | 获取的记录数 |  |[default to 100]

### Return type

[**models::FileListResultApiResponse**](FileListResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

