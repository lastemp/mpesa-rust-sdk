extern crate base64;

use crate::api_layer::{generate_auth_token, register_url};
use crate::mpesa::MpesaGateway;
use crate::{
    models::{
        B2CFailedData, B2CResultData, BusinessToCustomerInputDetails, C2bData,
        ConfirmationResponseData, CustomerToBusinessPaymentInputDetails, MixedTypeValue,
        RegisterUrlInputDetails, ValidationResponseData,
    },
    persistence::{
        create_incoming_c2b_mpesa_confirmation_requests,
        create_incoming_c2b_mpesa_validation_requests, get_mpesa_access_token,
        get_settings_details,
    },
};
use actix_web::{get, post, web, App, HttpRequest, HttpServer, Responder};
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use chrono::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
use std::str;

const C2B_BILL_TYPE: &str = "C2B";
const TRANSACTION_COMMAND_ID: &str = "BusinessPayment"; //SalaryPayment, BusinessPayment, PromotionPayment
const TRANSACTION_REMARKS: &str = "Performance payment fees";
const TRANSACTION_OCCASSION: &str = "Performance payment fees";

#[get("/")]
pub(crate) async fn index() -> impl Responder {
    format!("")
}

#[get("/generateauth")]
pub(crate) async fn generate_auth(data: web::Data<Pool>) -> impl Responder {
    //let api_key = get_api_key(&data);
    let consumer_key = get_settings_details(&data, String::from("consumerkeympesa"));
    let consumer_secret = get_settings_details(&data, String::from("consumersecretmpesa"));
    let auth_token_url = get_settings_details(&data, String::from("authtokenurlmpesa"));
    let mpesa_gateway = MpesaGateway::new(consumer_key, consumer_secret, auth_token_url);
    /*
    tokio::spawn(async move {
        // Process each request concurrently.
        generate_auth_token(data, api_key, api_url).await;
    });
    */
    /*
    let xy = mpesa_gateway.get_auth_token();
    let access_token: String = xy.await;
    println!("access_token: {:?}", &access_token);
    */
    format!("")
}

#[get("/registerclienturls")]
pub(crate) async fn register_client_urls(data: web::Data<Pool>) -> impl Responder {
    let register_url_details = get_register_url_details(&data);
    /*
    tokio::spawn(async move {
        // Process each request concurrently.
        register_url(data, register_url_details).await;
    });
    */
    let consumer_key: String = get_settings_details(&data, String::from("consumerkeympesa"));
    let consumer_secret: String = get_settings_details(&data, String::from("consumersecretmpesa"));
    let auth_token_url: String = get_settings_details(&data, String::from("authtokenurlmpesa"));
    let mpesa_gateway: MpesaGateway =
        MpesaGateway::new(consumer_key, consumer_secret, auth_token_url);
    let xy = mpesa_gateway.get_register_url(register_url_details);
    let register_url_response_data = xy.await;
    println!(
        "register_url_response_data: {:?}",
        &register_url_response_data
    );

    format!("")
}

#[post("/processb2c")]
pub(crate) async fn process_b2c(data: web::Data<Pool>) -> impl Responder {
    let register_url_details = get_register_url_details(&data);
    let consumer_key: String = get_settings_details(&data, String::from("consumerkeympesa"));
    let consumer_secret: String = get_settings_details(&data, String::from("consumersecretmpesa"));
    let auth_token_url: String = get_settings_details(&data, String::from("authtokenurlmpesa"));

    let mobile_no = String::from("254712*****4");
    let amount_paid: u32 = 150;
    let command_id = TRANSACTION_COMMAND_ID.to_string();
    let _remarks = TRANSACTION_REMARKS.to_string();
    let _occassion = TRANSACTION_OCCASSION.to_string();

    let business_to_customer_data = get_business_to_customer_details(
        &data,
        mobile_no.to_string(),
        amount_paid,
        command_id.to_string(),
        _remarks.to_string(),
        _occassion.to_string(),
    );

    /*
    println!(
        "business_to_customer_data: {:?}",
        &business_to_customer_data
    );
    */

    let mpesa_gateway: MpesaGateway =
        MpesaGateway::new(consumer_key, consumer_secret, auth_token_url);
    let xy = mpesa_gateway.get_b2c(business_to_customer_data);
    let business_to_customer_response_data = xy.await;
    println!(
        "business_to_customer_response_data: {:?}",
        &business_to_customer_response_data
    );

    format!("")
}

