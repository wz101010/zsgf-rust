# \SystemFileApi

All URIs are relative to *https://api.zashigaofa.cn*

Method | HTTP request | Description
------------- | ------------- | -------------
[**system_file_create_folder**](SystemFileApi.md#system_file_create_folder) | **POST** /SystemFile/CreateFolder | 创建文件夹
[**system_file_delete**](SystemFileApi.md#system_file_delete) | **DELETE** /SystemFile | 删除文件
[**system_file_rename**](SystemFileApi.md#system_file_rename) | **POST** /SystemFile/Rename | 重命名文件
[**system_file_upload**](SystemFileApi.md#system_file_upload) | **POST** /SystemFile | 上传文件
[**system_files**](SystemFileApi.md#system_files) | **GET** /SystemFile | 获取文件列表



## system_file_create_folder

> models::BooleanApiResponse system_file_create_folder(path)
创建文件夹

根据指定路径创建文件夹

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | Option<**String**> | 文件夹路径 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## system_file_delete

> models::BooleanApiResponse system_file_delete(path)
删除文件

根据指定路径删除文件

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | Option<**String**> | 文件路径 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## system_file_rename

> models::BooleanApiResponse system_file_rename(source_name, dest_name)
重命名文件

根据指定的源文件名和目标文件名重命名文件

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_name** | Option<**String**> | 源文件名 |  |
**dest_name** | Option<**String**> | 目标文件名 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## system_file_upload

> models::StringApiResponse system_file_upload(path, file)
上传文件

根据指定路径上传文件

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | Option<**String**> | 上传的路径 |  |
**file** | Option<**std::path::PathBuf**> | 上传的文件 |  |

### Return type

[**models::StringApiResponse**](StringApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## system_files

> models::SystemFileListResultApiResponse system_files(path)
获取文件列表

根据指定路径获取文件列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | Option<**String**> | 文件路径 |  |

### Return type

[**models::SystemFileListResultApiResponse**](SystemFileListResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

