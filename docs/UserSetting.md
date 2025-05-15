# UserSetting

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | 用户的唯一标识符。 | [optional]
**user_id** | **i64** | 关联的用户ID，表示该配置属于哪个用户。 | 
**provider_code** | Option<**String**> | 提供商的唯一代码，用于标识服务提供者。 | [optional]
**group_code** | Option<**String**> | 组的唯一代码，用于分类设置项。 | [optional]
**code** | **String** | 设置项的唯一代码或键名，用于标识具体的配置项。 | 
**value** | **String** | 设置项的具体值或选项。 | 
**tags** | Option<**String**> | 用于对设置项进行分类或标记的标签。 | [optional]
**description** | Option<**String**> | 设置项的详细描述，说明其作用或用途。 | [optional]
**frontend_usable** | Option<**bool**> | 指示该设置项是否在前端界面中可用。 | [optional]
**create_date** | Option<**String**> | 设置项的创建时间。 | [optional]
**last_update** | Option<**String**> | 设置项的最后更新时间。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