#[post("/processc2bpayment")]
pub(crate) async fn process_c2b_payment(data: web::Data<Pool>) -> impl Responder {
    let consumer_key: String = get_settings_details(&data, String::from("consumerkeympesa"));
    let consumer_secret: String = get_settings_details(&data, String::from("consumersecretmpesa"));
    let auth_token_url: String = get_settings_details(&data, String::from("authtokenurlmpesa"));

    let api_url: String =
        String::from("https://sandbox.safaricom.co.ke/mpesa/stkpush/v1/processrequest");
    let business_short_code: String = String::from("174***");
    let _password: String = String::from("***");
    let time_stamp: String = Local::now().format("%Y%m%d%H%M%S").to_string(); //"YYYYMMDDHHmmss";
    let transaction_type: String = String::from("CustomerPayBillOnline");
    let _amount: u32 = 1;
    let party_a: u64 = 25470*****9;
    let party_b: u32 = 174***;
    let phone_number: u64 = 25472*****8;
    let call_back_url: String = String::from("https://mydomain.com/path");
    let account_reference: String = String::from("Company X LTD");
    let transaction_desc: String = String::from("Payment of X");

    let customer_to_business_details: CustomerToBusinessPaymentInputDetails =
        CustomerToBusinessPaymentInputDetails {
            api_url: api_url,
            business_short_code: business_short_code,
            _password: _password,
            time_stamp: time_stamp,
            transaction_type: transaction_type,
            _amount: _amount,
            party_a: party_a,
            party_b: party_b,
            phone_number: phone_number,
            call_back_url: call_back_url,
            account_reference: account_reference,
            transaction_desc: transaction_desc,
        };

    /*
    println!(
        "customer_to_business_details: {:?}",
        &customer_to_business_details
    );
    */
    let mpesa_gateway: MpesaGateway =
        MpesaGateway::new(consumer_key, consumer_secret, auth_token_url);
    let xy = mpesa_gateway.get_c2b_payment(customer_to_business_details);
    let customer_to_business_response_data = xy.await;
    println!(
        "customer_to_business_response_data: {:?}",
        &customer_to_business_response_data
    );

    format!("")
}

#[post("/b2c/timeout")]
pub(crate) async fn get_b2c_timeout(
    result_data: web::Json<B2CFailedData>,
    data: web::Data<Pool>,
) -> impl Responder {
    let result_type = &result_data.Result.ResultType;
    let result_code = &result_data.Result.ResultCode;
    let result_desc = &result_data.Result.ResultDesc;
    let originator_conversation_id = &result_data.Result.OriginatorConversationID;
    let conversation_id = &result_data.Result.ConversationID;
    let transaction_id = &result_data.Result.TransactionID;
    let reference_item = &result_data.Result.ReferenceData.ReferenceItem;
    let queue_timeout_url = &reference_item.Value;

    println!("result_desc: {:?}", &result_desc);
    println!(
        "originator_conversation_id: {:?}",
        &originator_conversation_id
    );
    /*
    create_b2c_timeout(
        &data,
        *result_type,
        *result_code,
        result_desc.to_string(),
        originator_conversation_id.to_string(),
        conversation_id.to_string(),
        transaction_id.to_string(),
        queue_timeout_url.to_string(),
    );
    */

    format!("")
}

