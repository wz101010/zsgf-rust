# ZSGF Client SDK for Rust

![Crates.io MSRV](https://img.shields.io/crates/v/zsgf-client)

一个功能完整的 Rust SDK，用于与 ZSGF 服务平台进行交互。提供用户管理、支付、存储、文件管理等全方位的 API 支持。

## 🚀 快速开始

### 安装

```bash
cargo add zsgf-client
```

### 基本使用

```rust
use zsgf_client::{Client, Configuration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建配置
    let config = Configuration {
        base_path: "https://api.zsgf.com".to_string(),
        app_key: "your_app_key".to_string(),
        // 其他配置参数...
    };
    
    // 创建客户端
    let client = Client::new(config);
    
    // 使用 API
    let user_api = client.user_api();
    let profile = user_api.user_profile("your_app_key").await?;
    
    println!("用户资料: {:?}", profile);
    
    Ok(())
}
```

## 📚 核心功能

### 用户管理
- 用户注册/登录（邮箱、手机、密码、UnionID）
- 个人资料管理
- 二次验证
- 账号注销

### 支付集成
- 支付宝支付（当面付、PC支付、WAP支付）
- 订单管理
- 退款处理

### 数据存储
- 灵活的数据存储和查询
- 聚合查询支持
- 数据增删改查

### 文件管理
- 文件上传/下载
- 文件夹管理
- 文件重命名和删除

### 社交功能
- 用户关注/粉丝系统
- 好友推荐
- 地理位置服务

### 第三方集成
- 微信小程序/公众号
- 钉钉集成
- OAuth 2.0 授权

## 🔧 详细配置

```rust
use zsgf_client::Configuration;

let config = Configuration {
    base_path: "https://api.zsgf.com".to_string(),
    app_key: "your_app_key".to_string(),
    access_token: Some("your_access_token".to_string()),
    timeout: Some(std::time::Duration::from_secs(30)),
    // 其他可选配置...
};
```

## 📖 API 分类说明

### 🔐 认证与授权

| API 类 | 描述 |
|--------|------|
| `AccessTokenApi` | 访问令牌管理 |
| `OAuthApi` | OAuth 2.0 授权流程 |
| `ExternalAccountApi` | 外部账号绑定与登录 |

**示例用法：**
```rust
// 创建访问令牌
let token_request = AccessTokenPostRequest {
    name: "My Token".to_string(),
    // 其他字段...
};
let token = client.access_token_api()
    .access_token_post("app_key", token_request).await?;
```

### 👤 用户管理

| API 类 | 描述 |
|--------|------|
| `UserApi` | 用户注册、登录、资料管理 |
| `UserFriendsApi` | 用户关系（关注、粉丝、好友推荐） |
| `UserLocationApi` | 用户地理位置管理 |
| `UserCurrencyApi` | 用户虚拟货币管理 |

**示例用法：**
```rust
// 用户邮箱注册
let signup_request = EmailSignUpRequest {
    email: "user@example.com".to_string(),
    password: "secure_password".to_string(),
    verification_code: "123456".to_string(),
};
let result = client.user_api()
    .user_email_sign_up("app_key", signup_request).await?;

// 获取用户资料
let profile = client.user_api()
    .user_profile("app_key").await?;
```

### 💰 支付服务

| API 类 | 描述 |
|--------|------|
| `AlipayApi` | 支付宝支付集成 |
| `OrderApi` | 订单管理系统 |

**示例用法：**
```rust
// 创建支付宝订单
let order_request = AlipayCreateOrderRequest {
    out_trade_no: "ORDER_001".to_string(),
    total_amount: "99.00".to_string(),
    subject: "商品购买".to_string(),
    // 其他字段...
};
let payment_url = client.alipay_api()
    .alipay_create_order("app_key", order_request).await?;
```

### 💾 数据存储

| API 类 | 描述 |
|--------|------|
| `StorageApi` | 灵活的数据存储和查询服务 |

**示例用法：**
```rust
// 添加数据
let data = serde_json::json!({
    "name": "张三",
    "age": 25,
    "city": "北京"
});
let result = client.storage_api()
    .storage_post("app_key", "users", data).await?;

// 查询数据
let users = client.storage_api()
    .storage_list("app_key", "users", None, None, None, None).await?;
```

### 📁 文件管理

| API 类 | 描述 |
|--------|------|
| `FileApi` | 文件上传、下载、管理 |

**示例用法：**
```rust
// 上传文件
let file_data = std::fs::read("./image.jpg")?;
let upload_result = client.file_api()
    .file_upload("app_key", file_data, Some("image.jpg".to_string())).await?;

// 获取文件列表
let files = client.file_api()
    .files("app_key", None, None).await?;
```

### 🔗 第三方集成

| API 类 | 描述 |
|--------|------|
| `WechatApi` | 微信小程序/公众号集成 |
| `DingTalkApi` | 钉钉企业应用集成 |

**示例用法：**
```rust
// 微信小程序登录
let session_result = client.wechat_api()
    .wechat_js_code2_session("app_key", "js_code").await?;

// 发送微信订阅消息
let msg_request = /* 构建消息请求 */;
let send_result = client.wechat_api()
    .wechat_subscribe_send("app_key", msg_request).await?;
```

### 🏢 应用管理

| API 类 | 描述 |
|--------|------|
| `AppApi` | 应用信息查询和管理 |

## 🛠️ 高级用法

### 错误处理

```rust
use zsgf_client::Error;

match client.user_api().user_profile("app_key").await {
    Ok(profile) => println!("获取成功: {:?}", profile),
    Err(Error::ApiError(api_error)) => {
        println!("API 错误: {} - {}", api_error.code, api_error.message);
    },
    Err(Error::HttpError(http_error)) => {
        println!("网络错误: {:?}", http_error);
    },
    Err(e) => println!("其他错误: {:?}", e),
}
```

### 自定义配置

```rust
let config = Configuration {
    base_path: "https://your-custom-endpoint.com".to_string(),
    app_key: "your_app_key".to_string(),
    access_token: Some("bearer_token".to_string()),
    timeout: Some(std::time::Duration::from_secs(60)),
    retry_count: Some(3),
    // 更多自定义选项...
};
```

## 🔍 常见使用场景

### 1. 用户登录流程
```rust
// 发送验证码
client.user_api().user_send_sms_code("app_key", phone_request).await?;

// 手机号登录
let login_result = client.user_api()
    .user_phone_sign_in("app_key", phone_signin_request).await?;
```

### 2. 支付流程
```rust
// 创建订单
let order = client.order_api().order_create("app_key", order_request).await?;

// 发起支付
let payment = client.alipay_api()
    .alipay_create_order_page_pay("app_key", payment_request).await?;
```

### 3. 数据管理
```rust
// 批量查询
let query_params = /* 构建查询参数 */;
let results = client.storage_api()
    .storage_list("app_key", "table_name", Some(query_params), None, None, None).await?;

// 聚合统计
let aggregate_result = client.storage_api()
    .storage_aggregate("app_key", "table_name", "COUNT(*)", None).await?;
```

## 📝 注意事项

1. **API Key 安全**: 请妥善保管您的 `app_key`，不要在客户端代码中硬编码
2. **错误处理**: 建议对所有 API 调用进行适当的错误处理
3. **速率限制**: 注意 API 调用频率限制，避免超出配额
4. **数据验证**: 在发送请求前验证数据格式和必填字段

## 📚 更多资源

- [API 完整文档](./docs/)
- [模型定义](./docs/models/)

## 🤝 支持与反馈

如果您在使用过程中遇到问题或有改进建议，请：
提交 [Issue](https://github.com/wz101010/zsgf-rust)

---

## API 详细参考

### 完整 API 列表

<details>
<summary>点击展开查看所有 API 端点</summary>

#### 🔐 认证与授权 API

| 类 | 方法 | HTTP 请求 | 描述 |
|---|---|---|---|
| *AccessTokenApi* | [**access_token_delete**](docs/AccessTokenApi.md#access_token_delete) | **DELETE** /AccessToken/{appKey}/{id} | 删除令牌 |
| *AccessTokenApi* | [**access_token_post**](docs/AccessTokenApi.md#access_token_post) | **POST** /AccessToken/{appKey} | 创建令牌 |
| *AccessTokenApi* | [**access_token_put**](docs/AccessTokenApi.md#access_token_put) | **PUT** /AccessToken/{appKey}/{id} | 更新令牌 |
| *AccessTokenApi* | [**access_tokens**](docs/AccessTokenApi.md#access_tokens) | **GET** /AccessToken/{appKey} | 令牌列表 |
| *OAuthApi* | [**o_auth_authorize**](docs/OAuthApi.md#o_auth_authorize) | **POST** /OAuth/{appKey}/Authorize | 获取访问令牌 |
| *OAuthApi* | [**o_auth_consents**](docs/OAuthApi.md#o_auth_consents) | **GET** /OAuth/{appKey}/Consents | 获取授权记录 |
| *OAuthApi* | [**o_auth_delete_consent**](docs/OAuthApi.md#o_auth_delete_consent) | **DELETE** /OAuth/{appKey}/Consents/{id} | 删除授权记录 |
| *OAuthApi* | [**o_auth_grant_code**](docs/OAuthApi.md#o_auth_grant_code) | **POST** /OAuth/{appKey}/GrantCode | 获取授权码 |
| *OAuthApi* | [**o_auth_profile**](docs/OAuthApi.md#o_auth_profile) | **GET** /OAuth/{appKey}/Profile | 获取用户资料 |
| *ExternalAccountApi* | [**external_account_sign_in**](docs/ExternalAccountApi.md#external_account_sign_in) | **POST** /ExternalAccount/{appKey}/SignIn | 外部账号登录 |
| *ExternalAccountApi* | [**user_external_account_bind**](docs/ExternalAccountApi.md#user_external_account_bind) | **POST** /ExternalAccount/{appKey} | 绑定外部账号 |
| *ExternalAccountApi* | [**user_o_auth_accounts**](docs/ExternalAccountApi.md#user_o_auth_accounts) | **GET** /ExternalAccount/{appKey} | 外部账号列表 |
| *ExternalAccountApi* | [**user_o_auth_accounts_put_bind**](docs/ExternalAccountApi.md#user_o_auth_accounts_put_bind) | **PUT** /ExternalAccount/{appKey}/{id} | 更新绑定账号 |
| *ExternalAccountApi* | [**user_o_auth_accounts_un_bind**](docs/ExternalAccountApi.md#user_o_auth_accounts_un_bind) | **DELETE** /ExternalAccount/{appKey}/{id} | 删除绑定账号 |

#### 👤 用户管理 API

| 类 | 方法 | HTTP 请求 | 描述 |
|---|---|---|---|
| *UserApi* | [**user_deactivate_hard**](docs/UserApi.md#user_deactivate_hard) | **DELETE** /User/{appKey}/DeactivateHard | 注销账号 |
| *UserApi* | [**user_email_sign_in**](docs/UserApi.md#user_email_sign_in) | **POST** /User/{appKey}/EmailSignIn | 邮箱登录 |
| *UserApi* | [**user_email_sign_up**](docs/UserApi.md#user_email_sign_up) | **POST** /User/{appKey}/EmailSignUp | 邮箱注册 |
| *UserApi* | [**user_phone_sign_in**](docs/UserApi.md#user_phone_sign_in) | **POST** /User/{appKey}/PhoneSignIn | 手机登录 |
| *UserApi* | [**user_phone_sign_up**](docs/UserApi.md#user_phone_sign_up) | **POST** /User/{appKey}/PhoneSignUp | 手机注册 |
| *UserApi* | [**user_profile**](docs/UserApi.md#user_profile) | **GET** /User/{appKey}/Profile | 获取个人资料 |
| *UserApi* | [**user_reset_email**](docs/UserApi.md#user_reset_email) | **PUT** /User/{appKey}/ResetEmail | 重置邮箱 |
| *UserApi* | [**user_reset_phone**](docs/UserApi.md#user_reset_phone) | **PUT** /User/{appKey}/ResetPhone | 重置手机号 |
| *UserApi* | [**user_reset_pwd**](docs/UserApi.md#user_reset_pwd) | **POST** /User/{appKey}/ResetPwd | 重置密码 |
| *UserApi* | [**user_send_email_code**](docs/UserApi.md#user_send_email_code) | **POST** /User/{appKey}/SendEmailCode | 发送邮箱验证码 |
| *UserApi* | [**user_send_sms_code**](docs/UserApi.md#user_send_sms_code) | **POST** /User/{appKey}/SendSMSCode | 发送手机验证码 |
| *UserApi* | [**user_sign_in**](docs/UserApi.md#user_sign_in) | **POST** /User/{appKey}/SignIn | 密码登录 |
| *UserApi* | [**user_sign_up**](docs/UserApi.md#user_sign_up) | **POST** /User/{appKey}/SignUp | 账号注册 |
| *UserApi* | [**user_two_factor_auth**](docs/UserApi.md#user_two_factor_auth) | **GET** /User/{appKey}/TwoFactorAuth | 二次验证 |
| *UserApi* | [**user_union_id_sign_in**](docs/UserApi.md#user_union_id_sign_in) | **POST** /User/{appKey}/UnionIDSignIn | UnionID登录 |
| *UserApi* | [**user_union_id_sign_up**](docs/UserApi.md#user_union_id_sign_up) | **POST** /User/{appKey}/UnionIDSignUp | UnionID注册 |
| *UserApi* | [**user_update_profile**](docs/UserApi.md#user_update_profile) | **PUT** /User/{appKey}/Profile | 更新个人资料 |
| *UserFriendsApi* | [**user_common_interests**](docs/UserFriendsApi.md#user_common_interests) | **GET** /UserFriends/{appKey}/CommonInterests | 推荐相似兴趣用户 |
| *UserFriendsApi* | [**user_follow_user**](docs/UserFriendsApi.md#user_follow_user) | **POST** /UserFriends/{appKey}/Follower/{userId} | 添加关注 |
| *UserFriendsApi* | [**user_follower_put**](docs/UserFriendsApi.md#user_follower_put) | **PUT** /UserFriends/{appKey}/Follower/{id} | 刷新粉丝数据 |
| *UserFriendsApi* | [**user_followers**](docs/UserFriendsApi.md#user_followers) | **GET** /UserFriends/{appKey}/Followers | 获取粉丝列表 |
| *UserFriendsApi* | [**user_following**](docs/UserFriendsApi.md#user_following) | **GET** /UserFriends/{appKey}/Following | 获取关注列表 / 判断是否关注 |
| *UserFriendsApi* | [**user_friends_near_by**](docs/UserFriendsApi.md#user_friends_near_by) | **GET** /UserFriends/{appKey}/NearBy | 推荐附近用户 |
| *UserFriendsApi* | [**user_mutual_followers**](docs/UserFriendsApi.md#user_mutual_followers) | **GET** /UserFriends/{appKey}/MutualFollowers | 推荐共同粉丝用户 |
| *UserFriendsApi* | [**user_mutual_followings**](docs/UserFriendsApi.md#user_mutual_followings) | **GET** /UserFriends/{appKey}/MutualFollowings | 推荐共同关注用户 |
| *UserFriendsApi* | [**user_profile_by_id**](docs/UserFriendsApi.md#user_profile_by_id) | **GET** /UserFriends/{appKey}/Profile/{userId} | 获取用户资料 |
| *UserFriendsApi* | [**user_unfollow_user**](docs/UserFriendsApi.md#user_unfollow_user) | **DELETE** /UserFriends/{appKey}/Follower/{userId} | 取消关注 |
| *UserLocationApi* | [**user_location**](docs/UserLocationApi.md#user_location) | **GET** /UserLocation/{appKey}/{id} | 获取位置详情 |
| *UserLocationApi* | [**user_location_delete**](docs/UserLocationApi.md#user_location_delete) | **DELETE** /UserLocation/{appKey}/{id} | 删除位置 |
| *UserLocationApi* | [**user_location_post**](docs/UserLocationApi.md#user_location_post) | **POST** /UserLocation/{appKey} | 添加位置 |
| *UserLocationApi* | [**user_location_put**](docs/UserLocationApi.md#user_location_put) | **PUT** /UserLocation/{appKey}/{id} | 更新位置 |
| *UserLocationApi* | [**user_locations**](docs/UserLocationApi.md#user_locations) | **GET** /UserLocation/{appKey} | 获取位置列表 |
| *UserCurrencyApi* | [**user_currencies**](docs/UserCurrencyApi.md#user_currencies) | **GET** /UserCurrency/{appKey}/{id} | 获取用户资产 |
| *UserCurrencyApi* | [**user_currency_consume**](docs/UserCurrencyApi.md#user_currency_consume) | **POST** /UserCurrency/{appKey}/CurrencyConsume | 消费虚拟币 |
| *UserCurrencyApi* | [**user_currency_exchange**](docs/UserCurrencyApi.md#user_currency_exchange) | **POST** /UserCurrency/{appKey}/CurrencyExchange | 兑换虚拟币 |
| *UserCurrencyApi* | [**user_currency_recharge**](docs/UserCurrencyApi.md#user_currency_recharge) | **POST** /UserCurrency/{appKey}/CurrencyRecharge | 充值虚拟币 |
| *UserCurrencyApi* | [**user_currency_transactions**](docs/UserCurrencyApi.md#user_currency_transactions) | **GET** /UserCurrency/{appKey}/CurrencyTransactions | 虚拟币交易记录 |

#### 💰 支付服务 API

| 类 | 方法 | HTTP 请求 | 描述 |
|---|---|---|---|
| *AlipayApi* | [**alipay_create_order**](docs/AlipayApi.md#alipay_create_order) | **POST** /Alipay/{appKey}/CreateOrder | 创建当面付订单 |
| *AlipayApi* | [**alipay_create_order_page_pay**](docs/AlipayApi.md#alipay_create_order_page_pay) | **POST** /Alipay/{appKey}/CreateOrderPagePay | 创建PC支付订单 |
| *AlipayApi* | [**alipay_create_order_wap_pay**](docs/AlipayApi.md#alipay_create_order_wap_pay) | **POST** /Alipay/{appKey}/CreateOrderWapPay | 创建WAP支付订单 |
| *AlipayApi* | [**alipay_order_detail**](docs/AlipayApi.md#alipay_order_detail) | **GET** /Alipay/{appKey}/OrderDetail | 获取订单详情 |
| *AlipayApi* | [**alipay_order_refund**](docs/AlipayApi.md#alipay_order_refund) | **POST** /Alipay/{appKey}/OrderRefund | 发起订单退款 |
| *AlipayApi* | [**alipay_return_page_notify**](docs/AlipayApi.md#alipay_return_page_notify) | **POST** /Alipay/{appKey}/ReturnPageNotify | 支付成功回调通知 |
| *OrderApi* | [**order**](docs/OrderApi.md#order) | **GET** /Order/{appKey}/{id} | 获取订单详情 |
| *OrderApi* | [**order_create**](docs/OrderApi.md#order_create) | **POST** /Order/{appKey}/Create | 创建订单 |
| *OrderApi* | [**orders**](docs/OrderApi.md#orders) | **GET** /Order/{appKey} | 获取订单列表 |

#### 💾 数据存储 API

| 类 | 方法 | HTTP 请求 | 描述 |
|---|---|---|---|
| *StorageApi* | [**storage_aggregate**](docs/StorageApi.md#storage_aggregate) | **GET** /Storage/{appKey}/{table}/Aggregate | 聚合查询 |
| *StorageApi* | [**storage_delete**](docs/StorageApi.md#storage_delete) | **DELETE** /Storage/{appKey}/{table}/{id} | 删除数据 |
| *StorageApi* | [**storage_detail**](docs/StorageApi.md#storage_detail) | **GET** /Storage/{appKey}/{table}/{id} | 数据详情 |
| *StorageApi* | [**storage_list**](docs/StorageApi.md#storage_list) | **GET** /Storage/{appKey}/{table} | 查询数据 |
| *StorageApi* | [**storage_post**](docs/StorageApi.md#storage_post) | **POST** /Storage/{appKey}/{table} | 添加数据 |
| *StorageApi* | [**storage_put**](docs/StorageApi.md#storage_put) | **PUT** /Storage/{appKey}/{table}/{id} | 更新数据 |

#### 📁 文件管理 API

| 类 | 方法 | HTTP 请求 | 描述 |
|---|---|---|---|
| *FileApi* | [**file_create_folder**](docs/FileApi.md#file_create_folder) | **POST** /File/{appKey}/CreateFolder | 创建文件夹 |
| *FileApi* | [**file_delete**](docs/FileApi.md#file_delete) | **DELETE** /File/{appKey} | 删除文件 / 文件夹 |
| *FileApi* | [**file_rename**](docs/FileApi.md#file_rename) | **POST** /File/{appKey}/Rename | 重命名文件 / 文件夹 |
| *FileApi* | [**file_upload**](docs/FileApi.md#file_upload) | **POST** /File/{appKey}/Upload | 上传文件 |
| *FileApi* | [**files**](docs/FileApi.md#files) | **GET** /File/{appKey} | 获取文件列表 |

#### 🔗 第三方集成 API

| 类 | 方法 | HTTP 请求 | 描述 |
|---|---|---|---|
| *WechatApi* | [**confirm_qr_code_login**](docs/WechatApi.md#confirm_qr_code_login) | **POST** /Wechat/{appKey}/QR-Auth/Confirm-Login | 确认二维码登录请求 |
| *WechatApi* | [**confirm_qr_code_registration**](docs/WechatApi.md#confirm_qr_code_registration) | **POST** /Wechat/{appKey}/QR-Auth/Confirm-Register | 确认二维码注册请求 |
| *WechatApi* | [**initiate_qr_auth_session**](docs/WechatApi.md#initiate_qr_auth_session) | **POST** /Wechat/{appKey}/QR-Auth/Initiate | 初始化二维码认证会话 |
| *WechatApi* | [**scan_qr_code_for_auth**](docs/WechatApi.md#scan_qr_code_for_auth) | **POST** /Wechat/{appKey}/QR-Auth/Scan | 验证二维码扫描结果 |
| *WechatApi* | [**wechat_decrypt**](docs/WechatApi.md#wechat_decrypt) | **GET** /Wechat/{appKey}/Decrypt | 解密小程序用户数据 |
| *WechatApi* | [**wechat_generate_scheme**](docs/WechatApi.md#wechat_generate_scheme) | **POST** /Wechat/{appKey}/GenerateScheme | 生成小程序Scheme码 |
| *WechatApi* | [**wechat_js_code2_session**](docs/WechatApi.md#wechat_js_code2_session) | **GET** /Wechat/{appKey}/JSCode2Session | 校验小程序登录状态 |
| *WechatApi* | [**wechat_js_config**](docs/WechatApi.md#wechat_js_config) | **GET** /Wechat/{appKey}/JSConfig | 配置公众号JS SDK |
| *WechatApi* | [**wechat_msg_sec_check**](docs/WechatApi.md#wechat_msg_sec_check) | **POST** /Wechat/{appKey}/MsgSecCheck | 小程序内容安全检测 |
| *WechatApi* | [**wechat_subscribe_msg**](docs/WechatApi.md#wechat_subscribe_msg) | **POST** /Wechat/{appKey}/SubscribeMSG | 发送公众号一次性订阅消息 |
| *WechatApi* | [**wechat_subscribe_send**](docs/WechatApi.md#wechat_subscribe_send) | **POST** /Wechat/{appKey}/SubscribeSend | 发送小程序订阅消息 |
| *WechatApi* | [**wechat_url_link_generate**](docs/WechatApi.md#wechat_url_link_generate) | **POST** /Wechat/{appKey}/UrlLinkGenerate | 生成小程序URL跳转链接 |
| *WechatApi* | [**wechat_user_info**](docs/WechatApi.md#wechat_user_info) | **GET** /Wechat/{appKey}/UserInfo | 获取公众号H5 UnionID |
| *WechatApi* | [**wechat_wxa_code_get**](docs/WechatApi.md#wechat_wxa_code_get) | **POST** /Wechat/{appKey}/WXACodeGet | 获取小程序码（普通） |
| *WechatApi* | [**wechat_wxa_code_get_unlimited**](docs/WechatApi.md#wechat_wxa_code_get_unlimited) | **POST** /Wechat/{appKey}/WXACodeGetUnlimited | 获取小程序码（无限制） |
| *DingTalkApi* | [**ding_talk_user_info**](docs/DingTalkApi.md#ding_talk_user_info) | **GET** /DingTalk/{appKey}/UserInfo | 获取用户资料 |

#### 🏢 应用管理 API

| 类 | 方法 | HTTP 请求 | 描述 |
|---|---|---|---|
| *AppApi* | [**app_info**](docs/AppApi.md#app_info) | **GET** /App/{appKey}/Info | 应用详情 |

</details>

### 数据模型文档

所有 API 请求和响应使用的数据结构，请参考：

#### 核心模型
- [User](docs/User.md) - 用户信息模型
- [Order](docs/Order.md) - 订单信息模型
- [TokenModel](docs/TokenModel.md) - 令牌模型

#### 用户相关模型
- [GetUserProfileResult](docs/GetUserProfileResult.md) - 用户资料结果
- [UserCurrency](docs/UserCurrency.md) - 用户虚拟货币
- [FollowerModel](docs/FollowerModel.md) - 关注者模型
- [GeoLocationModel](docs/GeoLocationModel.md) - 地理位置模型

#### 支付相关模型
- [AlipayTradeQueryResponse](docs/AlipayTradeQueryResponse.md) - 支付宝交易查询响应
- [AlipayTradeRefundResponse](docs/AlipayTradeRefundResponse.md) - 支付宝退款响应
- [CreateOrderRequest](docs/CreateOrderRequest.md) - 创建订单请求
- [CreateOrderResult](docs/CreateOrderResult.md) - 创建订单结果

#### 存储相关模型
- [StorageListResult](docs/StorageListResult.md) - 存储查询结果

#### 文件管理模型
- [FileListResult](docs/FileListResult.md) - 文件列表结果
- [FileItem](docs/FileItem.md) - 文件项目
- [DirectoryItem](docs/DirectoryItem.md) - 目录项目

#### 微信相关模型
- [WechatJsConfigResult](docs/WechatJsConfigResult.md) - 微信JS配置结果
- [UserQrCodeScanResult](docs/UserQrCodeScanResult.md) - 二维码扫描结果

#### API 响应包装器
- [ObjectApiResponse](docs/ObjectApiResponse.md) - 通用对象响应
- [StringApiResponse](docs/StringApiResponse.md) - 字符串响应
- [BooleanApiResponse](docs/BooleanApiResponse.md) - 布尔值响应
- [Int64ApiResponse](docs/Int64ApiResponse.md) - 整数响应

[查看完整模型列表...](docs/) 