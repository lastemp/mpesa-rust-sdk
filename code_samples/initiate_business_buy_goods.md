# initiate business buy goods

This functionality initiates business buy goods request.

## business_buy_goods

This code sample shows how to invoke function business_buy_goods of the sdk.

```rust
use mpesa_rust_sdk::MpesaGateway;
use mpesa_rust_sdk::models::{BusinessBuyGoodsInputDetails, BusinessBuyGoodsResponseData, BusinessBuyGoodsErrorResponseData};

let consumer_key: String = String::from("***");
let consumer_secret: String = String::from("***");
let auth_token_url: String = String::from("***");

let b2b_payment_request_url: String =
	String::from("https://sandbox.safaricom.co.ke/mpesa/b2b/v1/paymentrequest");
let _initiator: String = String::from("***");
let security_credential: String = String::from("***");
let command_id: String = String::from("BusinessBuyGoods");
let sender_identifier_type: String = String::from("4");
let reciever_identifier_type: String = String::from("4");
let _amount: u32 = 145;
let party_a: String = String::from("***");
let party_b: String = String::from("***");
let account_reference: String = String::from("***");
let _requester: String = String::from("2547***");
let _remarks: String = String::from("ok");
let queue_time_out_url: String = String::from("https://mydomain.com/b2b/queue/");
let result_url: String = String::from("https://mydomain.com/b2b/result/");

let _result = BusinessBuyGoodsInputDetails::new(
	b2b_payment_request_url,
	_initiator,
	security_credential,
	command_id,
	sender_identifier_type,
	reciever_identifier_type,
	_amount,
	party_a,
	party_b,
	account_reference,
	_requester,
	_remarks,
	queue_time_out_url,
	result_url,
);
	
if let Ok(business_buy_goods_details) = _result {
	let _result = MpesaGateway::new(consumer_key, consumer_secret, auth_token_url);
	if let Ok(mpesa_gateway) = _result {
		let _output = mpesa_gateway.business_buy_goods(business_buy_goods_details);

		let _result: std::result::Result<
			(
				Option<BusinessBuyGoodsResponseData>,
				Option<BusinessBuyGoodsErrorResponseData>,
			),
			String,
		> = _output.await;

		match _result {
			Ok(business_buy_goods_data) => {
				// Lets unpack the tuple
				let (business_buy_goods_response_data, business_buy_goods_error_response_data) =
					business_buy_goods_data;

				// business_buy_goods_response_data
				if let Some(response_data) = business_buy_goods_response_data {
					println!("business_buy_goods_response_data: {:?}", &response_data);
				}

				// business_buy_goods_error_response_data
				if let Some(response_data) = business_buy_goods_error_response_data {
					println!(
						"business_buy_goods_error_response_data: {:?}",
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