#[post("/b2c/result")]
pub(crate) async fn get_b2c_result(
    result_data: web::Json<B2CResultData>,
    data: web::Data<Pool>,
) -> impl Responder {
    let result_type = &result_data.Result.ResultType;
    let result_code = &result_data.Result.ResultCode;
    let result_desc = &result_data.Result.ResultDesc;
    let originator_conversation_id = &result_data.Result.OriginatorConversationID;
    let conversation_id = &result_data.Result.ConversationID;
    let transaction_id = &result_data.Result.TransactionID;
    let result_parameters = &result_data.Result.ResultParameters;
    let mut transaction_amount: f32 = 0.0;
    let mut transaction_receipt = String::from("");
    let mut b2c_recipient_is_registered_customer = String::from("");
    let mut b2c_charges_paid_account_available_funds: f32 = 0.0;
    let mut receiver_party_public_name = String::from("");
    let mut transaction_completed_date_time = String::from("");
    let mut b2c_utility_account_available_funds: f32 = 0.0;
    let mut b2c_working_account_available_funds: f32 = 0.0;
    let reference_item = &result_data.Result.ReferenceData.ReferenceItem;
    let queue_timeout_url = &reference_item.Value;

    for result_parameter in result_parameters.ResultParameter.iter() {
        let _key = &result_parameter.Key;
        let _value = &result_parameter.Value;

        //TransactionAmount
        if _key
            .to_string()
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("TransactionAmount"))
        {
            transaction_amount = match _value {
                MixedTypeValue::StringValue(s) => 0.0,
                MixedTypeValue::IntegerValue(i) => *i as f32,
                MixedTypeValue::FloatValue(f) => *f,
                _ => 0.0,
            }
        }

        //TransactionReceipt
        if _key
            .to_string()
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("TransactionReceipt"))
        {
            transaction_receipt = match _value {
                MixedTypeValue::StringValue(s) => s.to_string(),
                _ => String::from(""),
            }
        }

        //B2CRecipientIsRegisteredCustomer
        if _key
            .to_string()
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("B2CRecipientIsRegisteredCustomer"))
        {
            b2c_recipient_is_registered_customer = match _value {
                MixedTypeValue::StringValue(s) => s.to_string(),
                _ => String::from(""),
            }
        }

        //B2CChargesPaidAccountAvailableFunds
        if _key
            .to_string()
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("B2CChargesPaidAccountAvailableFunds"))
        {
            b2c_charges_paid_account_available_funds = match _value {
                MixedTypeValue::StringValue(s) => 0.0,
                MixedTypeValue::IntegerValue(i) => *i as f32,
                MixedTypeValue::FloatValue(f) => *f,
                _ => 0.0,
            }
        }

        //ReceiverPartyPublicName
        if _key
            .to_string()
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("ReceiverPartyPublicName"))
        {
            receiver_party_public_name = match _value {
                MixedTypeValue::StringValue(s) => s.to_string(),
                _ => String::from(""),
            }
        }

        //TransactionCompletedDateTime
        if _key
            .to_string()
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("TransactionCompletedDateTime"))
        {
            transaction_completed_date_time = match _value {
                MixedTypeValue::StringValue(s) => s.to_string(),
                _ => String::from(""),
            }
        }

        //B2CUtilityAccountAvailableFunds
        if _key
            .to_string()
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("B2CUtilityAccountAvailableFunds"))
        {
            b2c_utility_account_available_funds = match _value {
                MixedTypeValue::StringValue(s) => 0.0,
                MixedTypeValue::IntegerValue(i) => *i as f32,
                MixedTypeValue::FloatValue(f) => *f,
                _ => 0.0,
            }
        }

        //B2CWorkingAccountAvailableFunds
        if _key
            .to_string()
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("B2CWorkingAccountAvailableFunds"))
        {
            b2c_working_account_available_funds = match _value {
                MixedTypeValue::StringValue(s) => 0.0,
                MixedTypeValue::IntegerValue(i) => *i as f32,
                MixedTypeValue::FloatValue(f) => *f,
                _ => 0.0,
            }
        }
    }

    if transaction_id.replace(" ", "").trim().len() > 0
        && transaction_receipt.replace(" ", "").trim().len() > 0
    {
        // Lets insert each entry
        /*
        create_b2c_result(
            &data,
            *result_type,
            *result_code,
            result_desc.to_string(),
            originator_conversation_id.to_string(),
            conversation_id.to_string(),
            transaction_id.to_string(),
            transaction_amount,
            transaction_receipt.to_string(),
            b2c_recipient_is_registered_customer.to_string(),
            b2c_charges_paid_account_available_funds,
            receiver_party_public_name.to_string(),
            transaction_completed_date_time.to_string(),
            b2c_utility_account_available_funds,
            b2c_working_account_available_funds,
            queue_timeout_url.to_string(),
        );
        */
        println!("result_desc: {:?}", &result_desc);
        println!(
            "originator_conversation_id: {:?}",
            &originator_conversation_id
        );
    }

    format!("")
}

