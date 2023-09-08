use reqwest::StatusCode;

use crate::{
    models::models::{
        BusinessPayBillErrorResponseData, BusinessPayBillInputDetails, BusinessPayBillResponseData,
    },
    util::util::{build_business_paybill_data, build_headers},
};

pub async fn pay_bill(
    business_paybill_details: BusinessPayBillInputDetails,
    access_token: String,
) -> std::result::Result<
    (
        Option<BusinessPayBillResponseData>,
        Option<BusinessPayBillErrorResponseData>,
    ),
    String,
> {
    let api_url: String = business_paybill_details.get_api_url();
    let _initiator: String = business_paybill_details.get_initiator();
    let security_credential: String = business_paybill_details.get_security_credential();
    let command_id: String = business_paybill_details.get_command_id();
    let sender_identifier_type: String = business_paybill_details.get_sender_identifier_type();
    let reciever_identifier_type: String = business_paybill_details.get_reciever_identifier_type();
    let _amount: u32 = business_paybill_details.get_amount();
    let party_a: String = business_paybill_details.get_party_a();
    let party_b: String = business_paybill_details.get_party_b();
    let account_reference: String = business_paybill_details.get_account_reference();
    let _requester: String = business_paybill_details.get_requester();
    let _remarks: String = business_paybill_details.get_remarks();
    let queue_time_out_url: String = business_paybill_details.get_queue_time_out_url();
    let result_url: String = business_paybill_details.get_result_url();

    let business_paybill_data = build_business_paybill_data(
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

    let client = reqwest::Client::new();

    let res = client
        .post(api_url)
        .headers(build_headers(access_token))
        .json(&business_paybill_data)
        .send()
        .await;

    match res {
        Err(_err) => {
            return Err(_err.to_string());
        }
        Ok(response) => match response.status() {
            StatusCode::OK => {
                match response.json::<BusinessPayBillResponseData>().await {
                    Ok(business_paybill_response_data) => {
                        // Handle success case
                        let business_paybill_error_response_data = None;
                        let my_output = (
                            Some(business_paybill_response_data),
                            business_paybill_error_response_data,
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
                match response.json::<BusinessPayBillErrorResponseData>().await {
                    Ok(business_paybill_error_response_data) => {
                        // Handle success case
                        let business_paybill_response_data = None;
                        let my_output = (
                            business_paybill_response_data,
                            Some(business_paybill_error_response_data),
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
