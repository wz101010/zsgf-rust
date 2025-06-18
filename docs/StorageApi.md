# \StorageApi

All URIs are relative to *https://api-dev.zashigaofa.cn*

Method | HTTP request | Description
------------- | ------------- | -------------
[**storage_aggregate**](StorageApi.md#storage_aggregate) | **GET** /Storage/{appKey}/{table}/Aggregate | 聚合查询
[**storage_delete**](StorageApi.md#storage_delete) | **DELETE** /Storage/{appKey}/{table}/{id} | 删除数据
[**storage_detail**](StorageApi.md#storage_detail) | **GET** /Storage/{appKey}/{table}/{id} | 数据详情
[**storage_list**](StorageApi.md#storage_list) | **GET** /Storage/{appKey}/{table} | 查询数据
[**storage_post**](StorageApi.md#storage_post) | **POST** /Storage/{appKey}/{table} | 添加数据
[**storage_put**](StorageApi.md#storage_put) | **PUT** /Storage/{appKey}/{table}/{id} | 更新数据



## storage_aggregate

> models::ObjectListApiResponse storage_aggregate(table, app_key, pipeline)
聚合查询

根据聚合管道查询指定表中的数据

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table** | **String** | 表名称 | [required] |
**app_key** | **String** |  | [required] |
**pipeline** | Option<**String**> | 构建聚合查询 |  |

### Return type

[**models::ObjectListApiResponse**](ObjectListApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_delete

> models::BooleanApiResponse storage_delete(table, id, app_key)
删除数据

删除指定表中指定ID的数据

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table** | **String** | 表名称 | [required] |
**id** | **String** | 数据ID | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_detail

> models::ObjectApiResponse storage_detail(table, id, app_key, project)
数据详情

获取指定表中指定ID的数据详情

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table** | **String** | 表名称 | [required] |
**id** | **String** | 数据ID | [required] |
**app_key** | **String** |  | [required] |
**project** | Option<**String**> | json格式 |  |

### Return type

[**models::ObjectApiResponse**](ObjectApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_list

> models::StorageListResultApiResponse storage_list(table, app_key, filter, project, sort, start_time, end_time, explain, take, skip)
查询数据

根据条件查询指定表中的数据

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table** | **String** | 表名称 | [required] |
**app_key** | **String** |  | [required] |
**filter** | Option<**String**> | 过滤，json格式 |  |
**project** | Option<**String**> | 字段映射，json格式 |  |
**sort** | Option<**String**> | 排序，json格式 |  |
**start_time** | Option<**String**> | 开始时间 |  |
**end_time** | Option<**String**> | 结束时间 |  |
**explain** | Option<**bool**> | 查看执行计划 |  |[default to false]
**take** | Option<**i32**> | 默认为10 |  |[default to 10]
**skip** | Option<**i32**> | 默认为0 |  |[default to 0]

### Return type

[**models::StorageListResultApiResponse**](StorageListResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_post

> models::StringApiResponse storage_post(table, app_key, request_body)
添加数据

向指定表中添加数据，可以是单个json对象或json数组

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table** | **String** | 表名称（小写字母加数字,1-50位） | [required] |
**app_key** | **String** |  | [required] |
**request_body** | [**Vec<serde_json::Value>**](serde_json::Value.md) | json对象 / json数组 | [required] |

### Return type

[**models::StringApiResponse**](StringApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_put

> models::BooleanApiResponse storage_put(table, id, app_key, request_body, replace)
更新数据

更新指定表中指定ID的数据，可以选择全量更新或部分更新

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table** | **String** | 表名称 | [required] |
**id** | **String** | 数据ID | [required] |
**app_key** | **String** |  | [required] |
**request_body** | [**Vec<serde_json::Value>**](serde_json::Value.md) | json格式 | [required] |
**replace** | Option<**bool**> | 是否为全量更新，默认为false |  |[default to false]

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

