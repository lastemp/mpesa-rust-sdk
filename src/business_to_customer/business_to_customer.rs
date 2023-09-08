use reqwest::StatusCode;

use crate::{
    models::models::{
        BusinessToCustomerErrorResponseData, BusinessToCustomerInputDetails,
        BusinessToCustomerResponseData,
    },
    util::util::{build_business_to_customer_data, build_headers},
};

pub async fn b2c(
    business_to_customer_details: BusinessToCustomerInputDetails,
    access_token: String,
) -> std::result::Result<
    (
        Option<BusinessToCustomerResponseData>,
        Option<BusinessToCustomerErrorResponseData>,
    ),
    String,
> {
    let api_url: String = business_to_customer_details.get_api_url();
    let originator_conversation_id = business_to_customer_details.get_originator_conversation_id();
    let initiator_name: String = business_to_customer_details.get_initiator_name();
    let security_credential: String = business_to_customer_details.get_security_credential();
    let command_id: String = business_to_customer_details.get_command_id();
    let amount: u32 = business_to_customer_details.get_amount();
    let party_a: u32 = business_to_customer_details.get_party_a();
    let party_b: String = business_to_customer_details.get_party_b();
    let _remarks: String = business_to_customer_details.get_remarks();
    let queue_time_out_url: String = business_to_customer_details.get_queue_time_out_url();
    let result_url: String = business_to_customer_details.get_result_url();
    let _occassion: String = business_to_customer_details.get_occassion();

    let business_to_customer_data = build_business_to_customer_data(
        originator_conversation_id,
        initiator_name,
        security_credential,
        command_id,
        amount,
        party_a,
        party_b,
        _remarks,
        queue_time_out_url,
        result_url,
        _occassion,
    );

    let client = reqwest::Client::new();

    let res = client
        .post(api_url)
        .headers(build_headers(access_token))
        .json(&business_to_customer_data)
        .send()
        .await;

    match res {
        Err(_err) => {
            return Err(_err.to_string());
        }
        Ok(response) => match response.status() {
            StatusCode::OK => {
                match response.json::<BusinessToCustomerResponseData>().await {
                    Ok(business_to_customer_response_data) => {
                        // Handle success case
                        let business_to_customer_error_response_data = None;

                        let my_output = (
                            Some(business_to_customer_response_data),
                            business_to_customer_error_response_data,
                        );

                        return Ok(my_output);
                    }
                    Err(_err) => {
                        // Handle error case
                        return Err(_err.to_string());
                    }
                }
            }
            s => {
                match response.json::<BusinessToCustomerErrorResponseData>().await {
                    Ok(business_to_customer_error_response_data) => {
                        // Handle success case
                        let business_to_customer_response_data = None;

                        let my_output = (
                            business_to_customer_response_data,
                            Some(business_to_customer_error_response_data),
                        );
                        return Ok(my_output);
                    }
                    Err(_err) => {
                        // Handle error case
                        return Err(_err.to_string());
                    }
                }
            }
        },
    };
}
