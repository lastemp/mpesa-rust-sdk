# unpack b2c result data

This functionality unpacks json "b2c result" request data.

## get_b2c_result

This code sample shows how to unpack json "b2c result" data.

```rust
use mpesa_rust_sdk::MpesaGateway;
use mpesa_rust_sdk::models::B2CResultData;

#[post("/b2c/result")]
pub(crate) async fn get_b2c_result(
    result_data: web::Json<B2CResultData>,
) -> impl Responder {
    let result_type = &result_data.Result.ResultType;
    let result_code = &result_data.Result.ResultCode;
    let result_desc = &result_data.Result.ResultDesc;
    let originator_conversation_id = &result_data.Result.OriginatorConversationID;
    let conversation_id = &result_data.Result.ConversationID;
    let transaction_id = &result_data.Result.TransactionID;
    let result_parameters = &result_data.Result.ResultParameters;
    let reference_item = &result_data.Result.ReferenceData.ReferenceItem;
    let queue_timeout_url = &reference_item.Value;

    let consumer_key: String = String::from("***");
    let consumer_secret: String = String::from("***");
    let auth_token_url: String = String::from("***");

    let _result = MpesaGateway::new(consumer_key, consumer_secret, auth_token_url);
    if let Ok(mpesa_gateway) = _result {
        let b2c_result_parameters_output_details =
            mpesa_gateway.get_b2c_result_parameters_output_details(result_parameters);

        println!(
            "b2c_result_parameters_output_details: {:?}",
            &b2c_result_parameters_output_details
        );

        println!("result_desc: {:?}", &result_desc);

        println!(
            "originator_conversation_id: {:?}",
            &originator_conversation_id
        );
    } else if let Err(e) = _result {
        println!("Data Error: {:?}", e)
    } else {
        println!("Unexpected error occured during processing")
    };

    format!("")
}
```
