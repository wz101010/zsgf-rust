# AuthorizePolicy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | 鉴权策略的唯一标识符。 | [optional]
**policy_name** | Option<**String**> | 鉴权策略的名称。根据鉴权类型填写不同的名称：角色类型填写角色名称，用户类型填写用户ID，访问令牌类型填写令牌ID。 | [optional]
**authorize_type** | Option<**String**> | 鉴权策略的类型，可选值为 'access_token', 'user', 或 'role'。 | [optional]
**policy_value** | Option<**String**> | 与鉴权策略关联的权限集合，多个权限可以用逗号分隔。 | [optional]
**create_date** | Option<**String**> | 鉴权策略的创建日期，默认为当前时间。 | [optional]
**last_update** | Option<**String**> | 鉴权策略的最后更新日期，默认为当前时间。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


