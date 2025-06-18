# GrantRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**redirect_uri** | Option<**String**> | 返回地址。默认无限制，也可在【安全-开放认证网址白名单】配置 | [optional]
**grant_type** | **String** | 授权类型。可选：email、phone、unionid、account | 
**scopes** | **String** | 自定义授权范围，用英文空格分隔 | 
**user_name** | Option<**String**> | 用户名。授权类型为：email/phone/account必填 | [optional]
**password** | Option<**String**> | 登录密码。授权类型为：email/phone/account必填 | [optional]
**union_id** | Option<**String**> | unionId。授权类型为：unionid必填 | [optional]
**platform** | Option<**String**> | 平台。授权类型为：unionid必填 | [optional]
**expire_in_days** | Option<**i32**> | 授权有效期。1~99天 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


