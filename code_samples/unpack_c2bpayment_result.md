# unpack c2b payment result data

This functionality unpacks json "c2b payment result" request data.

## get_c2bpayment_result

This code sample shows how to unpack json "c2b payment result" data.

```rust
use mpesa_rust_sdk::MpesaGateway;
use mpesa_rust_sdk::models::CustomerToBusinessPaymentResultData;

#[post("/c2bpayment/result")]
pub(crate) async fn get_c2bpayment_result(
    result_data: web::Json<CustomerToBusinessPaymentResultData>,
) -> impl Responder {
    let merchant_request_id = &result_data.Body.stkCallback.MerchantRequestID;
    let checkout_request_id = &result_data.Body.stkCallback.CheckoutRequestID;
    let result_code = &result_data.Body.stkCallback.ResultCode;
    let result_desc = &result_data.Body.stkCallback.ResultDesc;

    let mut item_details = Vec::new();

    let my_item = Item { Item: item_details };

    let callback_meta_data = &result_data
        .Body
        .stkCallback
        .CallbackMetadata
        .as_ref()
        .unwrap_or(&my_item);
    let list_of_items = &callback_meta_data.Item;

    println!("merchant_request_id: {:?}", &merchant_request_id);
    println!("checkout_request_id: {:?}", &checkout_request_id);
    println!("result_code: {:?}", &result_code);
    println!("result_desc: {:?}", &result_desc);

    let consumer_key: String = String::from("***");
    let consumer_secret: String = String::from("***");
    let auth_token_url: String = String::from("***");

    let _result = MpesaGateway::new(consumer_key, consumer_secret, auth_token_url);
    if let Ok(mpesa_gateway) = _result {
        let c2b_payment_result_parameters_output_details =
            mpesa_gateway.get_c2b_payment_result_parameters_output_details(list_of_items);

        println!(
            "c2b_payment_result_parameters_output_details: {:?}",
            &c2b_payment_result_parameters_output_details
        );
    } else if let Err(e) = _result {
        println!("Data Error: {:?}", e)
    } else {
        println!("Unexpected error occured during processing")
    };

    format!("")
}
```
