# initiate b2c payment

This functionality initiates b2c request.

## get_b2c

This code sample shows how to invoke function get_b2c of the sdk.

```rust
fn get_business_to_customer_details(
    my_party_b: String,
    my_amount: u32,
    my_command_id: String,
    my_remarks: String,
    my_occassion: String,
) -> Result<BusinessToCustomerInputDetails, String> {
    let my_api_url: String = String::from("***");
    let my_originator_conversation_id = Local::now().format("%Y%m%d%H%M%S%3f").to_string(); // test only
    let my_initiator_name: String = String::from("***");
    let my_security_credential: String = String::from("***");
    let my_party_a: String = String::from("***");
    let my_queue_time_out_url: String = String::from("***");
    let my_result_url: String = String::from("***");

    let my_party_a: u32 = match my_party_a.parse::<u32>() {
        Ok(a) => a,
        Err(e) => 0,
    };

    let _result = BusinessToCustomerInputDetails::new(
        my_api_url,
        my_originator_conversation_id,
        my_initiator_name,
        my_security_credential,
        my_command_id,
        my_amount,
        my_party_a,
        my_party_b,
        my_remarks,
        my_queue_time_out_url,
        my_result_url,
        my_occassion,
    );

    _result
}
```

```rust
use mpesa_rust_sdk::MpesaGateway;
use mpesa_rust_sdk::models::{BusinessToCustomerInputDetails, BusinessToCustomerResponseData, BusinessToCustomerErrorResponseData};

const TRANSACTION_COMMAND_ID: &str = "BusinessPayment"; //SalaryPayment, BusinessPayment, PromotionPayment
const TRANSACTION_REMARKS: &str = "Performance payment fees";
const TRANSACTION_OCCASSION: &str = "Performance payment fees";

let consumer_key: String = String::from("***");
let consumer_secret: String = String::from("***");
let auth_token_url: String = String::from("***");

let mobile_no = String::from("2547***");
let amount_paid: u32 = 1500;
let command_id = TRANSACTION_COMMAND_ID.to_string();
let _remarks = TRANSACTION_REMARKS.to_string();
let _occassion = TRANSACTION_OCCASSION.to_string();

let _result = get_business_to_customer_details(
	mobile_no.to_string(),
	amount_paid,
	command_id.to_string(),
	_remarks.to_string(),
	_occassion.to_string(),
);
	
if let Ok(business_to_customer_data) = _result {
	let _result = MpesaGateway::new(consumer_key, consumer_secret, auth_token_url);
	if let Ok(mpesa_gateway) = _result {
		let _output = mpesa_gateway.get_b2c(business_to_customer_data);

		let _result: std::result::Result<
			(
				BusinessToCustomerResponseData,
				BusinessToCustomerErrorResponseData,
			),
			reqwest::Error,
		> = _output.await;

		let (
			business_to_customer_response_data,
			business_to_customer_error_response_data,
			error_data,
		) = match _result {
			Ok(x) => {
				let (a, b) = x;
				(a, b, None)
			}
			Err(e) => {
				let a = BusinessToCustomerResponseData {
					OriginatorConversationID: None,
					ConversationID: None,
					ResponseCode: None,
					ResponseDescription: None,
				};

				let b = BusinessToCustomerErrorResponseData {
					requestId: None,
					errorCode: None,
					errorMessage: None,
				};

				(a, b, Some(e))
			}
		};

		println!(
			"business_to_customer_response_data: {:?}",
			&business_to_customer_response_data
		);

		println!(
			"business_to_customer_error_response_data: {:?}",
			&business_to_customer_error_response_data
		);
	} else if let Err(e) = _result {
		println!("Data Error: {:?}", e)
	} else {
		println!("Unexpected error occured during processing")
	};
} else if let Err(e) = _result {
	println!("Data Error: {:?}", e)
} else {
	println!("Unexpected error occured during processing")
};	
```
