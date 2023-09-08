# initiate register url

This functionality initiates register url request.

## register_url

This code sample shows how to invoke function register_url of the sdk.

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
	let consumer_key: String = get_settings_details(&data, String::from("consumerkeympesa"));
	let consumer_secret: String =
		get_settings_details(&data, String::from("consumersecretmpesa"));
	let auth_token_url: String = get_settings_details(&data, String::from("authtokenurlmpesa"));

	let _result = MpesaGateway::new(consumer_key, consumer_secret, auth_token_url);
	if let Ok(mpesa_gateway) = _result {
		// Initiate the request through the sdk
		let _output = mpesa_gateway.register_url(register_url_details);

		let _result: std::result::Result<RegisterUrlResponseData, String> = _output.await;

		if let Ok(register_url_response_data) = _result {
			println!(
				"register_url_response_data: {:?}",
				&register_url_response_data
			);
		} else if let Err(e) = _result {
			println!("Processing Error: {:?}", e)
		}
	};
};
```
