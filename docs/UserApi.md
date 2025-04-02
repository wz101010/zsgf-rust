# \UserApi

All URIs are relative to *https://api.zashigaofa.cn*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user**](UserApi.md#user) | **GET** /User/{appKey}/{id} | 获取用户详情
[**user_clear**](UserApi.md#user_clear) | **DELETE** /User/{appKey}/Clear | 清空用户数据
[**user_common_interests**](UserApi.md#user_common_interests) | **GET** /User/{appKey}/Friends/CommonInterests | 有共同爱好的用户推荐
[**user_delete**](UserApi.md#user_delete) | **DELETE** /User/{appKey}/{id} | 删除用户
[**user_email_sign_in**](UserApi.md#user_email_sign_in) | **POST** /User/{appKey}/EmailSignIn | 邮箱登录
[**user_email_sign_up**](UserApi.md#user_email_sign_up) | **POST** /User/{appKey}/EmailSignUp | 邮箱注册
[**user_export**](UserApi.md#user_export) | **GET** /User/{appKey}/Export | 导出用户数据
[**user_follow_user**](UserApi.md#user_follow_user) | **POST** /User/{appKey}/Follower/{userId} | 关注用户
[**user_follower_put**](UserApi.md#user_follower_put) | **PUT** /User/{appKey}/Follower/{id} | 更新粉丝
[**user_followers**](UserApi.md#user_followers) | **GET** /User/{appKey}/Followers | 获取我的粉丝列表
[**user_following**](UserApi.md#user_following) | **GET** /User/{appKey}/Following | 获取我的关注列表
[**user_friends_near_by**](UserApi.md#user_friends_near_by) | **GET** /User/{appKey}/Friends/NearBy | 附近的用户推荐
[**user_import**](UserApi.md#user_import) | **POST** /User/{appKey}/Import | 导入用户数据
[**user_location**](UserApi.md#user_location) | **GET** /User/{appKey}/Location/{id} | 获取位置详情
[**user_location_delete**](UserApi.md#user_location_delete) | **DELETE** /User/{appKey}/Location/{id} | 删除位置
[**user_location_post**](UserApi.md#user_location_post) | **POST** /User/{appKey}/Location | 添加位置
[**user_location_put**](UserApi.md#user_location_put) | **PUT** /User/{appKey}/Location/{id} | 更新位置
[**user_locations**](UserApi.md#user_locations) | **GET** /User/{appKey}/Locations | 获取位置列表
[**user_mutual_followers**](UserApi.md#user_mutual_followers) | **GET** /User/{appKey}/Friends/MutualFollowers | 有共同粉丝的用户推荐
[**user_mutual_followings**](UserApi.md#user_mutual_followings) | **GET** /User/{appKey}/Friends/MutualFollowings | 有共同关注的用户推荐
[**user_o_auth_account_bind**](UserApi.md#user_o_auth_account_bind) | **POST** /User/{appKey}/OAuthAccountBind | 外部账号 绑定
[**user_o_auth_account_sign_in**](UserApi.md#user_o_auth_account_sign_in) | **POST** /User/{appKey}/OAuthAccountSignIn | 外部账号 登录
[**user_o_auth_accounts**](UserApi.md#user_o_auth_accounts) | **GET** /User/{appKey}/OAuthAccounts | 外部账号 绑定列表
[**user_o_auth_accounts_put_bind**](UserApi.md#user_o_auth_accounts_put_bind) | **PUT** /User/{appKey}/OAuthAccounts/{id} | 外部账号 更新绑定
[**user_o_auth_accounts_un_bind**](UserApi.md#user_o_auth_accounts_un_bind) | **DELETE** /User/{appKey}/OAuthAccounts/{id} | 外部账号 删除绑定
[**user_phone_sign_in**](UserApi.md#user_phone_sign_in) | **POST** /User/{appKey}/PhoneSignIn | 手机登录
[**user_phone_sign_up**](UserApi.md#user_phone_sign_up) | **POST** /User/{appKey}/PhoneSignUp | 手机注册
[**user_profile**](UserApi.md#user_profile) | **GET** /User/{appKey}/Profile | 获取个人资料
[**user_put**](UserApi.md#user_put) | **PUT** /User/{appKey}/{id} | 更新用户信息
[**user_qr_code_pre_sign_in**](UserApi.md#user_qr_code_pre_sign_in) | **POST** /User/{appKey}/QRCodePreSignIn | 微信小程序 - 预登陆
[**user_qr_code_scan**](UserApi.md#user_qr_code_scan) | **POST** /User/{appKey}/QRCodeScan | 微信小程序 - 扫码
[**user_qr_code_sign_in**](UserApi.md#user_qr_code_sign_in) | **POST** /User/{appKey}/QRCodeSignIn | 微信小程序 - 网页登录
[**user_qr_code_sign_up**](UserApi.md#user_qr_code_sign_up) | **POST** /User/{appKey}/QRCodeSignUp | 微信小程序 - 注册
[**user_reset_pwd**](UserApi.md#user_reset_pwd) | **POST** /User/{appKey}/ResetPwd | 重置密码
[**user_send_email_code**](UserApi.md#user_send_email_code) | **POST** /User/{appKey}/SendEmailCode | 发送邮箱验证码
[**user_send_sms_code**](UserApi.md#user_send_sms_code) | **POST** /User/{appKey}/SendSMSCode | 发送手机验证码
[**user_sign_in**](UserApi.md#user_sign_in) | **POST** /User/{appKey}/SignIn | 账号密码 登录
[**user_sign_up**](UserApi.md#user_sign_up) | **POST** /User/{appKey}/SignUp | 账号密码 注册
[**user_two_factor_auth**](UserApi.md#user_two_factor_auth) | **GET** /User/{appKey}/TwoFactorAuth | 双因素验证
[**user_unfollow_user**](UserApi.md#user_unfollow_user) | **DELETE** /User/{appKey}/Follower/{userId} | 取消关注
[**user_union_id_sign_in**](UserApi.md#user_union_id_sign_in) | **POST** /User/{appKey}/UnionIDSignIn | UnionID登录
[**user_union_id_sign_up**](UserApi.md#user_union_id_sign_up) | **POST** /User/{appKey}/UnionIDSignUp | UnionID注册
[**user_update_profile**](UserApi.md#user_update_profile) | **PUT** /User/{appKey}/Profile | 更新个人资料
[**users**](UserApi.md#users) | **GET** /User/{appKey} | 获取用户列表



## user

> models::UserApiResponse user(id, app_key)
获取用户详情

根据用户ID获取用户详情

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 用户ID | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::UserApiResponse**](UserApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_clear

> models::BooleanApiResponse user_clear(app_key)
清空用户数据

清空用户数据

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_common_interests

> models::UserCommonInterestsResultApiResponse user_common_interests(app_key, tag, skip, take)
有共同爱好的用户推荐

推荐有共同爱好的用户

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**tag** | Option<**String**> | 兴趣标签 |  |
**skip** | Option<**i32**> | 跳过的记录数 |  |[default to 0]
**take** | Option<**i32**> | 获取的记录数 |  |[default to 10]

### Return type

[**models::UserCommonInterestsResultApiResponse**](UserCommonInterestsResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_delete

> models::BooleanApiResponse user_delete(id, app_key)
删除用户

根据用户ID删除用户

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 用户ID | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_email_sign_in

> models::TokenModelApiResponse user_email_sign_in(app_key, email_sign_in_request)
邮箱登录

使用邮箱进行登录

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**email_sign_in_request** | Option<[**EmailSignInRequest**](EmailSignInRequest.md)> | 登录请求参数 |  |

### Return type

[**models::TokenModelApiResponse**](TokenModelApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_email_sign_up

> models::TokenModelApiResponse user_email_sign_up(app_key, email_sign_up_request)
邮箱注册

使用邮箱进行注册

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**email_sign_up_request** | Option<[**EmailSignUpRequest**](EmailSignUpRequest.md)> | 注册请求参数 |  |

### Return type

[**models::TokenModelApiResponse**](TokenModelApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_export

> std::path::PathBuf user_export(app_key)
导出用户数据

导出所有用户数据为Excel文件

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.openxmlformats-officedocument.spreadsheetml.sheet

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_follow_user

> models::BooleanApiResponse user_follow_user(user_id, app_key)
关注用户

关注指定用户

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | 要关注的用户ID | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_follower_put

> models::BooleanApiResponse user_follower_put(id, app_key, follower_put_model)
更新粉丝

根据粉丝ID更新粉丝信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 粉丝ID | [required] |
**app_key** | **String** |  | [required] |
**follower_put_model** | Option<[**FollowerPutModel**](FollowerPutModel.md)> | 更新粉丝的请求参数 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_followers

> models::UserFollowersResultApiResponse user_followers(app_key, tag, status, skip, take)
获取我的粉丝列表

根据条件获取我的粉丝列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**tag** | Option<**String**> | 标签 |  |
**status** | Option<**String**> | 状态 |  |
**skip** | Option<**i32**> | 跳过的记录数 |  |[default to 0]
**take** | Option<**i32**> | 获取的记录数 |  |[default to 10]

### Return type

[**models::UserFollowersResultApiResponse**](UserFollowersResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_following

> models::UserFollowingResultApiResponse user_following(app_key, tag, status, skip, take)
获取我的关注列表

根据条件获取我的关注列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**tag** | Option<**String**> | 标签 |  |
**status** | Option<**String**> | 状态 |  |
**skip** | Option<**i32**> | 跳过的记录数 |  |[default to 0]
**take** | Option<**i32**> | 获取的记录数 |  |[default to 10]

### Return type

[**models::UserFollowingResultApiResponse**](UserFollowingResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_friends_near_by

> models::UserFriendsNearByResultApiResponse user_friends_near_by(x, y, distance, app_key, gender, age_s, age_e, tag, r#type, skip, take)
附近的用户推荐

推荐附近的用户

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x** | **f64** | 纬度 | [required] |
**y** | **f64** | 经度 | [required] |
**distance** | **i64** | 附近距离，单位：米 | [required] |
**app_key** | **String** |  | [required] |
**gender** | Option<**String**> | 性别 |  |
**age_s** | Option<**i32**> | 年龄段起始 |  |
**age_e** | Option<**i32**> | 年龄段结束 |  |
**tag** | Option<**String**> | 兴趣标签 |  |
**r#type** | Option<**String**> | 分类 |  |
**skip** | Option<**i32**> | 跳过的记录数 |  |[default to 0]
**take** | Option<**i32**> | 获取的记录数 |  |[default to 10]

### Return type

[**models::UserFriendsNearByResultApiResponse**](UserFriendsNearByResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_import

> models::BooleanApiResponse user_import(app_key, check, file)
导入用户数据

导入用户数据

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**check** | Option<**bool**> | 是否进行检查 |  |
**file** | Option<**std::path::PathBuf**> | 导入的文件 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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

根据位置ID更新位置信息

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


## user_mutual_followers

> models::UserMutualFollowersResultApiResponse user_mutual_followers(app_key, skip, take)
有共同粉丝的用户推荐

推荐有共同粉丝的用户

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**skip** | Option<**i32**> | 跳过的记录数 |  |[default to 0]
**take** | Option<**i32**> | 获取的记录数 |  |[default to 10]

### Return type

[**models::UserMutualFollowersResultApiResponse**](UserMutualFollowersResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_mutual_followings

> models::UserMutualFollowingsResultApiResponse user_mutual_followings(app_key, skip, take)
有共同关注的用户推荐

推荐有共同关注的用户

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**skip** | Option<**i32**> | 跳过的记录数 |  |[default to 0]
**take** | Option<**i32**> | 获取的记录数 |  |[default to 10]

### Return type

[**models::UserMutualFollowingsResultApiResponse**](UserMutualFollowingsResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_o_auth_account_bind

> models::BooleanApiResponse user_o_auth_account_bind(app_key, o_auth_account_bind_request)
外部账号 绑定

绑定外部账号

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**o_auth_account_bind_request** | Option<[**OAuthAccountBindRequest**](OAuthAccountBindRequest.md)> | 绑定请求参数 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_o_auth_account_sign_in

> models::TokenModelApiResponse user_o_auth_account_sign_in(app_key, o_auth_account_sign_in_request)
外部账号 登录

使用外部账号进行登录

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**o_auth_account_sign_in_request** | Option<[**OAuthAccountSignInRequest**](OAuthAccountSignInRequest.md)> | 登录请求参数 |  |

### Return type

[**models::TokenModelApiResponse**](TokenModelApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_o_auth_accounts

> models::UserLoginsListApiResponse user_o_auth_accounts(app_key)
外部账号 绑定列表

获取外部账号绑定列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |

### Return type

[**models::UserLoginsListApiResponse**](UserLoginsListApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_o_auth_accounts_put_bind

> models::BooleanApiResponse user_o_auth_accounts_put_bind(id, app_key, o_auth_account_put_bind_request)
外部账号 更新绑定

更新外部账号绑定信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 绑定ID | [required] |
**app_key** | **String** |  | [required] |
**o_auth_account_put_bind_request** | Option<[**OAuthAccountPutBindRequest**](OAuthAccountPutBindRequest.md)> | 更新请求参数 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_o_auth_accounts_un_bind

> models::BooleanApiResponse user_o_auth_accounts_un_bind(id, app_key)
外部账号 删除绑定

删除外部账号绑定

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 绑定ID | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_phone_sign_in

> models::TokenModelApiResponse user_phone_sign_in(app_key, phone_sign_in_request)
手机登录

使用手机号码进行登录

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**phone_sign_in_request** | Option<[**PhoneSignInRequest**](PhoneSignInRequest.md)> | 登录请求参数 |  |

### Return type

[**models::TokenModelApiResponse**](TokenModelApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_phone_sign_up

> models::TokenModelApiResponse user_phone_sign_up(app_key, phone_sign_up_request)
手机注册

使用手机号码进行注册

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**phone_sign_up_request** | Option<[**PhoneSignUpRequest**](PhoneSignUpRequest.md)> | 注册请求参数 |  |

### Return type

[**models::TokenModelApiResponse**](TokenModelApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_profile

> models::UserProfileResultApiResponse user_profile(app_key)
获取个人资料

获取当前用户的个人资料

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |

### Return type

[**models::UserProfileResultApiResponse**](UserProfileResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_put

> models::BooleanApiResponse user_put(id, app_key, user)
更新用户信息

根据用户ID更新用户信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | 用户ID | [required] |
**app_key** | **String** |  | [required] |
**user** | Option<[**User**](User.md)> | 用户信息 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_qr_code_pre_sign_in

> models::Int64ApiResponse user_qr_code_pre_sign_in(app_key, qr_code_pre_sign_in_request)
微信小程序 - 预登陆

使用微信小程序二维码进行预登陆

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**qr_code_pre_sign_in_request** | Option<[**QrCodePreSignInRequest**](QrCodePreSignInRequest.md)> | 预登陆请求参数 |  |

### Return type

[**models::Int64ApiResponse**](Int64ApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_qr_code_scan

> models::UserQrCodeScanResultApiResponse user_qr_code_scan(app_key, qr_code_scan_request)
微信小程序 - 扫码

使用微信小程序二维码进行扫码

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**qr_code_scan_request** | Option<[**QrCodeScanRequest**](QrCodeScanRequest.md)> | 扫码请求参数 |  |

### Return type

[**models::UserQrCodeScanResultApiResponse**](UserQRCodeScanResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_qr_code_sign_in

> models::TokenModelApiResponse user_qr_code_sign_in(app_key, qr_code_sign_in_request)
微信小程序 - 网页登录

使用微信小程序二维码进行网页登录

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**qr_code_sign_in_request** | Option<[**QrCodeSignInRequest**](QrCodeSignInRequest.md)> | 登录请求参数 |  |

### Return type

[**models::TokenModelApiResponse**](TokenModelApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_qr_code_sign_up

> models::TokenModelApiResponse user_qr_code_sign_up(app_key, qr_code_sign_up_request)
微信小程序 - 注册

使用微信小程序二维码进行注册

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**qr_code_sign_up_request** | Option<[**QrCodeSignUpRequest**](QrCodeSignUpRequest.md)> | 注册请求参数 |  |

### Return type

[**models::TokenModelApiResponse**](TokenModelApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_reset_pwd

> models::BooleanApiResponse user_reset_pwd(app_key, app_user_reset_pwd_request)
重置密码

通过手机号或邮箱重置密码

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**app_user_reset_pwd_request** | Option<[**AppUserResetPwdRequest**](AppUserResetPwdRequest.md)> | 重置密码的请求参数 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_send_email_code

> models::BooleanApiResponse user_send_email_code(app_key, send_email_code_request)
发送邮箱验证码

发送邮箱验证码用于注册或找回密码

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**send_email_code_request** | Option<[**SendEmailCodeRequest**](SendEmailCodeRequest.md)> | 发送邮箱验证码的请求参数 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_send_sms_code

> models::BooleanApiResponse user_send_sms_code(app_key, send_sms_code_request)
发送手机验证码

发送手机验证码用于注册或找回密码

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**send_sms_code_request** | Option<[**SendSmsCodeRequest**](SendSmsCodeRequest.md)> | 发送手机验证码的请求参数 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_sign_in

> models::TokenModelApiResponse user_sign_in(app_key, sign_in_request)
账号密码 登录

使用账号密码进行登录

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**sign_in_request** | Option<[**SignInRequest**](SignInRequest.md)> | 登录请求参数 |  |

### Return type

[**models::TokenModelApiResponse**](TokenModelApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_sign_up

> models::TokenModelApiResponse user_sign_up(app_key, sign_up_request)
账号密码 注册

使用账号密码进行注册

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**sign_up_request** | Option<[**SignUpRequest**](SignUpRequest.md)> | 注册请求参数 |  |

### Return type

[**models::TokenModelApiResponse**](TokenModelApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_two_factor_auth

> models::SetupCodeApiResponse user_two_factor_auth(app_key)
双因素验证

获取双因素验证的设置信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |

### Return type

[**models::SetupCodeApiResponse**](SetupCodeApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_unfollow_user

> models::BooleanApiResponse user_unfollow_user(user_id, app_key)
取消关注

取消关注指定用户

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | 要取消关注的用户ID | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_union_id_sign_in

> models::TokenModelApiResponse user_union_id_sign_in(app_key, union_id_sign_in_request)
UnionID登录

使用UnionID进行登录

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**union_id_sign_in_request** | Option<[**UnionIdSignInRequest**](UnionIdSignInRequest.md)> | 登录请求参数 |  |

### Return type

[**models::TokenModelApiResponse**](TokenModelApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_union_id_sign_up

> models::TokenModelApiResponse user_union_id_sign_up(app_key, union_id_sign_up_request)
UnionID注册

使用UnionID进行注册

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**union_id_sign_up_request** | Option<[**UnionIdSignUpRequest**](UnionIdSignUpRequest.md)> | 注册请求参数 |  |

### Return type

[**models::TokenModelApiResponse**](TokenModelApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_update_profile

> models::BooleanApiResponse user_update_profile(app_key, update_profile_request)
更新个人资料

更新当前用户的个人资料

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**update_profile_request** | Option<[**UpdateProfileRequest**](UpdateProfileRequest.md)> | 更新个人资料的请求参数 |  |

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users

> models::UserListResultApiResponse users(app_key, user_name, email, phone, platform, union_id, role, skip, take)
获取用户列表

根据条件获取用户列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**user_name** | Option<**String**> | 用户名 |  |
**email** | Option<**String**> | 邮箱 |  |
**phone** | Option<**String**> | 电话 |  |
**platform** | Option<**String**> | 平台 |  |
**union_id** | Option<**String**> | 联合ID |  |
**role** | Option<**String**> | 角色 |  |
**skip** | Option<**i32**> | 跳过的记录数 |  |
**take** | Option<**i32**> | 获取的记录数 |  |

### Return type

[**models::UserListResultApiResponse**](UserListResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

