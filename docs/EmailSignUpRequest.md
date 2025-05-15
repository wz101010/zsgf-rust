# EmailSignUpRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | **String** | 用户邮箱地址 | 
**pwd** | **String** | 用户密码，长度为6到32个字符 | 
**email_code** | Option<**String**> | 邮箱验证码 | [optional]
**phone** | Option<**String**> | 手机号，必须为11位数字 | [optional]
**phone_code** | Option<**String**> | 手机验证码（只有启用的手机验证码功能时，才需要传入） | [optional]
**nick_name** | Option<**String**> | 用户昵称 | [optional]
**avatar** | Option<**String**> | 用户头像URL | [optional]
**data** | Option<**String**> | 自定义数据 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


