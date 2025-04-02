# UserAccessToken

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | 用户令牌的唯一标识符。 | [optional]
**user_id** | Option<**i64**> | 与令牌关联的用户ID。 | [optional]
**enable** | Option<**bool**> | 指示令牌是否处于启用状态。 | [optional]
**permissions** | Option<**String**> | 令牌拥有的权限列表，多个权限以逗号分隔。 | [optional]
**title** | Option<**String**> | 令牌的标题或名称，用于标识令牌。 | [optional]
**access_token** | Option<**String**> | 访问令牌的具体值，用于身份验证。 | [optional]
**tags** | Option<**String**> | 用于分类或标记令牌的标签。 | [optional]
**description** | Option<**String**> | 令牌的详细描述信息。 | [optional]
**expire_time** | Option<**String**> | 令牌的过期时间，超过该时间令牌将失效。 | [optional]
**create_date** | Option<**String**> | 令牌的创建日期，默认为当前时间。 | [optional]
**last_update** | Option<**String**> | 令牌的最后更新日期，默认为当前时间。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


