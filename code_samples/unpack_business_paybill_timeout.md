# unpack business paybill timeout data

This functionality unpacks json "business paybill timeout" request data.

## get_business_paybill_timeout

This code sample shows how to unpack json "business paybill timeout" data.

```rust
use mpesa_rust_sdk::MpesaGateway;
use mpesa_rust_sdk::models::BusinessPayBillFailedData;

#[post("/businesspaybill/timeout")]
pub(crate) async fn get_business_paybill_timeout(
    result_data: web::Json<BusinessPayBillFailedData>,
) -> impl Responder {
    let result_type = &result_data.Result.ResultType;
    let result_code = &result_data.Result.ResultCode;
    let result_desc = &result_data.Result.ResultDesc;
    let originator_conversation_id = &result_data.Result.OriginatorConversationID;
    let conversation_id = &result_data.Result.ConversationID;
    let transaction_id = &result_data.Result.TransactionID;
    let result_parameter = &result_data.Result.ResultParameters;
    let reference_data = &result_data.Result.ReferenceData.ReferenceItem;

    let consumer_key: String = String::from("***");
    let consumer_secret: String = String::from("***");
    let auth_token_url: String = String::from("***");

    let _result = MpesaGateway::new(consumer_key, consumer_secret, auth_token_url);
    if let Ok(mpesa_gateway) = _result {
        let business_paybill_timeout_parameters_output_details = mpesa_gateway
            .get_business_paybill_timeout_parameters_output_details(
                result_parameter,
                reference_data,
            );

        println!(
            "business_paybill_timeout_parameters_output_details: {:?}",
            &business_paybill_timeout_parameters_output_details
        );
    } else if let Err(e) = _result {
        println!("Data Error: {:?}", e)
    } else {
        println!("Unexpected error occured during processing")
    };

    format!("")
}
```
