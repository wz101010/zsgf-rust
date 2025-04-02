# CurrencyTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | 货币交易记录的唯一标识符。 | [optional]
**from_user_id** | Option<**i64**> | 发起交易的发送方用户ID，若为转账交易时必填。 | [optional]
**user_id** | Option<**i64**> | 进行货币交易的用户ID。 | [optional]
**transaction_type** | Option<**String**> | 货币交易的类型，例如 '消费', '奖励', '兑换', '转账' 等。 | [optional]
**currency_type** | Option<**String**> | 交易的货币类型，例如 'USD', 'CNY' 等。 | [optional]
**currency_change** | Option<**i64**> | 货币的变动数量，正数表示增加，负数表示减少。 | [optional]
**currency_balance** | Option<**f64**> | 交易完成后的货币余额。 | [optional]
**description** | Option<**String**> | 描述货币变动的具体原因或相关交易详情。 | [optional]
**status** | Option<**String**> | 货币交易的当前状态，例如 '成功', '失败', '待审核' 等。 | [optional]
**remark** | Option<**String**> | 交易的额外信息或管理员的备注。 | [optional]
**tags** | Option<**String**> | 用于分类或标记交易的标签。 | [optional]
**create_date** | Option<**String**> | 货币交易发生的时间，默认为当前时间。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


