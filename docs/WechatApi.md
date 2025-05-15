# \WechatApi

All URIs are relative to *https://api.zashigaofa.cn*

Method | HTTP request | Description
------------- | ------------- | -------------
[**wechat_decrypt**](WechatApi.md#wechat_decrypt) | **GET** /Wechat/{appKey}/Decrypt | 小程序-解密数据
[**wechat_generate_scheme**](WechatApi.md#wechat_generate_scheme) | **POST** /Wechat/{appKey}/GenerateScheme | 小程序-生成scheme码(该接口用于获取小程序 scheme 码，适用于短信、邮件、外部网页、微信内等拉起小程序的业务场景)
[**wechat_js_code2_session**](WechatApi.md#wechat_js_code2_session) | **GET** /Wechat/{appKey}/JSCode2Session | 小程序-登录凭证校验
[**wechat_js_config**](WechatApi.md#wechat_js_config) | **GET** /Wechat/{appKey}/JSConfig | 公众号H5-JS SDK Config
[**wechat_msg_sec_check**](WechatApi.md#wechat_msg_sec_check) | **POST** /Wechat/{appKey}/MsgSecCheck | 小程序-消息内容安全检测
[**wechat_subscribe_msg**](WechatApi.md#wechat_subscribe_msg) | **POST** /Wechat/{appKey}/SubscribeMSG | 公众号H5-发送一次性订阅消息
[**wechat_subscribe_send**](WechatApi.md#wechat_subscribe_send) | **POST** /Wechat/{appKey}/SubscribeSend | 小程序-发送订阅消息
[**wechat_url_link_generate**](WechatApi.md#wechat_url_link_generate) | **POST** /Wechat/{appKey}/UrlLinkGenerate | 小程序-生成网页跳转地址(获取小程序 URL Link，适用于短信、邮件、网页、微信内等拉起小程序的业务场景)
[**wechat_user_info**](WechatApi.md#wechat_user_info) | **GET** /Wechat/{appKey}/UserInfo | 公众号H5-获取用户UnionID
[**wechat_wxa_code_get**](WechatApi.md#wechat_wxa_code_get) | **POST** /Wechat/{appKey}/WXACodeGet | 小程序-获取小程序码
[**wechat_wxa_code_get_unlimited**](WechatApi.md#wechat_wxa_code_get_unlimited) | **POST** /Wechat/{appKey}/WXACodeGetUnlimited | 小程序-获取小程序码(无限制)



## wechat_decrypt

> models::StringApiResponse wechat_decrypt(app_key, encrypted_data, iv, session_key)
小程序-解密数据

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
小程序-生成scheme码(该接口用于获取小程序 scheme 码，适用于短信、邮件、外部网页、微信内等拉起小程序的业务场景)

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
小程序-登录凭证校验

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
公众号H5-JS SDK Config

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
小程序-消息内容安全检测

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
公众号H5-发送一次性订阅消息

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
小程序-发送订阅消息

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
小程序-生成网页跳转地址(获取小程序 URL Link，适用于短信、邮件、网页、微信内等拉起小程序的业务场景)

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
公众号H5-获取用户UnionID

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
小程序-获取小程序码

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
- **Accept**: image/jpeg, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wechat_wxa_code_get_unlimited

> std::path::PathBuf wechat_wxa_code_get_unlimited(app_key, request_body)
小程序-获取小程序码(无限制)

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
- **Accept**: image/jpeg, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

