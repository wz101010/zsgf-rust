# \UserFriendsApi

All URIs are relative to *https://api-dev.zashigaofa.cn*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_common_interests**](UserFriendsApi.md#user_common_interests) | **GET** /UserFriends/{appKey}/CommonInterests | 推荐相似兴趣用户
[**user_follow_user**](UserFriendsApi.md#user_follow_user) | **POST** /UserFriends/{appKey}/Follower/{userId} | 添加关注
[**user_follower_put**](UserFriendsApi.md#user_follower_put) | **PUT** /UserFriends/{appKey}/Follower/{id} | 刷新粉丝数据
[**user_followers**](UserFriendsApi.md#user_followers) | **GET** /UserFriends/{appKey}/Followers | 获取粉丝列表
[**user_following**](UserFriendsApi.md#user_following) | **GET** /UserFriends/{appKey}/Following | 获取关注列表 / 判断是否关注
[**user_friends_near_by**](UserFriendsApi.md#user_friends_near_by) | **GET** /UserFriends/{appKey}/NearBy | 推荐附近用户
[**user_mutual_followers**](UserFriendsApi.md#user_mutual_followers) | **GET** /UserFriends/{appKey}/MutualFollowers | 推荐共同粉丝用户
[**user_mutual_followings**](UserFriendsApi.md#user_mutual_followings) | **GET** /UserFriends/{appKey}/MutualFollowings | 推荐共同关注用户
[**user_profile_by_id**](UserFriendsApi.md#user_profile_by_id) | **GET** /UserFriends/{appKey}/Profile/{userId} | 获取用户资料
[**user_unfollow_user**](UserFriendsApi.md#user_unfollow_user) | **DELETE** /UserFriends/{appKey}/Follower/{userId} | 取消关注



## user_common_interests

> models::UserCommonInterestsResultApiResponse user_common_interests(app_key, tag, skip, take)
推荐相似兴趣用户

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


## user_follow_user

> models::BooleanApiResponse user_follow_user(user_id, app_key)
添加关注

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
刷新粉丝数据

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

> models::UserFollowersResultApiResponse user_followers(app_key, tag, status, target_user_id, skip, take)
获取粉丝列表

根据条件获取我的粉丝列表

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**tag** | Option<**String**> | 标签 |  |
**status** | Option<**String**> | 状态 |  |
**target_user_id** | Option<**i64**> | 指定用户的粉丝 |  |[default to 0]
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

> models::BooleanApiResponse user_following(app_key, tag, status, target_user_id, skip, take, check_user_id, only_ids)
获取关注列表 / 判断是否关注

根据条件获取我的关注列表，或判断是否关注某个用户

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**tag** | Option<**String**> | 用于过滤关注列表的标签（可选）。 |  |
**status** | Option<**String**> | 用于过滤关注列表的状态（可选）。 |  |
**target_user_id** | Option<**i64**> | 指定用户的关注记录，如果不提供则默认为当前用户的关注。 |  |[default to 0]
**skip** | Option<**i32**> | 跳过的记录数，用于分页（默认0）。 |  |[default to 0]
**take** | Option<**i32**> | 获取的记录数，用于分页（默认10）。 |  |[default to 10]
**check_user_id** | Option<**i64**> | 要判断是否关注的目标用户ID。如果提供此参数，方法将返回一个布尔值，表示当前用户是否关注该目标用户。 |  |
**only_ids** | Option<**bool**> | 是否只返回关注用户的ID集合，默认为false（即返回完整的关注用户信息）。 |  |[default to false]

### Return type

[**models::BooleanApiResponse**](BooleanApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_friends_near_by

> models::UserFriendsNearByResultApiResponse user_friends_near_by(longitude, latitude, app_key, country, state, city, district, gender, age_s, age_e, tag, distance, skip, take)
推荐附近用户

根据地理位置坐标和多种筛选条件，查询附近满足条件的用户列表，支持分页和按距离排序。 地理位置查询使用MySQL的ST_Distance_Sphere函数计算球面距离。 注意：longitude为经度(X轴)，latitude为纬度(Y轴)，参数顺序与常规坐标系一致

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**longitude** | **f64** | 当前用户经度坐标(WGS84坐标系) | [required] |
**latitude** | **f64** | 当前用户纬度坐标(WGS84坐标系) | [required] |
**app_key** | **String** |  | [required] |
**country** | Option<**String**> | 国家过滤条件（精确匹配） |  |
**state** | Option<**String**> | 省份过滤条件（精确匹配） |  |
**city** | Option<**String**> | 城市过滤条件（精确匹配） |  |
**district** | Option<**String**> | 区县过滤条件（精确匹配） |  |
**gender** | Option<**String**> | 性别过滤条件（可选值示例：Male/Female/Other） |  |
**age_s** | Option<**i32**> | 年龄起始范围（包含，0表示不限制） |  |
**age_e** | Option<**i32**> | 年龄结束范围（包含，0表示不限制） |  |
**tag** | Option<**String**> | 兴趣标签过滤（支持模糊匹配，如：\"运动\"） |  |
**distance** | Option<**i64**> | 搜索半径（单位：米，0表示不限制距离） |  |[default to 0]
**skip** | Option<**i32**> | 跳过的记录数（分页起始位置，默认0） |  |[default to 0]
**take** | Option<**i32**> | 获取的记录数（分页大小，默认10，最大100） |  |[default to 10]

### Return type

[**models::UserFriendsNearByResultApiResponse**](UserFriendsNearByResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_mutual_followers

> models::UserMutualFollowersResultApiResponse user_mutual_followers(app_key, skip, take)
推荐共同粉丝用户

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
推荐共同关注用户

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


## user_profile_by_id

> models::GetUserProfileResultApiResponse user_profile_by_id(user_id, app_key)
获取用户资料

用于他人主页展示

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | 用户ID | [required] |
**app_key** | **String** |  | [required] |

### Return type

[**models::GetUserProfileResultApiResponse**](GetUserProfileResultApiResponse.md)

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

