# \StorageApi

All URIs are relative to *https://api.zashigaofa.cn*

Method | HTTP request | Description
------------- | ------------- | -------------
[**storage_aggregate**](StorageApi.md#storage_aggregate) | **GET** /Storage/{appKey}/{table}/Aggregate | 聚合查询
[**storage_clear**](StorageApi.md#storage_clear) | **DELETE** /Storage/{appKey}/{table}/Clear | 清空表数据
[**storage_delete**](StorageApi.md#storage_delete) | **DELETE** /Storage/{appKey}/{table}/{id} | 删除数据
[**storage_delete_index**](StorageApi.md#storage_delete_index) | **DELETE** /Storage/{appKey}/{table}/indexes | 删除索引
[**storage_detail**](StorageApi.md#storage_detail) | **GET** /Storage/{appKey}/{table}/{id} | 数据详情
[**storage_execute_function**](StorageApi.md#storage_execute_function) | **GET** /Storage/{appKey}/ExecuteFunction | 执行函数
[**storage_export**](StorageApi.md#storage_export) | **GET** /Storage/{appKey}/{table}/Export | 导出数据
[**storage_import**](StorageApi.md#storage_import) | **POST** /Storage/{appKey}/{table}/Import | 导入数据
[**storage_indexes**](StorageApi.md#storage_indexes) | **GET** /Storage/{appKey}/{table}/Indexes | 获取索引
[**storage_list**](StorageApi.md#storage_list) | **GET** /Storage/{appKey}/{table} | 查询数据
[**storage_post**](StorageApi.md#storage_post) | **POST** /Storage/{appKey}/{table} | 添加数据
[**storage_post_index**](StorageApi.md#storage_post_index) | **POST** /Storage/{appKey}/{table}/indexes | 添加索引
[**storage_put**](StorageApi.md#storage_put) | **PUT** /Storage/{appKey}/{table}/{id} | 更新数据
[**storage_remove**](StorageApi.md#storage_remove) | **DELETE** /Storage/{appKey}/{table}/Remove | 删除表
[**storage_stats**](StorageApi.md#storage_stats) | **GET** /Storage/{appKey}/{table}/Stats | 数据表统计
[**storage_tables**](StorageApi.md#storage_tables) | **GET** /Storage/{appKey}/Tables | 获取数据表



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


## storage_clear

> models::Int64ApiResponse storage_clear(table, app_key, filter)
清空表数据

根据筛选条件清空指定表中的数据

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table** | **String** | 表名称 | [required] |
**app_key** | **String** |  | [required] |
**filter** | Option<**String**> | 筛选条件，json格式 |  |

### Return type

[**models::Int64ApiResponse**](Int64ApiResponse.md)

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


## storage_delete_index

> models::BooleanApiResponse storage_delete_index(table, app_key, index_name)
删除索引

删除指定表的指定索引

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table** | **String** | 表名称 | [required] |
**app_key** | **String** |  | [required] |
**index_name** | Option<**String**> | 索引名称 |  |

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


## storage_execute_function

> models::ObjectApiResponse storage_execute_function(nonce, timestamp, signature, app_key, execute_function_request)
执行函数

执行指定的函数

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nonce** | **String** | 随机字符串 | [required] |
**timestamp** | **i64** | 时间戳 | [required] |
**signature** | **String** | 签名 | [required] |
**app_key** | **String** |  | [required] |
**execute_function_request** | Option<[**ExecuteFunctionRequest**](ExecuteFunctionRequest.md)> | 函数请求参数 |  |

### Return type

[**models::ObjectApiResponse**](ObjectApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_export

> std::path::PathBuf storage_export(table, app_key, filter, start_time, end_time)
导出数据

根据筛选条件导出指定表中的数据

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table** | **String** | 表名称 | [required] |
**app_key** | **String** |  | [required] |
**filter** | Option<**String**> | 筛选条件，json格式 |  |
**start_time** | Option<**String**> | 开始时间 |  |
**end_time** | Option<**String**> | 结束时间 |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_import

> models::BooleanApiResponse storage_import(table, app_key, file)
导入数据

从文件导入数据到指定表中

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table** | **String** | 表名称 | [required] |
**app_key** | **String** |  | [required] |
**file** | Option<**std::path::PathBuf**> | 导入的文件 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_indexes

> models::ObjectListApiResponse storage_indexes(table, app_key)
获取索引

获取指定表的索引列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table** | **String** | 表名称 | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::ObjectListApiResponse**](ObjectListApiResponse.md)

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


## storage_post_index

> models::StringApiResponse storage_post_index(table, app_key, post_index_request)
添加索引

为指定表添加索引

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table** | **String** | 表名称（小写字母加数字,1-50位） | [required] |
**app_key** | **String** |  | [required] |
**post_index_request** | Option<[**PostIndexRequest**](PostIndexRequest.md)> | json对象 / json数组 |  |

### Return type

[**models::StringApiResponse**](StringApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_put

> models::BooleanApiResponse storage_put(table, id, app_key, body, replace)
更新数据

更新指定表中指定ID的数据，可以选择全量更新或部分更新

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table** | **String** | 表名称 | [required] |
**id** | **String** | 数据ID | [required] |
**app_key** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> | json格式 | [required] |
**replace** | Option<**bool**> | 是否为全量更新，默认为false |  |[default to false]

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_remove

> models::BooleanApiResponse storage_remove(table, app_key)
删除表

删除指定表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table** | **String** | 表名称 | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_stats

> models::ObjectApiResponse storage_stats(table, app_key)
数据表统计

获取指定表的数据统计信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table** | **String** | 表名称 | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::ObjectApiResponse**](ObjectApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_tables

> models::StringListApiResponse storage_tables(app_key)
获取数据表

获取当前应用的所有数据表名称

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |

### Return type

[**models::StringListApiResponse**](StringListApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