#[post("/validationc2b")]
pub(crate) async fn validation_c2b(
    validation_data: web::Json<C2bData>,
    data: web::Data<Pool>,
) -> impl Responder {
    let k = String::from(""); //Default value.

    let transaction_type = &validation_data.TransactionType;
    let trans_id = &validation_data.TransID;
    let trans_time = &validation_data.TransTime;
    let trans_amount = &validation_data.TransAmount;
    let business_short_code = &validation_data.BusinessShortCode;
    let bill_ref_number = &validation_data.BillRefNumber;
    let invoice_number = &validation_data.InvoiceNumber.as_ref().unwrap_or(&k);
    let org_account_balance = &validation_data.OrgAccountBalance;
    let third_party_trans_id = &validation_data.ThirdPartyTransID;
    let _msisdn = &validation_data.MSISDN;
    let first_name = &validation_data.FirstName;
    let middle_name = &validation_data.MiddleName;
    let last_name = &validation_data.LastName;
    let bill_type = &C2B_BILL_TYPE;

    let response_status = create_incoming_c2b_mpesa_validation_requests(
        &data,
        transaction_type.to_string(),
        trans_id.to_string(),
        trans_time.to_string(),
        trans_amount.to_string(),
        business_short_code.to_string(),
        bill_ref_number.to_string(),
        invoice_number.to_string(),
        org_account_balance.to_string(),
        third_party_trans_id.to_string(),
        _msisdn.to_string(),
        first_name.to_string(),
        middle_name.to_string(),
        last_name.to_string(),
        bill_type.to_string(),
    );

    let response_data = ValidationResponseData {
        ResultCode: response_status.status_code.to_string(),
        ResultDesc: response_status.status_description,
    };

    web::Json(response_data)
}

#[post("/confirmationc2b")]
pub(crate) async fn confirmation_c2b(
    confirmation_data: web::Json<C2bData>,
    data: web::Data<Pool>,
) -> impl Responder {
    let k = String::from(""); //Default value.

    let transaction_type = &confirmation_data.TransactionType;
    let trans_id = &confirmation_data.TransID;
    let trans_time = &confirmation_data.TransTime;
    let trans_amount = &confirmation_data.TransAmount;
    let business_short_code = &confirmation_data.BusinessShortCode;
    let bill_ref_number = &confirmation_data.BillRefNumber;
    let invoice_number = &confirmation_data.InvoiceNumber.as_ref().unwrap_or(&k);
    let org_account_balance = &confirmation_data.OrgAccountBalance;
    let third_party_trans_id = &confirmation_data.ThirdPartyTransID;
    let _msisdn = &confirmation_data.MSISDN;
    let first_name = &confirmation_data.FirstName;
    let middle_name = &confirmation_data.MiddleName;
    let last_name = &confirmation_data.LastName;
    let bill_type = &C2B_BILL_TYPE;

    let response_status = create_incoming_c2b_mpesa_confirmation_requests(
        &data,
        transaction_type.to_string(),
        trans_id.to_string(),
        trans_time.to_string(),
        trans_amount.to_string(),
        business_short_code.to_string(),
        bill_ref_number.to_string(),
        invoice_number.to_string(),
        org_account_balance.to_string(),
        third_party_trans_id.to_string(),
        _msisdn.to_string(),
        first_name.to_string(),
        middle_name.to_string(),
        last_name.to_string(),
        bill_type.to_string(),
    );

    let response_data = ConfirmationResponseData {
        ResultCode: response_status.status_code,
        ResultDesc: response_status.status_description,
    };

    web::Json(response_data)
}

