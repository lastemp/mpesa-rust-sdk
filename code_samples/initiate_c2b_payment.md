# initiate c2b payment

This functionality initiates c2b payment request.

## c2b_payment

This code sample shows how to invoke function c2b_payment of the sdk.

```rust
use mpesa_rust_sdk::MpesaGateway;
use mpesa_rust_sdk::models::{CustomerToBusinessPaymentInputDetails, CustomerToBusinessPaymentResponseData, CustomerToBusinessPaymentErrorResponseData};

let consumer_key: String = String::from("***");
let consumer_secret: String = String::from("***");
let auth_token_url: String = String::from("***");
let stk_push_url: String =
	String::from("https://sandbox.safaricom.co.ke/mpesa/stkpush/v1/processrequest");
let business_short_code: String = String::from("***");
let pass_key: String =
	String::from("***");
let time_stamp: String = Local::now().format("%Y%m%d%H%M%S").to_string(); //"YYYYMMDDHHmmss";
let transaction_type: String = String::from("CustomerPayBillOnline");
let _amount: u32 = 1;
let party_a: u64 = 2547***;
let party_b: u32 = 174***;
let phone_number: u64 = 2547***;
let call_back_url: String = String::from("https://mydomain.com/path");
let account_reference: String = String::from("Company X");
let transaction_desc: String = String::from("Payment of X");

// _password = Shortcode+Passkey+Timestamp)
let short_code = &business_short_code;
let mut _password: String = short_code.to_string();
_password.push_str(&pass_key);
_password.push_str(&time_stamp);
let encoded_password = general_purpose::STANDARD.encode(_password);

let _result = CustomerToBusinessPaymentInputDetails::new(
	stk_push_url,
	business_short_code,
	encoded_password,
	time_stamp,
	transaction_type,
	_amount,
	party_a,
	party_b,
	phone_number,
	call_back_url,
	account_reference,
	transaction_desc,
);
	
if let Ok(customer_to_business_details) = _result {
	let _result = MpesaGateway::new(consumer_key, consumer_secret, auth_token_url);
	if let Ok(mpesa_gateway) = _result {
		// Initiate the request through the sdk
		let _output = mpesa_gateway.c2b_payment(customer_to_business_details);
		let _result: std::result::Result<
			(
				Option<CustomerToBusinessPaymentResponseData>,
				Option<CustomerToBusinessPaymentErrorResponseData>,
			),
			String,
		> = _output.await;

		match _result {
			Ok(customer_to_business_data) => {
				// Lets unpack the tuple
				let (
					customer_to_business_response_data,
					customer_to_business_error_response_data,
				) = customer_to_business_data;

				// customer_to_business_response_data
				if let Some(response_data) = customer_to_business_response_data {
					println!("customer_to_business_response_data: {:?}", &response_data);
				}

				// customer_to_business_error_response_data
				if let Some(response_data) = customer_to_business_error_response_data {
					println!(
						"customer_to_business_error_response_data: {:?}",
						&response_data
					);
				}
			}
			Err(e) => {
				println!("Processing Error: {:?}", e)
			}
		}
	};
};
```
