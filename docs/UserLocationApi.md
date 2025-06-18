# \UserLocationApi

All URIs are relative to *https://api-dev.zashigaofa.cn*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_location**](UserLocationApi.md#user_location) | **GET** /UserLocation/{appKey}/{id} | 获取位置详情
[**user_location_delete**](UserLocationApi.md#user_location_delete) | **DELETE** /UserLocation/{appKey}/{id} | 删除位置
[**user_location_post**](UserLocationApi.md#user_location_post) | **POST** /UserLocation/{appKey} | 添加位置
[**user_location_put**](UserLocationApi.md#user_location_put) | **PUT** /UserLocation/{appKey}/{id} | 更新位置
[**user_locations**](UserLocationApi.md#user_locations) | **GET** /UserLocation/{appKey} | 获取位置列表



## user_location

> models::GeoLocationModelApiResponse user_location(id, app_key)
获取位置详情

根据位置ID获取位置详情

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 位置ID | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::GeoLocationModelApiResponse**](GeoLocationModelApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_location_delete

> models::BooleanApiResponse user_location_delete(id, app_key)
删除位置

根据位置ID删除位置信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 位置ID | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_location_post

> models::UserLocationPostResultApiResponse user_location_post(app_key, geo_location_model)
添加位置

添加新的位置信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**geo_location_model** | Option<[**GeoLocationModel**](GeoLocationModel.md)> | 位置信息 |  |

### Return type

[**models::UserLocationPostResultApiResponse**](UserLocationPostResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_location_put

> models::BooleanApiResponse user_location_put(id, app_key, geo_location_model)
更新位置

此方法用于更新指定位置ID的位置信息。如果位置不存在，则创建一个新的位置。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 位置ID | [required] |
**app_key** | **String** |  | [required] |
**geo_location_model** | Option<[**GeoLocationModel**](GeoLocationModel.md)> | 位置信息 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_locations

> models::UserLocationsResultApiResponse user_locations(app_key, tag, r#type, x, y, sphere, skip, take)
获取位置列表

根据条件获取位置列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**tag** | Option<**String**> | 标签 |  |
**r#type** | Option<**String**> | 分类 |  |
**x** | Option<**f64**> | 纬度 |  |
**y** | Option<**f64**> | 经度 |  |
**sphere** | Option<**i64**> | 附近距离，单位：米 |  |
**skip** | Option<**i32**> | 跳过的记录数 |  |
**take** | Option<**i32**> | 获取的记录数 |  |[default to 10]

### Return type

[**models::UserLocationsResultApiResponse**](UserLocationsResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

