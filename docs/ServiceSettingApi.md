# \ServiceSettingApi

All URIs are relative to *https://api.zashigaofa.cn*

Method | HTTP request | Description
------------- | ------------- | -------------
[**service_setting**](ServiceSettingApi.md#service_setting) | **GET** /ServiceSetting/{id} | 获取配置详情
[**service_setting_delete**](ServiceSettingApi.md#service_setting_delete) | **DELETE** /ServiceSetting/{id} | 删除配置
[**service_setting_group**](ServiceSettingApi.md#service_setting_group) | **GET** /ServiceSetting/Groups/{id} | 获取服务分组详情
[**service_setting_group_delete**](ServiceSettingApi.md#service_setting_group_delete) | **DELETE** /ServiceSetting/Groups/{id} | 删除服务分组
[**service_setting_group_post**](ServiceSettingApi.md#service_setting_group_post) | **POST** /ServiceSetting/Groups | 添加服务分组
[**service_setting_group_put**](ServiceSettingApi.md#service_setting_group_put) | **PUT** /ServiceSetting/Groups/{id} | 更新服务分组
[**service_setting_groups**](ServiceSettingApi.md#service_setting_groups) | **GET** /ServiceSetting/Groups | 获取服务分组列表
[**service_setting_item**](ServiceSettingApi.md#service_setting_item) | **GET** /ServiceSetting/Items/{id} | 服务配置详情
[**service_setting_item_delete**](ServiceSettingApi.md#service_setting_item_delete) | **DELETE** /ServiceSetting/Items/{id} | 删除服务配置
[**service_setting_item_post**](ServiceSettingApi.md#service_setting_item_post) | **POST** /ServiceSetting/Items | 添加服务配置
[**service_setting_item_put**](ServiceSettingApi.md#service_setting_item_put) | **PUT** /ServiceSetting/Items/{id} | 更新服务配置
[**service_setting_items**](ServiceSettingApi.md#service_setting_items) | **GET** /ServiceSetting/Items | 服务配置列表
[**service_setting_post**](ServiceSettingApi.md#service_setting_post) | **POST** /ServiceSetting | 增加配置
[**service_setting_provider**](ServiceSettingApi.md#service_setting_provider) | **GET** /ServiceSetting/Providers/{id} | 获取服务商详情
[**service_setting_provider_delete**](ServiceSettingApi.md#service_setting_provider_delete) | **DELETE** /ServiceSetting/Providers/{id} | 删除服务商
[**service_setting_provider_post**](ServiceSettingApi.md#service_setting_provider_post) | **POST** /ServiceSetting/Providers | 添加服务商
[**service_setting_provider_put**](ServiceSettingApi.md#service_setting_provider_put) | **PUT** /ServiceSetting/Providers/{id} | 更新服务商
[**service_setting_providers**](ServiceSettingApi.md#service_setting_providers) | **GET** /ServiceSetting/Providers | 获取服务商列表
[**service_setting_put**](ServiceSettingApi.md#service_setting_put) | **PUT** /ServiceSetting/{id} | 更新配置
[**service_settings**](ServiceSettingApi.md#service_settings) | **GET** /ServiceSetting | 获取配置列表



## service_setting

> models::SettingsApiResponse service_setting(id)
获取配置详情

根据配置ID获取配置详情

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 配置ID | [required] |

### Return type

[**models::SettingsApiResponse**](SettingsApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_setting_delete

> models::BooleanApiResponse service_setting_delete(id)
删除配置

根据配置ID删除配置

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 配置ID | [required] |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_setting_group

> models::ServiceGroupApiResponse service_setting_group(id)
获取服务分组详情

根据服务分组ID获取服务分组详情

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 服务分组ID | [required] |

### Return type

[**models::ServiceGroupApiResponse**](ServiceGroupApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_setting_group_delete

> models::BooleanApiResponse service_setting_group_delete(id)
删除服务分组

根据服务分组ID删除服务分组

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 服务分组ID | [required] |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_setting_group_post

> models::ServiceSettingGroupPostResultApiResponse service_setting_group_post(service_group)
添加服务分组

添加新的服务分组

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_group** | Option<[**ServiceGroup**](ServiceGroup.md)> | 服务分组实体 |  |

### Return type

[**models::ServiceSettingGroupPostResultApiResponse**](ServiceSettingGroupPostResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_setting_group_put

> models::BooleanApiResponse service_setting_group_put(id, service_group)
更新服务分组

根据服务分组ID更新服务分组信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 服务分组ID | [required] |
**service_group** | Option<[**ServiceGroup**](ServiceGroup.md)> | 服务分组实体 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_setting_groups

> models::ServiceGroupListApiResponse service_setting_groups(provider_id, show_flag)
获取服务分组列表

根据服务商ID和显示标志获取服务分组列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## service_setting_item

> models::ServiceItemApiResponse service_setting_item(id)
服务配置详情

根据服务配置ID获取服务配置详情

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 服务配置ID | [required] |

### Return type

[**models::ServiceItemApiResponse**](ServiceItemApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_setting_item_delete

> models::BooleanApiResponse service_setting_item_delete(id)
删除服务配置

根据服务配置ID删除服务配置

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 服务配置ID | [required] |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_setting_item_post

> models::ServiceSettingItemPostResultApiResponse service_setting_item_post(service_item)
添加服务配置

添加新的服务配置

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_item** | Option<[**ServiceItem**](ServiceItem.md)> | 服务配置实体 |  |

### Return type

[**models::ServiceSettingItemPostResultApiResponse**](ServiceSettingItemPostResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_setting_item_put

> models::BooleanApiResponse service_setting_item_put(id, service_item)
更新服务配置

根据服务配置ID更新服务配置信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 服务配置ID | [required] |
**service_item** | Option<[**ServiceItem**](ServiceItem.md)> | 服务配置实体 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_setting_items

> models::ServiceItemListApiResponse service_setting_items(biz_code, provider_code, group_code, show_flag)
服务配置列表

根据业务代码、服务商代码、分组代码和显示标志获取服务配置列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## service_setting_post

> models::ServiceSettingSettingPostResultApiResponse service_setting_post(settings)
增加配置

添加新的配置

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**settings** | Option<[**Settings**](Settings.md)> | 配置实体 |  |

### Return type

[**models::ServiceSettingSettingPostResultApiResponse**](ServiceSettingSettingPostResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_setting_provider

> models::ServiceProviderApiResponse service_setting_provider(id)
获取服务商详情

根据服务商ID获取服务商详情

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 服务商ID | [required] |

### Return type

[**models::ServiceProviderApiResponse**](ServiceProviderApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_setting_provider_delete

> models::BooleanApiResponse service_setting_provider_delete(id)
删除服务商

根据服务商ID删除服务商

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 服务商ID | [required] |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_setting_provider_post

> models::ServiceSettingProviderPostResultApiResponse service_setting_provider_post(service_provider)
添加服务商

添加新的服务商

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_provider** | Option<[**ServiceProvider**](ServiceProvider.md)> | 服务商实体 |  |

### Return type

[**models::ServiceSettingProviderPostResultApiResponse**](ServiceSettingProviderPostResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_setting_provider_put

> models::BooleanApiResponse service_setting_provider_put(id, service_provider)
更新服务商

根据服务商ID更新服务商信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 服务商ID | [required] |
**service_provider** | Option<[**ServiceProvider**](ServiceProvider.md)> | 服务商实体 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_setting_providers

> models::ServiceProviderListApiResponse service_setting_providers(biz_code, show_flag)
获取服务商列表

根据业务代码和显示标志获取服务商列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## service_setting_put

> models::BooleanApiResponse service_setting_put(id, settings)
更新配置

根据配置ID更新配置信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 配置ID | [required] |
**settings** | Option<[**Settings**](Settings.md)> | 配置实体 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_settings

> models::SettingsListApiResponse service_settings(biz_code, biz_id, provider_code, group_code, tag, code)
获取配置列表

根据业务代码、业务标识、服务商代码、分组代码、标签和配置项代码获取配置列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**biz_code** | **String** | 业务代码 | [required] |
**biz_id** | **String** | 业务标识 | [required] |
**provider_code** | Option<**String**> | 服务商代码 |  |
**group_code** | Option<**String**> | 分组代码 |  |
**tag** | Option<**String**> | 标签 |  |
**code** | Option<**String**> | 配置项代码 |  |

### Return type

[**models::SettingsListApiResponse**](SettingsListApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

