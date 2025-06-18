# QrCodeSignUpRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sign_in_key** | Option<**i64**> | 登录密钥 | [optional]
**union_id** | **String** | UnionID，长度必须在1到50个字符之间，只能包含字母、数字、下划线和连字符 | 
**nick_name** | Option<**String**> | 用户昵称 | [optional]
**avatar** | Option<**String**> | 用户头像URL | [optional]
**data** | Option<**String**> | 自定义数据 | [optional]
**email** | Option<**String**> | 用户邮箱 | [optional]
**email_code** | Option<**String**> | 邮箱验证码（只有启用的邮箱验证码功能时，才需要传入），长度为4到8位的数字 | [optional]
**phone** | Option<**String**> | 用户手机号，必须为11位数字 | [optional]
**phone_code** | Option<**String**> | 手机验证码（只有启用的手机验证码功能时，才需要传入），长度为4到8位的数字 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


