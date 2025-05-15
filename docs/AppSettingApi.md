# \AppSettingApi

All URIs are relative to *https://api.zashigaofa.cn*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_service_setting_group**](AppSettingApi.md#app_service_setting_group) | **GET** /AppSetting/{appKey}/Groups/{id} | 获取服务分组详情
[**app_service_setting_group_delete**](AppSettingApi.md#app_service_setting_group_delete) | **DELETE** /AppSetting/{appKey}/Groups/{id} | 删除服务分组
[**app_service_setting_group_post**](AppSettingApi.md#app_service_setting_group_post) | **POST** /AppSetting/{appKey}/Groups | 添加服务分组
[**app_service_setting_group_put**](AppSettingApi.md#app_service_setting_group_put) | **PUT** /AppSetting/{appKey}/Groups/{id} | 更新服务分组
[**app_service_setting_groups**](AppSettingApi.md#app_service_setting_groups) | **GET** /AppSetting/{appKey}/Groups | 获取服务分组列表
[**app_service_setting_item**](AppSettingApi.md#app_service_setting_item) | **GET** /AppSetting/{appKey}/Items/{id} | 服务配置项详情
[**app_service_setting_item_delete**](AppSettingApi.md#app_service_setting_item_delete) | **DELETE** /AppSetting/{appKey}/Items/{id} | 删除服务配置项
[**app_service_setting_item_post**](AppSettingApi.md#app_service_setting_item_post) | **POST** /AppSetting/{appKey}/Items | 添加服务配置项
[**app_service_setting_item_put**](AppSettingApi.md#app_service_setting_item_put) | **PUT** /AppSetting/{appKey}/Items/{id} | 更新服务配置项
[**app_service_setting_items**](AppSettingApi.md#app_service_setting_items) | **GET** /AppSetting/{appKey}/Items | 服务配置项列表
[**app_service_setting_provider**](AppSettingApi.md#app_service_setting_provider) | **GET** /AppSetting/{appKey}/Providers/{id} | 获取服务商详情
[**app_service_setting_provider_delete**](AppSettingApi.md#app_service_setting_provider_delete) | **DELETE** /AppSetting/{appKey}/Providers/{id} | 删除服务商
[**app_service_setting_provider_post**](AppSettingApi.md#app_service_setting_provider_post) | **POST** /AppSetting/{appKey}/Providers | 添加服务商
[**app_service_setting_provider_put**](AppSettingApi.md#app_service_setting_provider_put) | **PUT** /AppSetting/{appKey}/Providers/{id} | 更新服务商
[**app_service_setting_providers**](AppSettingApi.md#app_service_setting_providers) | **GET** /AppSetting/{appKey}/Providers | 获取服务商列表
[**app_setting**](AppSettingApi.md#app_setting) | **GET** /AppSetting/{appKey}/{id} | 配置详情
[**app_setting_delete**](AppSettingApi.md#app_setting_delete) | **DELETE** /AppSetting/{appKey}/{id} | 删除配置
[**app_setting_post**](AppSettingApi.md#app_setting_post) | **POST** /AppSetting/{appKey} | 增加配置
[**app_setting_put**](AppSettingApi.md#app_setting_put) | **PUT** /AppSetting/{appKey}/{id} | 更新配置
[**app_settings**](AppSettingApi.md#app_settings) | **GET** /AppSetting/{appKey} | 配置列表



## app_service_setting_group

> models::ServiceGroupApiResponse app_service_setting_group(id, app_key)
获取服务分组详情

根据服务分组ID获取服务分组详情

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 服务分组ID | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::ServiceGroupApiResponse**](ServiceGroupApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_service_setting_group_delete

> models::BooleanApiResponse app_service_setting_group_delete(id, app_key)
删除服务分组

根据服务分组ID删除服务分组

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 服务分组ID | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_service_setting_group_post

> models::AppSettingGroupPostResultApiResponse app_service_setting_group_post(app_key, service_group)
添加服务分组

添加新的服务分组信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**service_group** | Option<[**ServiceGroup**](ServiceGroup.md)> | 服务分组信息 |  |

### Return type

[**models::AppSettingGroupPostResultApiResponse**](AppSettingGroupPostResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_service_setting_group_put

> models::BooleanApiResponse app_service_setting_group_put(id, app_key, service_group)
更新服务分组

根据服务分组ID更新服务分组信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 服务分组ID | [required] |
**app_key** | **String** |  | [required] |
**service_group** | Option<[**ServiceGroup**](ServiceGroup.md)> | 服务分组信息 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_service_setting_groups

> models::ServiceGroupListApiResponse app_service_setting_groups(app_key, provider_id, show_flag)
获取服务分组列表

根据服务商ID和显示标志获取服务分组列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**provider_id** | Option<**i64**> | 服务商ID |  |
**show_flag** | Option<**bool**> | 是否显示 |  |[default to false]

### Return type

[**models::ServiceGroupListApiResponse**](ServiceGroupListApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_service_setting_item

> models::ServiceItemApiResponse app_service_setting_item(id, app_key)
服务配置项详情

根据服务配置项ID获取服务配置项详情

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 服务配置项ID | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::ServiceItemApiResponse**](ServiceItemApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_service_setting_item_delete

> models::BooleanApiResponse app_service_setting_item_delete(id, app_key)
删除服务配置项

根据服务配置项ID删除服务配置项

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 服务配置项ID | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_service_setting_item_post

> models::AppSettingItemPostResultApiResponse app_service_setting_item_post(app_key, service_item)
添加服务配置项

添加新的服务配置项信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**service_item** | Option<[**ServiceItem**](ServiceItem.md)> | 服务配置项信息 |  |

### Return type

[**models::AppSettingItemPostResultApiResponse**](AppSettingItemPostResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_service_setting_item_put

> models::BooleanApiResponse app_service_setting_item_put(id, app_key, service_item)
更新服务配置项

根据服务配置项ID更新服务配置项信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 服务配置项ID | [required] |
**app_key** | **String** |  | [required] |
**service_item** | Option<[**ServiceItem**](ServiceItem.md)> | 服务配置项信息 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_service_setting_items

> models::ServiceItemListApiResponse app_service_setting_items(app_key, biz_code, provider_code, group_code, show_flag)
服务配置项列表

根据业务代码、服务商代码、分组代码和显示标志获取服务配置项列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**biz_code** | Option<**String**> | 业务代码 |  |
**provider_code** | Option<**String**> | 服务商代码 |  |
**group_code** | Option<**String**> | 分组代码 |  |
**show_flag** | Option<**bool**> | 是否显示 |  |[default to false]

### Return type

[**models::ServiceItemListApiResponse**](ServiceItemListApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_service_setting_provider

> models::ServiceProviderApiResponse app_service_setting_provider(id, app_key)
获取服务商详情

根据服务商ID获取服务商详情

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 服务商ID | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::ServiceProviderApiResponse**](ServiceProviderApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_service_setting_provider_delete

> models::BooleanApiResponse app_service_setting_provider_delete(id, app_key)
删除服务商

根据服务商ID删除服务商

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 服务商ID | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_service_setting_provider_post

> models::AppSettingProviderPostResultApiResponse app_service_setting_provider_post(app_key, service_provider)
添加服务商

添加新的服务商信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**service_provider** | Option<[**ServiceProvider**](ServiceProvider.md)> | 服务商信息 |  |

### Return type

[**models::AppSettingProviderPostResultApiResponse**](AppSettingProviderPostResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_service_setting_provider_put

> models::BooleanApiResponse app_service_setting_provider_put(id, app_key, service_provider)
更新服务商

根据服务商ID更新服务商信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 服务商ID | [required] |
**app_key** | **String** |  | [required] |
**service_provider** | Option<[**ServiceProvider**](ServiceProvider.md)> | 服务商信息 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_service_setting_providers

> models::ServiceProviderListApiResponse app_service_setting_providers(app_key, biz_code, show_flag)
获取服务商列表

根据业务代码和显示标志获取服务商列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**biz_code** | Option<**String**> | 业务代码 |  |
**show_flag** | Option<**bool**> | 是否显示 |  |[default to false]

### Return type

[**models::ServiceProviderListApiResponse**](ServiceProviderListApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_setting

> models::AppSettingApiResponse app_setting(id, app_key)
配置详情

根据配置ID获取配置详情

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 配置ID | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::AppSettingApiResponse**](AppSettingApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_setting_delete

> models::BooleanApiResponse app_setting_delete(id, app_key)
删除配置

根据配置ID删除配置

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 配置ID | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_setting_post

> models::AppSettingSettingPostResultApiResponse app_setting_post(app_key, app_setting)
增加配置

添加新的配置内容

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**app_setting** | Option<[**AppSetting**](AppSetting.md)> | 配置内容 |  |

### Return type

[**models::AppSettingSettingPostResultApiResponse**](AppSettingSettingPostResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_setting_put

> models::BooleanApiResponse app_setting_put(id, app_key, app_setting)
更新配置

根据配置ID更新配置内容

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 配置ID | [required] |
**app_key** | **String** |  | [required] |
**app_setting** | Option<[**AppSetting**](AppSetting.md)> | 配置内容 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_settings

> models::AppSettingListApiResponse app_settings(app_key, provider_code, group_code, tag, code)
配置列表

根据服务商代码、分组代码、标签和配置项代码获取配置列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**provider_code** | Option<**String**> | 服务商代码 |  |
**group_code** | Option<**String**> | 分组代码 |  |
**tag** | Option<**String**> | 标签 |  |
**code** | Option<**String**> | 配置项代码 |  |

### Return type

[**models::AppSettingListApiResponse**](AppSettingListApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

