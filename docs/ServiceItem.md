# ServiceItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | 服务配置项的唯一标识符。 | [optional]
**biz_code** | Option<**String**> | 服务配置项所属的业务代码，用于分类管理。 | [optional]
**provider_code** | Option<**String**> | 关联的服务商代码，用于标识提供该配置项的服务商。 | [optional]
**group_code** | Option<**String**> | 服务配置项所属的功能分组代码，用于组织和管理相关配置项。 | [optional]
**name** | Option<**String**> | 服务配置项的名称，用于描述其功能或用途。 | [optional]
**code** | Option<**String**> | 服务配置项的唯一代码，用于系统内部标识。 | [optional]
**value_type** | Option<**String**> | 服务配置项的值类型，例如 'text', 'number', 'boolean' 等。默认为 'text'。 | [optional]
**icon** | Option<**String**> | 服务配置项的图标URL或路径，用于在界面上显示。 | [optional]
**value_defaults** | Option<**String**> | 服务配置项的默认值，当未设置具体值时使用。 | [optional]
**description** | Option<**String**> | 服务配置项的详细描述信息，用于说明其用途和配置方法。 | [optional]
**tags** | Option<**String**> | 用于分类或标记服务配置项的标签。 | [optional]
**is_system** | Option<**bool**> | 指示该配置项是否为系统级别的配置项，默认为 false。 | [optional]
**show** | Option<**bool**> | 指示服务配置项是否在界面上显示，默认为 true。 | [optional]
**show_index** | Option<**i32**> | 服务配置项在界面上的显示顺序。 | [optional]
**create_date** | Option<**String**> | 服务配置项的创建日期，默认为当前时间。 | [optional]
**last_update** | Option<**String**> | 服务配置项的最后更新日期，默认为当前时间。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


