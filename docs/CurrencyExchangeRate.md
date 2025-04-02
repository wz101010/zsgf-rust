# CurrencyExchangeRate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | 货币兑换比率的唯一标识符。 | [optional]
**from_currency_code** | Option<**String**> | 兑换的源货币代码，例如 'USD'。 | [optional]
**to_currency_code** | Option<**String**> | 兑换的目标货币代码，例如 'CNY'。 | [optional]
**exchange_rate** | Option<**i64**> | 从源货币到目标货币的兑换比率。例如，1 USD = 6.5 CNY。 | [optional]
**description** | Option<**String**> | 兑换比率的详细描述信息。 | [optional]
**create_date** | Option<**String**> | 货币兑换比率的创建日期，默认为当前时间。 | [optional]
**last_update** | Option<**String**> | 货币兑换比率的最后更新日期，默认为当前时间。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