fn get_api_key(data: &web::Data<Pool>) -> String {
    let consumer_key_mpesa = get_settings_details(&data, String::from("consumerkeympesa"));
    let consumer_secret_mpesa = get_settings_details(&data, String::from("consumersecretmpesa"));
    let mut password: String = consumer_key_mpesa;
    let k = ":"; // Separator
    password.push_str(k);
    password.push_str(&consumer_secret_mpesa);
    let encodedpassword = general_purpose::STANDARD.encode(password);

    let mut api_key = String::from("Basic");
    let k = " "; // Separator
    api_key.push_str(k);
    api_key.push_str(&encodedpassword);

    api_key
}

fn get_register_url_details(data: &web::Data<Pool>) -> RegisterUrlInputDetails {
    let mut access_token = String::from("Bearer");
    let k = " "; // Separator
    let password: String = get_mpesa_access_token(&data);
    access_token.push_str(k);
    access_token.push_str(&password);
    let api_url = get_settings_details(&data, String::from("c2bregisterurlmpesa"));
    let short_code = get_settings_details(&data, String::from("c2bregisterbusinessshortcodempesa"));
    let response_type = get_settings_details(&data, String::from("c2bregisterresponsetypempesa"));
    let confirmation_url = get_settings_details(&data, String::from("confirmationc2burlmpesa"));
    let validation_url = get_settings_details(&data, String::from("validationc2burlmpesa"));

    let register_url_details = RegisterUrlInputDetails {
        //access_token: access_token,
        api_url: api_url,
        short_code: short_code,
        response_type: response_type,
        confirmation_url: confirmation_url,
        validation_url: validation_url,
    };

    register_url_details
}

fn get_business_to_customer_details(
    data: &web::Data<Pool>,
    my_party_b: String,
    my_amount: u32,
    my_command_id: String,
    my_remarks: String,
    my_occassion: String,
) -> BusinessToCustomerInputDetails {
    let my_api_url: String = get_settings_details(&data, String::from("b2cpaymentrequesturlmpesa"));
    let my_initiator_name: String =
        get_settings_details(&data, String::from("b2cinitiatornamempesa"));
    let my_security_credential: String =
        get_settings_details(&data, String::from("b2csecuritycredentialmpesa"));
    let my_party_a: String = get_settings_details(&data, String::from("b2cpartyampesa"));
    let my_queue_time_out_url: String =
        get_settings_details(&data, String::from("b2capplicationqueuetimeouturl"));
    let my_result_url: String =
        get_settings_details(&data, String::from("b2capplicationresulturl"));

    let my_party_a: u32 = match my_party_a.parse::<u32>() {
        Ok(a) => a,
        Err(e) => 0,
    };

    let business_to_customer_data = BusinessToCustomerInputDetails {
        api_url: my_api_url,
        initiator_name: my_initiator_name,
        security_credential: my_security_credential,
        command_id: my_command_id,
        amount: my_amount,
        party_a: my_party_a,
        party_b: my_party_b,
        _remarks: my_remarks,
        queue_time_out_url: my_queue_time_out_url,
        result_url: my_result_url,
        _occassion: my_occassion,
    };

    business_to_customer_data
}
