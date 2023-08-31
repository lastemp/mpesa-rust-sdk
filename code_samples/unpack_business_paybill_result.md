# unpack business paybill result data

This functionality unpacks json "business paybill result" request data.

## get_business_paybill_result

This code sample shows how to unpack json "business paybill result" data.

```rust
use mpesa_rust_sdk::MpesaGateway;
use mpesa_rust_sdk::models::BusinessPayBillResultData;

#[post("/businesspaybill/result")]
pub(crate) async fn get_business_paybill_result(
    result_data: web::Json<BusinessPayBillResultData>,
) -> impl Responder {
    let result_type = &result_data.Result.ResultType;
    let result_code = &result_data.Result.ResultCode;
    let result_desc = &result_data.Result.ResultDesc;
    let originator_conversation_id = &result_data.Result.OriginatorConversationID;
    let conversation_id = &result_data.Result.ConversationID;
    let transaction_id = &result_data.Result.TransactionID;
    let result_parameters = &result_data.Result.ResultParameters;
    let reference_data = &result_data.Result.ReferenceData;

    let consumer_key: String = String::from("***");
    let consumer_secret: String = String::from("***");
    let auth_token_url: String = String::from("***");

    let _result = MpesaGateway::new(consumer_key, consumer_secret, auth_token_url);
    if let Ok(mpesa_gateway) = _result {
        let business_paybill_result_parameters_output_details =
            mpesa_gateway.get_business_paybill_result_parameters_output_details(result_parameters);

        let business_paybill_Reference_item_output_details =
            mpesa_gateway.get_business_paybill_Reference_item_output_details(reference_data);

        println!(
            "business_paybill_result_parameters_output_details: {:?}",
            &business_paybill_result_parameters_output_details
        );

        println!(
            "business_paybill_Reference_item_output_details: {:?}",
            &business_paybill_Reference_item_output_details
        );
    } else if let Err(e) = _result {
        println!("Data Error: {:?}", e)
    } else {
        println!("Unexpected error occured during processing")
    };

    format!("")
}
```
