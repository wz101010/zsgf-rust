# \WechatApi

All URIs are relative to *https://api-dev.zashigaofa.cn*

Method | HTTP request | Description
------------- | ------------- | -------------
[**confirm_qr_code_login**](WechatApi.md#confirm_qr_code_login) | **POST** /Wechat/{appKey}/QR-Auth/Confirm-Login | 确认二维码登录请求
[**confirm_qr_code_registration**](WechatApi.md#confirm_qr_code_registration) | **POST** /Wechat/{appKey}/QR-Auth/Confirm-Register | 确认二维码注册请求
[**initiate_qr_auth_session**](WechatApi.md#initiate_qr_auth_session) | **POST** /Wechat/{appKey}/QR-Auth/Initiate | 初始化二维码认证会话
[**scan_qr_code_for_auth**](WechatApi.md#scan_qr_code_for_auth) | **POST** /Wechat/{appKey}/QR-Auth/Scan | 验证二维码扫描结果
[**wechat_decrypt**](WechatApi.md#wechat_decrypt) | **GET** /Wechat/{appKey}/Decrypt | 解密小程序用户数据
[**wechat_generate_scheme**](WechatApi.md#wechat_generate_scheme) | **POST** /Wechat/{appKey}/GenerateScheme | 生成小程序Scheme码
[**wechat_js_code2_session**](WechatApi.md#wechat_js_code2_session) | **GET** /Wechat/{appKey}/JSCode2Session | 校验小程序登录状态
[**wechat_js_config**](WechatApi.md#wechat_js_config) | **GET** /Wechat/{appKey}/JSConfig | 配置公众号JS SDK
[**wechat_msg_sec_check**](WechatApi.md#wechat_msg_sec_check) | **POST** /Wechat/{appKey}/MsgSecCheck | 小程序内容安全检测
[**wechat_subscribe_msg**](WechatApi.md#wechat_subscribe_msg) | **POST** /Wechat/{appKey}/SubscribeMSG | 发送公众号一次性订阅消息
[**wechat_subscribe_send**](WechatApi.md#wechat_subscribe_send) | **POST** /Wechat/{appKey}/SubscribeSend | 发送小程序订阅消息
[**wechat_url_link_generate**](WechatApi.md#wechat_url_link_generate) | **POST** /Wechat/{appKey}/UrlLinkGenerate | 生成小程序URL跳转链接
[**wechat_user_info**](WechatApi.md#wechat_user_info) | **GET** /Wechat/{appKey}/UserInfo | 获取公众号H5 UnionID
[**wechat_wxa_code_get**](WechatApi.md#wechat_wxa_code_get) | **POST** /Wechat/{appKey}/WXACodeGet | 获取小程序码（普通）
[**wechat_wxa_code_get_unlimited**](WechatApi.md#wechat_wxa_code_get_unlimited) | **POST** /Wechat/{appKey}/WXACodeGetUnlimited | 获取小程序码（无限制）



## confirm_qr_code_login

> models::TokenModelApiResponse confirm_qr_code_login(app_key, qr_code_sign_in_request)
确认二维码登录请求

微信小程序用户确认二维码登录并获取访问令牌

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**qr_code_sign_in_request** | Option<[**QrCodeSignInRequest**](QrCodeSignInRequest.md)> | 登录确认请求参数 |  |

### Return type

[**models::TokenModelApiResponse**](TokenModelApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## confirm_qr_code_registration

> models::TokenModelApiResponse confirm_qr_code_registration(app_key, qr_code_sign_up_request)
确认二维码注册请求

微信小程序用户通过二维码完成注册并获取访问令牌

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**qr_code_sign_up_request** | Option<[**QrCodeSignUpRequest**](QrCodeSignUpRequest.md)> | 注册确认请求参数 |  |

### Return type

[**models::TokenModelApiResponse**](TokenModelApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## initiate_qr_auth_session

> models::Int64ApiResponse initiate_qr_auth_session(app_key, qr_code_pre_sign_in_request)
初始化二维码认证会话

创建用于微信小程序扫码登录/注册的认证会话

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**qr_code_pre_sign_in_request** | Option<[**QrCodePreSignInRequest**](QrCodePreSignInRequest.md)> | 认证会话初始化请求参数 |  |

### Return type

[**models::Int64ApiResponse**](Int64ApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scan_qr_code_for_auth

> models::UserQrCodeScanResultApiResponse scan_qr_code_for_auth(app_key, qr_code_scan_request)
验证二维码扫描结果

微信小程序扫描二维码并获取应用授权信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**qr_code_scan_request** | Option<[**QrCodeScanRequest**](QrCodeScanRequest.md)> | 二维码扫描请求参数 |  |

### Return type

[**models::UserQrCodeScanResultApiResponse**](UserQRCodeScanResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wechat_decrypt

> models::StringApiResponse wechat_decrypt(app_key, encrypted_data, iv, session_key)
解密小程序用户数据

解密小程序加密数据

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**encrypted_data** | Option<**String**> | 加密的数据 |  |
**iv** | Option<**String**> | 加密算法的初始向量 |  |
**session_key** | Option<**String**> | 会话密钥 |  |

### Return type

[**models::StringApiResponse**](StringApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wechat_generate_scheme

> models::StringApiResponse wechat_generate_scheme(app_key, request_body)
生成小程序Scheme码

生成小程序的scheme码

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**request_body** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | scheme码数据，开发参考：https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/qrcode-link/url-scheme/generateScheme.html |  |

### Return type

[**models::StringApiResponse**](StringApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wechat_js_code2_session

> models::StringApiResponse wechat_js_code2_session(app_key, js_code)
校验小程序登录状态

校验小程序登录凭证

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**js_code** | Option<**String**> | 登录凭证，开发参考：https://dwz.cn/icNajFh7 |  |

### Return type

[**models::StringApiResponse**](StringApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wechat_js_config

> models::WechatJsConfigResultApiResponse wechat_js_config(app_key, url)
配置公众号JS SDK

获取公众号H5的JS SDK配置

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**url** | Option<**String**> | 当前网页的URL |  |

### Return type

[**models::WechatJsConfigResultApiResponse**](WechatJSConfigResultApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wechat_msg_sec_check

> serde_json::Value wechat_msg_sec_check(app_key, request_body)
小程序内容安全检测

检测消息内容是否含有违法违规信息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**request_body** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | 消息内容数据，开发参考：https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/sec-center/sec-check/msgSecCheck.html |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wechat_subscribe_msg

> models::StringApiResponse wechat_subscribe_msg(app_key, request_body)
发送公众号一次性订阅消息

发送公众号H5一次性订阅消息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**request_body** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | 订阅消息数据，开发参考：https://dwz.cn/IXptek5n |  |

### Return type

[**models::StringApiResponse**](StringApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wechat_subscribe_send

> models::StringApiResponse wechat_subscribe_send(app_key, request_body)
发送小程序订阅消息

发送小程序订阅消息

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**request_body** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | 订阅消息数据，开发参考：https://dwz.cn/bohXaCnp |  |

### Return type

[**models::StringApiResponse**](StringApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wechat_url_link_generate

> models::StringApiResponse wechat_url_link_generate(app_key, request_body)
生成小程序URL跳转链接

生成小程序的网页跳转地址

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**request_body** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | 跳转地址数据，开发参考：https://developers.weixin.qq.com/miniprogram/dev/api-backend/open-api/url-link/urllink.generate.html |  |

### Return type

[**models::StringApiResponse**](StringApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wechat_user_info

> models::StringApiResponse wechat_user_info(app_key, openid)
获取公众号H5 UnionID

获取公众号H5用户的UnionID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**openid** | Option<**String**> | 用户的OpenID |  |

### Return type

[**models::StringApiResponse**](StringApiResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wechat_wxa_code_get

> std::path::PathBuf wechat_wxa_code_get(app_key, request_body)
获取小程序码（普通）

获取小程序码

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**request_body** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | 小程序码数据，开发参考：https://developers.weixin.qq.com/miniprogram/dev/api-backend/open-api/qr-code/wxacode.get.html |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: image/jpeg

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wechat_wxa_code_get_unlimited

> std::path::PathBuf wechat_wxa_code_get_unlimited(app_key, request_body)
获取小程序码（无限制）

获取无限制的小程序码

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_key** | **String** |  | [required] |
**request_body** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | 小程序码数据，开发参考：https://developers.weixin.qq.com/miniprogram/dev/api-backend/open-api/qr-code/wxacode.getUnlimited.html |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: image/jpeg

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

