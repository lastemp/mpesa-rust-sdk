# initiate register url

This functionality initiates register url request.

## get_register_url

This code sample shows how to invoke function get_register_url of the sdk.

```rust
fn get_register_url_details() -> Result<RegisterUrlInputDetails, String> {
    let api_url = String::from("***");
    let short_code = String::from("***");
    let response_type = String::from("***");
    let confirmation_url = String::from("***");
    let validation_url = String::from("***");
    
    let _result = RegisterUrlInputDetails::new(
        api_url,
        short_code,
        response_type,
        confirmation_url,
        validation_url,
    );

    _result
}
```

```rust
use mpesa_rust_sdk::MpesaGateway;
use mpesa_rust_sdk::models::{RegisterUrlInputDetails, RegisterUrlResponseData};

let _result = get_register_url_details();

if let Ok(register_url_details) = _result {
	let consumer_key: String = String::from("***");
	let consumer_secret: String = String::from("***");
	let auth_token_url: String = String::from("***");

	let _result = MpesaGateway::new(consumer_key, consumer_secret, auth_token_url);
	if let Ok(mpesa_gateway) = _result {
		let _output = mpesa_gateway.get_register_url(register_url_details);

		let _result: std::result::Result<RegisterUrlResponseData, reqwest::Error> =
			_output.await;

		let (register_url_response_data, error_data) = match _result {
			Ok(a) => (a, None),
			Err(e) => {
				let a = RegisterUrlResponseData {
					OriginatorCoversationID: None,
					ConversationID: None,
					ResponseDescription: None,
				};

				(a, Some(e))
			}
		};

		println!(
			"register_url_response_data: {:?}",
			&register_url_response_data
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
