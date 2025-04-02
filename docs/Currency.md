# Currency

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | 货币的唯一标识符。 | [optional]
**name** | Option<**String**> | 货币的名称，例如 '人民币', '美元' 等。 | [optional]
**code** | Option<**String**> | 货币的ISO标准代码，例如 'CNY', 'USD' 等。 | [optional]
**symbol** | Option<**String**> | 货币的符号，例如 '$', '€', '¥' 等。 | [optional]
**issuer** | Option<**String**> | 发行该货币的机构或国家名称。 | [optional]
**currency_type** | Option<**String**> | 货币的类型，例如 '法定货币', '加密货币' 等。 | [optional]
**tags** | Option<**String**> | 用于分类或标记货币的标签。 | [optional]
**status** | Option<**bool**> | 指示货币当前是否可用。 | [optional]
**enable_virtual_recharge** | Option<**bool**> | 指示该货币是否允许进行虚拟充值。 | [optional]
**enable_virtual_consume** | Option<**bool**> | 指示该货币是否允许进行虚拟消费。 | [optional]
**description** | Option<**String**> | 货币的详细描述信息。 | [optional]
**fiat_exchange_rate** | Option<**i64**> | 该货币与法定货币的兑换比率。 | [optional]
**fiat_daily_recharge_limit** | Option<**i64**> | 每日通过法定货币充值的最大限额。 | [optional]
**total_supply** | Option<**i64**> | 货币的总供应量，0 表示无限制。 | [optional]
**create_date** | Option<**String**> | 货币记录的创建日期，默认为当前时间。 | [optional]
**last_update** | Option<**String**> | 货币记录的最后更新日期，默认为当前时间。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


