extern crate base64;
extern crate mpesa_rust_sdk;

use crate::persistence::{
    create_incoming_c2b_mpesa_confirmation_requests, create_incoming_c2b_mpesa_validation_requests,
    get_mpesa_access_token, get_settings_details,
};
use actix_web::{get, post, web, App, HttpRequest, HttpServer, Responder};
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use chrono::prelude::*;
use mpesa_rust_sdk::models::models::{
    B2CFailedData, B2CResultData, BusinessBuyGoodsErrorResponseData, BusinessBuyGoodsFailedData,
    BusinessBuyGoodsInputDetails, BusinessBuyGoodsResponseData, BusinessBuyGoodsResultData,
    BusinessPayBillErrorResponseData, BusinessPayBillFailedData, BusinessPayBillInputDetails,
    BusinessPayBillResponseData, BusinessPayBillResultData, BusinessToCustomerErrorResponseData,
    BusinessToCustomerInputDetails, BusinessToCustomerResponseData, C2bData,
    ConfirmationResponseData, CustomerToBusinessPaymentErrorResponseData,
    CustomerToBusinessPaymentInputDetails, CustomerToBusinessPaymentResponseData,
    CustomerToBusinessPaymentResultData, Item, ItemDetails, MixedTypeValue,
    RegisterUrlInputDetails, RegisterUrlResponseData, ValidationResponseData,
};
use mpesa_rust_sdk::MpesaGateway;
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

#[get("/registerclienturls")]
pub(crate) async fn register_client_urls(data: web::Data<Pool>) -> impl Responder {
    let _result = get_register_url_details(&data);

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

    format!("")
}

