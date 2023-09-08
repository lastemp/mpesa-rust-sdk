use reqwest::StatusCode;

use crate::{
    models::models::{
        CustomerToBusinessPaymentErrorResponseData, CustomerToBusinessPaymentInputDetails,
        CustomerToBusinessPaymentResponseData,
    },
    util::util::{build_customer_to_business_data, build_headers},
};

// network initiated push
pub async fn c2b_payment(
    customer_to_business_payment_details: CustomerToBusinessPaymentInputDetails,
    access_token: String,
) -> std::result::Result<
    (
        Option<CustomerToBusinessPaymentResponseData>,
        Option<CustomerToBusinessPaymentErrorResponseData>,
    ),
    String,
> {
    let api_url: String = customer_to_business_payment_details.get_api_url();
    let business_short_code: String =
        customer_to_business_payment_details.get_business_short_code();
    let _password: String = customer_to_business_payment_details.get_password();
    let time_stamp: String = customer_to_business_payment_details.get_time_stamp();
    let transaction_type: String = customer_to_business_payment_details.get_transaction_type();
    let _amount: u32 = customer_to_business_payment_details.get_amount();
    let party_a: u64 = customer_to_business_payment_details.get_party_a();
    let party_b: u32 = customer_to_business_payment_details.get_party_b();
    let phone_number: u64 = customer_to_business_payment_details.get_phone_number();
    let call_back_url: String = customer_to_business_payment_details.get_call_back_url();
    let account_reference: String = customer_to_business_payment_details.get_account_reference();
    let transaction_desc: String = customer_to_business_payment_details.get_transaction_desc();

    let customer_to_business_data = build_customer_to_business_data(
        business_short_code,
        _password,
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

    let client = reqwest::Client::new();

    let res = client
        .post(api_url)
        .headers(build_headers(access_token))
        .json(&customer_to_business_data)
        .send()
        .await;

    match res {
        Err(_err) => {
            return Err(_err.to_string());
        }
        Ok(response) => match response.status() {
            StatusCode::OK => {
                match response
                    .json::<CustomerToBusinessPaymentResponseData>()
                    .await
                {
                    Ok(customer_to_business_response_data) => {
                        // Handle success case
                        let customer_to_business_error_response_data = None;
                        let my_output = (
                            Some(customer_to_business_response_data),
                            customer_to_business_error_response_data,
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
                match response
                    .json::<CustomerToBusinessPaymentErrorResponseData>()
                    .await
                {
                    Ok(customer_to_business_error_response_data) => {
                        // Handle success case
                        let customer_to_business_response_data = None;
                        let my_output = (
                            customer_to_business_response_data,
                            Some(customer_to_business_error_response_data),
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
