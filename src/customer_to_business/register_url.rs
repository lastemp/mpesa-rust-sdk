use reqwest::StatusCode;

use crate::{
    models::models::{RegisterUrlInputDetails, RegisterUrlResponseData},
    util::util::{build_headers, build_register_url_data},
};

pub async fn register_url(
    register_url_details: RegisterUrlInputDetails,
    access_token: String,
) -> std::result::Result<RegisterUrlResponseData, String> {
    let api_url: String = register_url_details.get_api_url();
    let short_code: String = register_url_details.get_short_code();
    let response_type: String = register_url_details.get_response_type();
    let confirmation_url: String = register_url_details.get_confirmation_url();
    let validation_url: String = register_url_details.get_validation_url();

    let register_url_data =
        build_register_url_data(short_code, response_type, confirmation_url, validation_url);

    let client = reqwest::Client::new();

    let res = client
        .post(api_url)
        .headers(build_headers(access_token))
        .json(&register_url_data)
        .send()
        .await;

    match res {
        Err(_err) => {
            return Err(_err.to_string());
        }
        Ok(response) => match response.status() {
            StatusCode::OK => {
                match response.json::<RegisterUrlResponseData>().await {
                    Ok(register_url_response_data) => {
                        // Handle success case
                        return Ok(register_url_response_data);
                    }
                    Err(_err) => {
                        // Handle error case
                        return Err(_err.to_string());
                    }
                }
            }
            s => {
                let mut _x = String::from("Request failed processing, status code: ");
                _x.push_str(&s.to_string());
                return Err(_x.to_string());
            }
        },
    };
}