#[post("/processb2c")]
pub(crate) async fn process_b2c(data: web::Data<Pool>) -> impl Responder {
    let consumer_key: String = get_settings_details(&data, String::from("consumerkeympesa"));
    let consumer_secret: String = get_settings_details(&data, String::from("consumersecretmpesa"));
    let auth_token_url: String = get_settings_details(&data, String::from("authtokenurlmpesa"));

    let mobile_no = String::from("2547***");
    let amount_paid: u32 = 1500;
    let command_id = TRANSACTION_COMMAND_ID.to_string();
    let _remarks = TRANSACTION_REMARKS.to_string();
    let _occassion = TRANSACTION_OCCASSION.to_string();

    let _result = get_business_to_customer_details(
        &data,
        mobile_no.to_string(),
        amount_paid,
        command_id.to_string(),
        _remarks.to_string(),
        _occassion.to_string(),
    );

    if let Ok(business_to_customer_data) = _result {
        let _result = MpesaGateway::new(consumer_key, consumer_secret, auth_token_url);
        if let Ok(mpesa_gateway) = _result {
            // Initiate the request through the sdk
            let _output = mpesa_gateway.b2c(business_to_customer_data);

            let _result: std::result::Result<
                (
                    Option<BusinessToCustomerResponseData>,
                    Option<BusinessToCustomerErrorResponseData>,
                ),
                String,
            > = _output.await;

            match _result {
                Ok(business_to_customer_data) => {
                    // Lets unpack the tuple
                    let (
                        business_to_customer_response_data,
                        business_to_customer_error_response_data,
                    ) = business_to_customer_data;

                    // business_to_customer_response_data
                    if let Some(response_data) = business_to_customer_response_data {
                        println!("business_to_customer_response_data: {:?}", &response_data);
                    }

                    // business_to_customer_error_response_data
                    if let Some(response_data) = business_to_customer_error_response_data {
                        println!(
                            "business_to_customer_error_response_data: {:?}",
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

    format!("")
}

#[post("/processc2bpayment")]
pub(crate) async fn process_c2b_payment(data: web::Data<Pool>) -> impl Responder {
    let consumer_key: String = get_settings_details(&data, String::from("consumerkeympesa"));
    let consumer_secret: String = get_settings_details(&data, String::from("consumersecretmpesa"));
    let auth_token_url: String = get_settings_details(&data, String::from("authtokenurlmpesa"));
    let stk_push_url: String =
        String::from("https://sandbox.safaricom.co.ke/mpesa/stkpush/v1/processrequest");
    let business_short_code: String = String::from("17***");
    let pass_key: String =
        String::from("***");
    let time_stamp: String = Local::now().format("%Y%m%d%H%M%S").to_string(); //"YYYYMMDDHHmmss";
    let transaction_type: String = String::from("CustomerPayBillOnline");
    let _amount: u32 = 1;
    let party_a: u64 = 2547***;
    let party_b: u32 = 17***;
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

    format!("")
}

#[post("/processbusinesspaybill")]
pub(crate) async fn process_business_paybill(data: web::Data<Pool>) -> impl Responder {
    let consumer_key: String = get_settings_details(&data, String::from("consumerkeympesa"));
    let consumer_secret: String = get_settings_details(&data, String::from("consumersecretmpesa"));
    let auth_token_url: String = get_settings_details(&data, String::from("authtokenurlmpesa"));

    let b2b_payment_request_url: String =
        String::from("https://sandbox.safaricom.co.ke/mpesa/b2b/v1/paymentrequest");
    let _initiator: String = String::from("***");
    let security_credential: String = String::from("***");
    let command_id: String = String::from("BusinessPayBill");
    let sender_identifier_type: String = String::from("4");
    let reciever_identifier_type: String = String::from("4");
    let _amount: u32 = 145;
    let party_a: String = String::from("6***");
    let party_b: String = String::from("0***");
    let account_reference: String = String::from("3***");
    let _requester: String = String::from("2547***");
    let _remarks: String = String::from("ok");
    let queue_time_out_url: String = String::from("https://mydomain.com/b2b/queue/");
    let result_url: String = String::from("https://mydomain.com/b2b/result/");

    let _result = BusinessPayBillInputDetails::new(
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

    if let Ok(business_paybill_details) = _result {
        let _result = MpesaGateway::new(consumer_key, consumer_secret, auth_token_url);

        if let Ok(mpesa_gateway) = _result {
            let _output = mpesa_gateway.business_paybill(business_paybill_details);

            let _result: std::result::Result<
                (
                    Option<BusinessPayBillResponseData>,
                    Option<BusinessPayBillErrorResponseData>,
                ),
                String,
            > = _output.await;

            match _result {
                Ok(business_paybill_data) => {
                    // Lets unpack the tuple
                    let (business_paybill_response_data, business_paybill_error_response_data) =
                        business_paybill_data;

                    // business_paybill_response_data
                    if let Some(response_data) = business_paybill_response_data {
                        println!("business_paybill_response_data: {:?}", &response_data);
                    }

                    // business_paybill_error_response_data
                    if let Some(response_data) = business_paybill_error_response_data {
                        println!("business_paybill_error_response_data: {:?}", &response_data);
                    }
                }
                Err(e) => {
                    println!("Processing Error: {:?}", e)
                }
            }
        };
    };

    format!("")
}

#[post("/processbusinessbuygoods")]
pub(crate) async fn process_business_buy_goods(data: web::Data<Pool>) -> impl Responder {
    let consumer_key: String = get_settings_details(&data, String::from("consumerkeympesa"));
    let consumer_secret: String = get_settings_details(&data, String::from("consumersecretmpesa"));
    let auth_token_url: String = get_settings_details(&data, String::from("authtokenurlmpesa"));

    let b2b_payment_request_url: String =
        String::from("https://sandbox.safaricom.co.ke/mpesa/b2b/v1/paymentrequest");
    let _initiator: String = String::from("***");
    let security_credential: String = String::from("***");
    let command_id: String = String::from("BusinessBuyGoods");
    let sender_identifier_type: String = String::from("4");
    let reciever_identifier_type: String = String::from("4");
    let _amount: u32 = 145;
    let party_a: String = String::from("6***");
    let party_b: String = String::from("0***");
    let account_reference: String = String::from("3***");
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
    println!("queue_timeout_url: {:?}", &queue_timeout_url);

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
    let reference_item = &result_data.Result.ReferenceData.ReferenceItem;
    let queue_timeout_url = &reference_item.Value;

    let consumer_key: String = get_settings_details(&data, String::from("consumerkeympesa"));
    let consumer_secret: String = get_settings_details(&data, String::from("consumersecretmpesa"));
    let auth_token_url: String = get_settings_details(&data, String::from("authtokenurlmpesa"));

    let _result = MpesaGateway::new(consumer_key, consumer_secret, auth_token_url);
    if let Ok(mpesa_gateway) = _result {
        let b2c_result_parameters_output_details =
            mpesa_gateway.get_b2c_result_parameters_output_details(result_parameters);

        println!(
            "b2c_result_parameters_output_details: {:?}",
            &b2c_result_parameters_output_details
        );

        println!("result_desc: {:?}", &result_desc);

        println!(
            "originator_conversation_id: {:?}",
            &originator_conversation_id
        );
    } else if let Err(e) = _result {
        println!("Data Error: {:?}", e)
    } else {
        println!("Unexpected error occured during processing")
    };

    format!("")
}

#[post("/c2bpayment/result")]
pub(crate) async fn get_c2bpayment_result(
    result_data: web::Json<CustomerToBusinessPaymentResultData>,
    data: web::Data<Pool>,
) -> impl Responder {
    let merchant_request_id = &result_data.Body.stkCallback.MerchantRequestID;
    let checkout_request_id = &result_data.Body.stkCallback.CheckoutRequestID;
    let result_code = &result_data.Body.stkCallback.ResultCode;
    let result_desc = &result_data.Body.stkCallback.ResultDesc;
    /*
    let item_details_1 = ItemDetails {
        Name: String::from("Amount"),
        Value: MixedTypeValue::FloatValue(150.00),
    };
    let item_details_2 = ItemDetails {
        Name: String::from("MpesaReceiptNumber"),
        Value: MixedTypeValue::StringValue(String::from("NLJ7RT61SV")),
    };
    */
    let mut item_details = Vec::new();

    let my_item = Item { Item: item_details };

    let callback_meta_data = &result_data
        .Body
        .stkCallback
        .CallbackMetadata
        .as_ref()
        .unwrap_or(&my_item);
    let list_of_items = &callback_meta_data.Item;

    println!("merchant_request_id: {:?}", &merchant_request_id);
    println!("checkout_request_id: {:?}", &checkout_request_id);
    println!("result_code: {:?}", &result_code);
    println!("result_desc: {:?}", &result_desc);

    let consumer_key: String = get_settings_details(&data, String::from("consumerkeympesa"));
    let consumer_secret: String = get_settings_details(&data, String::from("consumersecretmpesa"));
    let auth_token_url: String = get_settings_details(&data, String::from("authtokenurlmpesa"));

    let _result = MpesaGateway::new(consumer_key, consumer_secret, auth_token_url);
    if let Ok(mpesa_gateway) = _result {
        let c2b_payment_result_parameters_output_details =
            mpesa_gateway.get_c2b_payment_result_parameters_output_details(list_of_items);

        println!(
            "c2b_payment_result_parameters_output_details: {:?}",
            &c2b_payment_result_parameters_output_details
        );
    } else if let Err(e) = _result {
        println!("Data Error: {:?}", e)
    } else {
        println!("Unexpected error occured during processing")
    };

    format!("")
}

#[post("/businesspaybill/result")]
pub(crate) async fn get_business_paybill_result(
    result_data: web::Json<BusinessPayBillResultData>,
    data: web::Data<Pool>,
) -> impl Responder {
    let result_type = &result_data.Result.ResultType;
    let result_code = &result_data.Result.ResultCode;
    let result_desc = &result_data.Result.ResultDesc;
    let originator_conversation_id = &result_data.Result.OriginatorConversationID;
    let conversation_id = &result_data.Result.ConversationID;
    let transaction_id = &result_data.Result.TransactionID;
    let result_parameters = &result_data.Result.ResultParameters;
    let reference_data = &result_data.Result.ReferenceData;

    let consumer_key: String = get_settings_details(&data, String::from("consumerkeympesa"));
    let consumer_secret: String = get_settings_details(&data, String::from("consumersecretmpesa"));
    let auth_token_url: String = get_settings_details(&data, String::from("authtokenurlmpesa"));

    let _result = MpesaGateway::new(consumer_key, consumer_secret, auth_token_url);
    if let Ok(mpesa_gateway) = _result {
        let business_paybill_result_parameters_output_details =
            mpesa_gateway.get_business_paybill_result_parameters_output_details(result_parameters);

        let business_paybill_Reference_item_output_details =
            mpesa_gateway.get_business_paybill_Reference_item_output_details(reference_data);

        println!(
            "business_paybill_result_parameters_output_details: {:?}",
            &business_paybill_result_parameters_output_details
        );

        println!(
            "business_paybill_Reference_item_output_details: {:?}",
            &business_paybill_Reference_item_output_details
        );
    } else if let Err(e) = _result {
        println!("Data Error: {:?}", e)
    } else {
        println!("Unexpected error occured during processing")
    };

    format!("")
}

#[post("/businessbuygoods/result")]
pub(crate) async fn get_business_buy_goods_result(
    result_data: web::Json<BusinessBuyGoodsResultData>,
    data: web::Data<Pool>,
) -> impl Responder {
    let result_type = &result_data.Result.ResultType;
    let result_code = &result_data.Result.ResultCode;
    let result_desc = &result_data.Result.ResultDesc;
    let originator_conversation_id = &result_data.Result.OriginatorConversationID;
    let conversation_id = &result_data.Result.ConversationID;
    let transaction_id = &result_data.Result.TransactionID;
    let result_parameters = &result_data.Result.ResultParameters;
    let reference_data = &result_data.Result.ReferenceData;

    let consumer_key: String = get_settings_details(&data, String::from("consumerkeympesa"));
    let consumer_secret: String = get_settings_details(&data, String::from("consumersecretmpesa"));
    let auth_token_url: String = get_settings_details(&data, String::from("authtokenurlmpesa"));

    let _result = MpesaGateway::new(consumer_key, consumer_secret, auth_token_url);
    if let Ok(mpesa_gateway) = _result {
        let business_buy_goods_result_parameters_output_details = mpesa_gateway
            .get_business_buy_goods_result_parameters_output_details(result_parameters);

        let business_buy_goods_reference_item_output_details =
            mpesa_gateway.get_business_buy_goods_reference_item_output_details(reference_data);

        println!(
            "business_buy_goods_result_parameters_output_details: {:?}",
            &business_buy_goods_result_parameters_output_details
        );

        println!(
            "business_buy_goods_reference_item_output_details: {:?}",
            &business_buy_goods_reference_item_output_details
        );
    } else if let Err(e) = _result {
        println!("Data Error: {:?}", e)
    } else {
        println!("Unexpected error occured during processing")
    };

    format!("")
}

#[post("/businesspaybill/timeout")]
pub(crate) async fn get_business_paybill_timeout(
    result_data: web::Json<BusinessPayBillFailedData>,
    data: web::Data<Pool>,
) -> impl Responder {
    let result_type = &result_data.Result.ResultType;
    let result_code = &result_data.Result.ResultCode;
    let result_desc = &result_data.Result.ResultDesc;
    let originator_conversation_id = &result_data.Result.OriginatorConversationID;
    let conversation_id = &result_data.Result.ConversationID;
    let transaction_id = &result_data.Result.TransactionID;
    let result_parameter = &result_data.Result.ResultParameters;
    let reference_data = &result_data.Result.ReferenceData.ReferenceItem;

    let consumer_key: String = get_settings_details(&data, String::from("consumerkeympesa"));
    let consumer_secret: String = get_settings_details(&data, String::from("consumersecretmpesa"));
    let auth_token_url: String = get_settings_details(&data, String::from("authtokenurlmpesa"));

    let _result = MpesaGateway::new(consumer_key, consumer_secret, auth_token_url);
    if let Ok(mpesa_gateway) = _result {
        let business_paybill_timeout_parameters_output_details = mpesa_gateway
            .get_business_paybill_timeout_parameters_output_details(
                result_parameter,
                reference_data,
            );

        println!(
            "business_paybill_timeout_parameters_output_details: {:?}",
            &business_paybill_timeout_parameters_output_details
        );
    } else if let Err(e) = _result {
        println!("Data Error: {:?}", e)
    } else {
        println!("Unexpected error occured during processing")
    };

    format!("")
}

#[post("/businessbuygoods/timeout")]
pub(crate) async fn get_business_buy_goods_timeout(
    result_data: web::Json<BusinessBuyGoodsFailedData>,
    data: web::Data<Pool>,
) -> impl Responder {
    let result_type = &result_data.Result.ResultType;
    let result_code = &result_data.Result.ResultCode;
    let result_desc = &result_data.Result.ResultDesc;
    let originator_conversation_id = &result_data.Result.OriginatorConversationID;
    let conversation_id = &result_data.Result.ConversationID;
    let transaction_id = &result_data.Result.TransactionID;
    let result_parameter = &result_data.Result.ResultParameters;
    let reference_data = &result_data.Result.ReferenceData.ReferenceItem;

    let consumer_key: String = get_settings_details(&data, String::from("consumerkeympesa"));
    let consumer_secret: String = get_settings_details(&data, String::from("consumersecretmpesa"));
    let auth_token_url: String = get_settings_details(&data, String::from("authtokenurlmpesa"));

    let _result = MpesaGateway::new(consumer_key, consumer_secret, auth_token_url);
    if let Ok(mpesa_gateway) = _result {
        let business_buy_goods_timeout_parameters_output_details = mpesa_gateway
            .get_business_buy_goods_timeout_parameters_output_details(
                result_parameter,
                reference_data,
            );

        println!(
            "business_buy_goods_timeout_parameters_output_details: {:?}",
            &business_buy_goods_timeout_parameters_output_details
        );
    } else if let Err(e) = _result {
        println!("Data Error: {:?}", e)
    } else {
        println!("Unexpected error occured during processing")
    };

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

fn get_register_url_details(data: &web::Data<Pool>) -> Result<RegisterUrlInputDetails, String> {
    let api_url = get_settings_details(&data, String::from("c2bregisterurlmpesa"));
    let short_code = get_settings_details(&data, String::from("c2bregisterbusinessshortcodempesa"));
    let response_type = get_settings_details(&data, String::from("c2bregisterresponsetypempesa"));
    let confirmation_url = get_settings_details(&data, String::from("confirmationc2burlmpesa"));
    let validation_url = get_settings_details(&data, String::from("validationc2burlmpesa"));
    /*
    let register_url_details = RegisterUrlInputDetails {
        //api_url: api_url,
        short_code: short_code,
        response_type: response_type,
        confirmation_url: confirmation_url,
        validation_url: validation_url,
    };
    */
    let _result = RegisterUrlInputDetails::new(
        api_url,
        short_code,
        response_type,
        confirmation_url,
        validation_url,
    );

    _result
}

fn get_business_to_customer_details(
    data: &web::Data<Pool>,
    my_party_b: String,
    my_amount: u32,
    my_command_id: String,
    my_remarks: String,
    my_occassion: String,
) -> Result<BusinessToCustomerInputDetails, String> {
    let my_api_url: String = get_settings_details(&data, String::from("b2cpaymentrequesturlmpesa"));
    let my_originator_conversation_id = Local::now().format("%Y%m%d%H%M%S%3f").to_string(); // test only
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
    /*
    let business_to_customer_data = BusinessToCustomerInputDetails {
        //api_url: my_api_url,
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
    */

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
