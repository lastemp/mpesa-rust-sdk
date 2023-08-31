# unpack b2c timeout data

This functionality unpacks json "b2c timeout" request data.

## get_b2c_timeout

This code sample shows how to unpack json "b2c timeout" data.

```rust
use mpesa_rust_sdk::models::B2CFailedData;

#[post("/b2c/timeout")]
pub(crate) async fn get_b2c_timeout(
    result_data: web::Json<B2CFailedData>,
) -> impl Responder {
    let result_type = &result_data.Result.ResultType;
    let result_code = &result_data.Result.ResultCode;
    let result_desc = &result_data.Result.ResultDesc;
    let originator_conversation_id = &result_data.Result.OriginatorConversationID;
    let conversation_id = &result_data.Result.ConversationID;
    let transaction_id = &result_data.Result.TransactionID;
    let reference_item = &result_data.Result.ReferenceData.ReferenceItem;
    let queue_timeout_url = &reference_item.Value;

    println!("result_desc: {:?}", &result_desc);
    println!(
        "originator_conversation_id: {:?}",
        &originator_conversation_id
    );
    println!("queue_timeout_url: {:?}", &queue_timeout_url);

    format!("")
}
```
