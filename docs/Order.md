# Order

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | 订单的唯一标识符。 | [optional]
**user_id** | Option<**i64**> | 创建订单的用户ID。 | [optional]
**pay_type** | Option<**String**> | 订单的支付类型，例如 '信用卡', '支付宝', '微信支付' 等。 | [optional]
**amount** | Option<**f64**> | 订单的总金额。 | [optional]
**order_no** | Option<**String**> | 订单的唯一编号，通常由系统生成。 | [optional]
**trade_no** | Option<**String**> | 与订单关联的交易编号，通常由支付平台提供。 | [optional]
**status** | Option<**String**> | 订单的当前状态，例如 '待支付', '已完成', '已取消' 等。 | [optional]
**product_type** | Option<**String**> | 订单中商品的类型分类。 | [optional]
**product_id** | Option<**String**> | 订单中商品的唯一标识符。 | [optional]
**product_name** | Option<**String**> | 订单中商品的名称。 | [optional]
**allow_refund** | Option<**bool**> | 指示订单是否允许进行退款操作。 | [optional]
**allow_refund_until** | Option<**String**> | 订单允许进行退款操作的截止时间。 | [optional]
**tags** | Option<**String**> | 用于分类或标记订单的标签。 | [optional]
**remark** | Option<**String**> | 订单的额外备注信息。 | [optional]
**description** | Option<**String**> | 订单的详细描述信息。 | [optional]
**order_pay_time** | Option<**String**> | 订单完成支付的时间。 | [optional]
**expire_time** | Option<**String**> | 订单的过期时间，超过该时间订单将自动取消。 | [optional]
**create_date** | Option<**String**> | 订单的创建时间，默认为当前时间。 | [optional]
**last_update** | Option<**String**> | 订单的最后更新时间，默认为当前时间。 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


