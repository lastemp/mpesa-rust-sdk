use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};

use actix_web::web;
use chrono::prelude::*;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::models::{
    AuthTokenResponseData, BusinessBuyGoodsData, BusinessBuyGoodsErrorResponseData,
    BusinessBuyGoodsInputDetails, BusinessBuyGoodsResponseData, BusinessPayBillData,
    BusinessPayBillErrorResponseData, BusinessPayBillInputDetails, BusinessPayBillResponseData,
    BusinessToCustomerData, BusinessToCustomerErrorResponseData, BusinessToCustomerInputDetails,
    BusinessToCustomerResponseData, CustomerToBusinessPaymentData,
    CustomerToBusinessPaymentErrorResponseData, CustomerToBusinessPaymentInputDetails,
    CustomerToBusinessPaymentResponseData, RegisterUrlData, RegisterUrlInputDetails,
    RegisterUrlResponseData,
};

const AUTHORISATION_BEARER: &str = "Bearer";

#[derive(Debug)]
pub struct MpesaGateway {
    consumer_key: String,
    consumer_secret: String,
    auth_token_url: String,
    register_url: String,
    b2c_payment_request_url: String,
    stk_push_url: String,
    b2b_payment_request_url: String,
}

impl MpesaGateway {
    pub fn new(
        consumer_key: String,
        consumer_secret: String,
        auth_token_url: String,
        register_url: String,
        b2c_payment_request_url: String,
        stk_push_url: String,
        b2b_payment_request_url: String,
    ) -> Self {
        Self {
            consumer_key: consumer_key,
            consumer_secret: consumer_secret,
            auth_token_url: auth_token_url,
            register_url: register_url,
            b2c_payment_request_url: b2c_payment_request_url,
            stk_push_url: stk_push_url,
            b2b_payment_request_url: b2b_payment_request_url,
        }
    }

    fn get_api_key(&self) -> String {
        let consumer_key = &self.consumer_key;
        let consumer_secret = &self.consumer_secret;
        let mut password: String = consumer_key.to_string();
        let k = ":"; // Separator
        password.push_str(k);
        password.push_str(&consumer_secret);
        let encodedpassword = general_purpose::STANDARD.encode(password);

        let mut api_key = String::from("Basic");
        let k = " "; // Separator
        api_key.push_str(k);
        api_key.push_str(&encodedpassword);

        api_key
    }

    pub async fn get_auth_token(&self) -> String {
        let api_key = self.get_api_key();
        //println!("api_key: {:?}", &api_key);
        let api_url = &self.auth_token_url;
        //println!("api_url: {:?}", &api_url);

        /*
        let xy = tokio::spawn(async move {
            // Process each request concurrently.
            let _access_token = generate_auth_token(api_key, api_url.to_string()).await;
            let access_token: String = match _access_token {
                Ok(a) => a,
                Err(e) => String::from(""),
            };
            return access_token;
        });

        let access_token: String = match xy.await {
            Ok(a) => a,
            Err(e) => String::from(""),
        };

        access_token
        */
        let xy = generate_auth_token(api_key, api_url.to_string()).await;

        let access_token: String = match xy {
            Ok(a) => {
                if !a.is_empty() {
                    let mut access_token = AUTHORISATION_BEARER.to_string();
                    let k = " "; // Separator
                    access_token.push_str(k);
                    access_token.push_str(&a);

                    access_token
                } else {
                    String::from("")
                }
            }
            Err(e) => String::from(""),
        };

        access_token
    }

    pub async fn get_register_url(
        &self,
        register_url_details: RegisterUrlInputDetails,
    ) -> RegisterUrlResponseData {
        let _output = self.get_auth_token();
        let access_token: String = _output.await;
        let api_url = &self.register_url;
        //println!("access_token: {:?}", &access_token);
        let register_url_response_data = RegisterUrlResponseData {
            OriginatorCoversationID: None,
            ConversationID: None,
            ResponseDescription: None,
        };

        if access_token.is_empty()
            || api_url.is_empty()
            || register_url_details.short_code.is_empty()
        {
            println!("access_token or api_url or register_url_details is empty");
            /*
            let b = RegisterUrlResponseData {
                OriginatorCoversationID: None,
                ConversationID: None,
                ResponseDescription: None,
            };
            return b;
            */
            return register_url_response_data;
        }

        let _result = register_url(register_url_details, access_token, api_url.to_string()).await;

        let register_url_response_data: RegisterUrlResponseData = match _result {
            Ok(a) => a,
            Err(e) => {
                /*
                let b = RegisterUrlResponseData {
                    OriginatorCoversationID: None,
                    ConversationID: None,
                    ResponseDescription: None,
                };

                b
                */
                register_url_response_data
            }
        };

        register_url_response_data
    }

    pub async fn get_b2c(
        &self,
        business_to_customer_details: BusinessToCustomerInputDetails,
    ) -> BusinessToCustomerResponseData {
        let _output = self.get_auth_token();
        let access_token: String = _output.await;
        let api_url = &self.b2c_payment_request_url;
        //println!("access_token: {:?}", &access_token);

        let business_to_customer_response_data = BusinessToCustomerResponseData {
            OriginatorConversationID: None,
            ConversationID: None,
            ResponseCode: None,
            ResponseDescription: None,
        };

        if access_token.is_empty()
            || api_url.is_empty()
            || business_to_customer_details.command_id.is_empty()
        {
            /*
            println!("access_token: {:?}", &access_token);
            println!(
                "business_to_customer_details: {:?}",
                &business_to_customer_details
            );
            let b = BusinessToCustomerResponseData {
                OriginatorConversationID: None,
                ConversationID: None,
                ResponseCode: None,
                ResponseDescription: None,
            };
            return b;
            */
            println!("access_token or api_url or business_to_customer_details is empty");
            return business_to_customer_response_data;
        }

        let _result = business_to_customer(
            business_to_customer_details,
            access_token,
            api_url.to_string(),
        )
        .await;

        let business_to_customer_response_data: BusinessToCustomerResponseData = match _result {
            Ok(a) => a,
            Err(e) => {
                /*
                let b = BusinessToCustomerResponseData {
                    OriginatorConversationID: None,
                    ConversationID: None,
                    ResponseCode: None,
                    ResponseDescription: None,
                };

                b
                */
                business_to_customer_response_data
            }
        };

        business_to_customer_response_data
    }

    pub async fn get_c2b_payment(
        &self,
        customer_to_business_details: CustomerToBusinessPaymentInputDetails,
    ) -> CustomerToBusinessPaymentResponseData {
        let _output = self.get_auth_token();
        let access_token: String = _output.await;
        let api_url = &self.stk_push_url;
        //println!("access_token: {:?}", &access_token);
        let customer_to_business_response_data = CustomerToBusinessPaymentResponseData {
            MerchantRequestID: None,
            CheckoutRequestID: None,
            ResponseCode: None,
            ResponseDescription: None,
            CustomerMessage: None,
        };

        if access_token.is_empty()
            || api_url.is_empty()
            || customer_to_business_details.business_short_code.is_empty()
        {
            /*
            println!("access_token: {:?}", &access_token);
            println!(
                "customer_to_business_details: {:?}",
                &customer_to_business_details
            );
            */
            println!("access_token or api_url or customer_to_business_details is empty");
            /*
            let b = CustomerToBusinessPaymentResponseData {
                MerchantRequestID: None,
                CheckoutRequestID: None,
                ResponseCode: None,
                ResponseDescription: None,
                CustomerMessage: None,
            };
            return b;
            */
            return customer_to_business_response_data;
        }

        let _result = customer_to_business_payment(
            customer_to_business_details,
            access_token,
            api_url.to_string(),
        )
        .await;

        let customer_to_business_response_data: CustomerToBusinessPaymentResponseData =
            match _result {
                Ok(a) => a,
                Err(e) => {
                    /*
                    let b = CustomerToBusinessPaymentResponseData {
                        MerchantRequestID: None,
                        CheckoutRequestID: None,
                        ResponseCode: None,
                        ResponseDescription: None,
                        CustomerMessage: None,
                    };

                    b
                    */
                    customer_to_business_response_data
                }
            };

        customer_to_business_response_data
    }

    pub async fn get_business_paybill(
        &self,
        business_paybill_details: BusinessPayBillInputDetails,
    ) -> BusinessPayBillResponseData {
        let _output = self.get_auth_token();
        let access_token: String = _output.await;
        let api_url = &self.b2b_payment_request_url;
        //println!("access_token: {:?}", &access_token);
        let business_paybill_response_data = BusinessPayBillResponseData {
            OriginatorConversationID: None,
            ConversationID: None,
            ResponseCode: None,
            ResponseDescription: None,
        };

        if access_token.is_empty()
            || api_url.is_empty()
            || business_paybill_details.command_id.is_empty()
        {
            /*
            println!("access_token: {:?}", &access_token);
            println!(
                "business_paybill_details: {:?}",
                &business_paybill_details
            );
            */
            println!("access_token or api_url or business_paybill_details is empty");
            /*
            let b = BusinessPayBillResponseData {
                OriginatorConversationID: None,
                ConversationID: None,
                ResponseCode: None,
                ResponseDescription: None,
            };
            return b;
            */
            return business_paybill_response_data;
        }

        let _result =
            business_paybill(business_paybill_details, access_token, api_url.to_string()).await;

        let business_paybill_response_data: BusinessPayBillResponseData = match _result {
            Ok(a) => a,
            Err(e) => {
                /*
                let b = BusinessPayBillResponseData {
                    OriginatorConversationID: None,
                    ConversationID: None,
                    ResponseCode: None,
                    ResponseDescription: None,
                };

                b
                */
                business_paybill_response_data
            }
        };

        business_paybill_response_data
    }

    pub async fn get_business_buy_goods(
        &self,
        business_buy_goods_details: BusinessBuyGoodsInputDetails,
    ) -> BusinessBuyGoodsResponseData {
        let _output = self.get_auth_token();
        let access_token: String = _output.await;
        let api_url = &self.b2b_payment_request_url;
        //println!("access_token: {:?}", &access_token);
        let business_buy_goods_response_data = BusinessBuyGoodsResponseData {
            OriginatorConversationID: None,
            ConversationID: None,
            ResponseCode: None,
            ResponseDescription: None,
        };

        if access_token.is_empty()
            || api_url.is_empty()
            || business_buy_goods_details.command_id.is_empty()
        {
            /*
            println!("access_token: {:?}", &access_token);
            println!(
                "business_buy_goods_details: {:?}",
                &business_buy_goods_details
            );
            */
            println!("access_token or api_url or business_buy_goods_details is empty");
            /*
            let b = BusinessBuyGoodsResponseData {
                OriginatorConversationID: None,
                ConversationID: None,
                ResponseCode: None,
                ResponseDescription: None,
            };
            return b;
            */
            return business_buy_goods_response_data;
        }

        let _result = business_buy_goods(
            business_buy_goods_details,
            access_token,
            api_url.to_string(),
        )
        .await;

        let business_buy_goods_response_data: BusinessBuyGoodsResponseData = match _result {
            Ok(a) => a,
            Err(e) => {
                /*
                let b = BusinessBuyGoodsResponseData {
                    OriginatorConversationID: None,
                    ConversationID: None,
                    ResponseCode: None,
                    ResponseDescription: None,
                };

                b
                */
                business_buy_goods_response_data
            }
        };

        business_buy_goods_response_data
    }
}

async fn generate_auth_token(
    api_key: String,
    api_url: String,
) -> std::result::Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut access_token = String::from("");
    // "%Y-%m-%d %H:%M:%S" i.e "yyyy-MM-dd HH:mm:ss"
    // "%Y-%m-%d %H:%M:%S%.3f" i.e "yyyy-MM-dd HH:mm:ss.SSS"
    let date_to_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    let res = client
        .get(api_url)
        .header(CONTENT_TYPE, "text/plain")
        .header(ACCEPT, "application/json")
        .header("Authorization", api_key)
        .send()
        //.await?; //The "?" after the await returns errors immediately and hence will not be captured on match clause below
        .await;

    match res {
        Err(e) => {
            println!("server not responding");
        }
        Ok(response) => {
            match response.status() {
                StatusCode::OK => {
                    let date_from_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                    let k = String::from(""); //Default value.

                    let my_output = response.json::<AuthTokenResponseData>().await?;

                    //let access_token = &my_output.access_token.as_ref().unwrap_or(&k);
                    let _access_token = &my_output.access_token.as_ref().unwrap_or(&k);
                    access_token = _access_token.to_string();
                    /*
                    let expires_in = &my_output.expires_in.as_ref().unwrap_or(&k);

                    let expires_in: u32 = match expires_in.parse::<u32>() {
                        Ok(a) => a,
                        Err(e) => 0,
                    };

                    create_mpesa_access_token(
                        &data,
                        access_token.to_string(),
                        expires_in,
                        date_to_mpesa,
                        date_from_mpesa,
                    );
                    */
                }
                s => println!("Received response status: {:?}", s),
            }
        }
    };

    Ok(access_token)
}

pub async fn register_url(
    register_url_details: RegisterUrlInputDetails,
    access_token: String,
    api_url: String,
) -> std::result::Result<RegisterUrlResponseData, reqwest::Error> {
    let short_code: String = register_url_details.short_code;
    let response_type: String = register_url_details.response_type;
    let confirmation_url: String = register_url_details.confirmation_url;
    let validation_url: String = register_url_details.validation_url;

    let register_url_data = RegisterUrlData {
        ShortCode: short_code,
        ResponseType: response_type,
        ConfirmationURL: confirmation_url,
        ValidationURL: validation_url,
    };
    let client = reqwest::Client::new();
    let mut register_url_response_data = RegisterUrlResponseData {
        OriginatorCoversationID: None,
        ConversationID: None,
        ResponseDescription: None,
    };
    //println!("register_url_data: {:?}", &register_url_data);
    //println!("access_token: {:?}", &access_token);
    // "%Y-%m-%d %H:%M:%S" i.e "yyyy-MM-dd HH:mm:ss"
    // "%Y-%m-%d %H:%M:%S%.3f" i.e "yyyy-MM-dd HH:mm:ss.SSS"
    let date_to_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    let res = client
        .post(api_url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header("Authorization", access_token)
        .json(&register_url_data)
        .send()
        //.await?; //The "?" after the await returns errors immediately and hence will not be captured on match clause below
        .await;

    match res {
        Err(e) => {
            println!("server not responding");
        }
        Ok(response) => {
            match response.status() {
                StatusCode::OK => {
                    let date_from_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                    let k = String::from(""); //Default value.

                    let my_output = response.json::<RegisterUrlResponseData>().await?;

                    let originator_coversation_id =
                        &my_output.OriginatorCoversationID.as_ref().unwrap_or(&k);
                    let conversation_id = &my_output.ConversationID.as_ref().unwrap_or(&k);
                    let response_description =
                        &my_output.ResponseDescription.as_ref().unwrap_or(&k);
                    register_url_response_data.OriginatorCoversationID =
                        Some(originator_coversation_id.to_string());
                    register_url_response_data.ConversationID = Some(conversation_id.to_string());
                    register_url_response_data.ResponseDescription =
                        Some(response_description.to_string());
                    /*
                    create_mpesa_register_url(
                        &data,
                        originator_coversation_id.to_string(),
                        conversation_id.to_string(),
                        response_description.to_string(),
                        register_url_data.ShortCode,
                        register_url_data.ConfirmationURL,
                        register_url_data.ValidationURL,
                        date_to_mpesa,
                        date_from_mpesa,
                    );
                    */
                }
                s => println!("Received response status: {:?}", s),
            }
        }
    };

    Ok(register_url_response_data)
}

pub async fn business_to_customer(
    business_to_customer_details: BusinessToCustomerInputDetails,
    access_token: String,
    api_url: String,
) -> std::result::Result<BusinessToCustomerResponseData, reqwest::Error> {
    //let api_url: String = business_to_customer_details.api_url;
    let initiator_name: String = business_to_customer_details.initiator_name;
    let security_credential: String = business_to_customer_details.security_credential;
    let command_id: String = business_to_customer_details.command_id;
    let amount: u32 = business_to_customer_details.amount;
    let party_a: u32 = business_to_customer_details.party_a;
    let party_b: String = business_to_customer_details.party_b;
    let _remarks: String = business_to_customer_details._remarks;
    let queue_time_out_url: String = business_to_customer_details.queue_time_out_url;
    let result_url: String = business_to_customer_details.result_url;
    let _occassion: String = business_to_customer_details._occassion;

    let business_to_customer_data = BusinessToCustomerData {
        InitiatorName: initiator_name,
        SecurityCredential: security_credential,
        CommandID: command_id,
        Amount: amount,
        PartyA: party_a,
        PartyB: party_b,
        Remarks: _remarks,
        QueueTimeOutURL: queue_time_out_url,
        ResultURL: result_url,
        Occassion: _occassion,
    };

    let mut business_to_customer_response_data = BusinessToCustomerResponseData {
        OriginatorConversationID: None,
        ConversationID: None,
        ResponseCode: None,
        ResponseDescription: None,
    };
    /*
    println!("access_token: {:?}", &access_token);
    println!(
        "business_to_customer_data: {:?}",
        &business_to_customer_data
    );
    */
    let client = reqwest::Client::new();
    // "%Y-%m-%d %H:%M:%S" i.e "yyyy-MM-dd HH:mm:ss"
    // "%Y-%m-%d %H:%M:%S%.3f" i.e "yyyy-MM-dd HH:mm:ss.SSS"
    let date_to_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    let res = client
        .post(api_url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header("Authorization", access_token)
        .json(&business_to_customer_data)
        .send()
        //.await?; //The "?" after the await returns errors immediately and hence will not be captured on match clause below
        .await;

    match res {
        Err(e) => {
            //println!("server not responding");
            println!("server not responding: {:?}", e.to_string());
        }
        Ok(response) => {
            match response.status() {
                StatusCode::OK => {
                    let date_from_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                    let k = String::from(""); //Default value.

                    let my_output = response.json::<BusinessToCustomerResponseData>().await?;

                    let originator_conversation_id =
                        &my_output.OriginatorConversationID.as_ref().unwrap_or(&k);
                    let conversation_id = &my_output.ConversationID.as_ref().unwrap_or(&k);
                    let response_code = &my_output.ResponseCode.as_ref().unwrap_or(&k);
                    let response_description =
                        &my_output.ResponseDescription.as_ref().unwrap_or(&k);
                    let request_id = String::from("");
                    let error_code = String::from("");
                    let error_message = String::from("");

                    //
                    business_to_customer_response_data.OriginatorConversationID =
                        Some(originator_conversation_id.to_string());
                    business_to_customer_response_data.ConversationID =
                        Some(conversation_id.to_string());
                    business_to_customer_response_data.ResponseCode =
                        Some(response_code.to_string());
                    business_to_customer_response_data.ResponseDescription =
                        Some(response_description.to_string());
                    /*
                    println!(
                        "business_to_customer_response_data: {:?}",
                        &business_to_customer_response_data
                    );
                    */

                    /*
                    create_b2c_acknowledgement(
                        &data,
                        originator_conversation_id.to_string(),
                        conversation_id.to_string(),
                        response_code.to_string(),
                        response_description.to_string(),
                        business_to_customer_data.CommandID,
                        business_to_customer_data.PartyA,
                        business_to_customer_data.PartyB,
                        business_to_customer_data.Amount,
                        request_id,
                        error_code,
                        error_message,
                        date_to_mpesa,
                        date_from_mpesa,
                    );
                    */
                }
                s => {
                    let date_from_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                    let k = String::from(""); //Default value.
                    let my_output = response
                        .json::<BusinessToCustomerErrorResponseData>()
                        .await?;
                    let request_id = &my_output.requestId.as_ref().unwrap_or(&k);
                    let error_code = &my_output.errorCode.as_ref().unwrap_or(&k);
                    let error_message = &my_output.errorMessage.as_ref().unwrap_or(&k);
                    let originator_conversation_id = String::from("");
                    let conversation_id = String::from("");
                    let response_code = String::from("");
                    let response_description = String::from("");

                    println!("my_output: {:?}", &my_output);
                    /*
                    create_b2c_acknowledgement(
                        &data,
                        originator_conversation_id.to_string(),
                        conversation_id.to_string(),
                        response_code.to_string(),
                        response_description.to_string(),
                        business_to_customer_data.CommandID,
                        business_to_customer_data.PartyA,
                        business_to_customer_data.PartyB,
                        business_to_customer_data.Amount,
                        request_id.to_string(),
                        error_code.to_string(),
                        error_message.to_string(),
                        date_to_mpesa,
                        date_from_mpesa,
                    );
                    */
                }
            }
        }
    };

    Ok(business_to_customer_response_data)
}

// network initiated push
pub async fn customer_to_business_payment(
    customer_to_business_payment_details: CustomerToBusinessPaymentInputDetails,
    access_token: String,
    api_url: String,
) -> std::result::Result<CustomerToBusinessPaymentResponseData, reqwest::Error> {
    //let api_url: String = customer_to_business_payment_details.api_url;
    let business_short_code: String = customer_to_business_payment_details.business_short_code;
    let _password: String = customer_to_business_payment_details._password;
    let time_stamp: String = customer_to_business_payment_details.time_stamp;
    let transaction_type: String = customer_to_business_payment_details.transaction_type;
    let _amount: u32 = customer_to_business_payment_details._amount;
    let party_a: u64 = customer_to_business_payment_details.party_a;
    let party_b: u32 = customer_to_business_payment_details.party_b;
    let phone_number: u64 = customer_to_business_payment_details.phone_number;
    let call_back_url: String = customer_to_business_payment_details.call_back_url;
    let account_reference: String = customer_to_business_payment_details.account_reference;
    let transaction_desc: String = customer_to_business_payment_details.transaction_desc;

    let customer_to_business_data = CustomerToBusinessPaymentData {
        BusinessShortCode: business_short_code,
        Password: _password,
        Timestamp: time_stamp,
        TransactionType: transaction_type,
        Amount: _amount,
        PartyA: party_a,
        PartyB: party_b,
        PhoneNumber: phone_number,
        CallBackURL: call_back_url,
        AccountReference: account_reference,
        TransactionDesc: transaction_desc,
    };

    let mut customer_to_business_response_data = CustomerToBusinessPaymentResponseData {
        MerchantRequestID: None,
        CheckoutRequestID: None,
        ResponseCode: None,
        ResponseDescription: None,
        CustomerMessage: None,
    };

    /*
    println!("access_token: {:?}", &access_token);
    println!("api_url: {:?}", &api_url);
    println!(
        "customer_to_business_data: {:?}",
        &customer_to_business_data
    );
    */
    let client = reqwest::Client::new();
    // "%Y-%m-%d %H:%M:%S" i.e "yyyy-MM-dd HH:mm:ss"
    // "%Y-%m-%d %H:%M:%S%.3f" i.e "yyyy-MM-dd HH:mm:ss.SSS"
    //let date_to_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    let res = client
        .post(api_url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header("Authorization", access_token)
        .json(&customer_to_business_data)
        .send()
        //.await?; //The "?" after the await returns errors immediately and hence will not be captured on match clause below
        .await;

    match res {
        Err(e) => {
            //println!("server not responding");
            println!("server not responding: {:?}", e.to_string());
        }
        Ok(response) => {
            match response.status() {
                StatusCode::OK => {
                    let date_from_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                    let k = String::from(""); //Default value.
                    let m: u32 = 0; //Default value.

                    let my_output = response
                        .json::<CustomerToBusinessPaymentResponseData>()
                        .await?;

                    let merchant_request_id = &my_output.MerchantRequestID.as_ref().unwrap_or(&k);
                    let checkout_request_id = &my_output.CheckoutRequestID.as_ref().unwrap_or(&k);
                    let response_code = &my_output.ResponseCode.as_ref().unwrap_or(&k);
                    let response_description =
                        &my_output.ResponseDescription.as_ref().unwrap_or(&k);
                    let customer_message = &my_output.CustomerMessage.as_ref().unwrap_or(&k);

                    //
                    customer_to_business_response_data.MerchantRequestID =
                        Some(merchant_request_id.to_string());
                    customer_to_business_response_data.CheckoutRequestID =
                        Some(checkout_request_id.to_string());
                    customer_to_business_response_data.ResponseCode =
                        Some(response_code.to_string());
                    customer_to_business_response_data.ResponseDescription =
                        Some(response_description.to_string());
                    customer_to_business_response_data.CustomerMessage =
                        Some(customer_message.to_string());
                    /*
                    println!(
                        "business_to_customer_response_data: {:?}",
                        &business_to_customer_response_data
                    );
                    */

                    /*
                    create_b2c_acknowledgement(
                        &data,
                        originator_conversation_id.to_string(),
                        conversation_id.to_string(),
                        response_code.to_string(),
                        response_description.to_string(),
                        business_to_customer_data.CommandID,
                        business_to_customer_data.PartyA,
                        business_to_customer_data.PartyB,
                        business_to_customer_data.Amount,
                        request_id,
                        error_code,
                        error_message,
                        date_to_mpesa,
                        date_from_mpesa,
                    );
                    */
                }
                s => {
                    let date_from_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                    let k = String::from(""); //Default value.
                    let m: u32 = 0; //Default value.
                    let my_output = response
                        .json::<CustomerToBusinessPaymentErrorResponseData>()
                        .await?;

                    let request_id = &my_output.requestId.as_ref().unwrap_or(&k);
                    let error_code = &my_output.errorCode.as_ref().unwrap_or(&k);
                    let error_message = &my_output.errorMessage.as_ref().unwrap_or(&k);

                    let merchant_request_id = request_id.to_string();
                    let checkout_request_id = String::from("");
                    let response_code = error_code.to_string();
                    let response_description = error_message.to_string();
                    let customer_message = String::from("");

                    customer_to_business_response_data.MerchantRequestID =
                        Some(merchant_request_id);
                    customer_to_business_response_data.CheckoutRequestID =
                        Some(checkout_request_id);
                    customer_to_business_response_data.ResponseCode = Some(response_code);
                    customer_to_business_response_data.ResponseDescription =
                        Some(response_description);
                    customer_to_business_response_data.CustomerMessage = Some(customer_message);

                    //println!("my_output: {:?}", &my_output);

                    /*
                    create_b2c_acknowledgement(
                        &data,
                        originator_conversation_id.to_string(),
                        conversation_id.to_string(),
                        response_code.to_string(),
                        response_description.to_string(),
                        business_to_customer_data.CommandID,
                        business_to_customer_data.PartyA,
                        business_to_customer_data.PartyB,
                        business_to_customer_data.Amount,
                        request_id.to_string(),
                        error_code.to_string(),
                        error_message.to_string(),
                        date_to_mpesa,
                        date_from_mpesa,
                    );
                    */
                }
            }
        }
    };

    Ok(customer_to_business_response_data)
}

pub async fn business_paybill(
    business_paybill_details: BusinessPayBillInputDetails,
    access_token: String,
    api_url: String,
) -> std::result::Result<BusinessPayBillResponseData, reqwest::Error> {
    //let api_url: String = business_paybill_details.api_url;
    let _initiator: String = business_paybill_details._initiator;
    let security_credential: String = business_paybill_details.security_credential;
    let command_id: String = business_paybill_details.command_id;
    let sender_identifier_type: String = business_paybill_details.sender_identifier_type;
    let reciever_identifier_type: String = business_paybill_details.reciever_identifier_type;
    let _amount: u32 = business_paybill_details._amount;
    let party_a: String = business_paybill_details.party_a;
    let party_b: String = business_paybill_details.party_b;
    let account_reference: String = business_paybill_details.account_reference;
    let _requester: String = business_paybill_details._requester;
    let _remarks: String = business_paybill_details._remarks;
    let queue_time_out_url: String = business_paybill_details.queue_time_out_url;
    let result_url: String = business_paybill_details.result_url;

    let business_paybill_data = BusinessPayBillData {
        Initiator: _initiator,
        SecurityCredential: security_credential,
        CommandID: command_id,
        SenderIdentifierType: sender_identifier_type,
        RecieverIdentifierType: reciever_identifier_type,
        Amount: _amount,
        PartyA: party_a,
        PartyB: party_b,
        AccountReference: account_reference,
        Requester: _requester,
        Remarks: _remarks,
        QueueTimeOutURL: queue_time_out_url,
        ResultURL: result_url,
    };

    let mut business_paybill_response_data = BusinessPayBillResponseData {
        OriginatorConversationID: None,
        ConversationID: None,
        ResponseCode: None,
        ResponseDescription: None,
    };

    /*
    println!("access_token: {:?}", &access_token);
    println!("api_url: {:?}", &api_url);
    println!(
        "customer_to_business_data: {:?}",
        &customer_to_business_data
    );
    */
    let client = reqwest::Client::new();
    // "%Y-%m-%d %H:%M:%S" i.e "yyyy-MM-dd HH:mm:ss"
    // "%Y-%m-%d %H:%M:%S%.3f" i.e "yyyy-MM-dd HH:mm:ss.SSS"
    //let date_to_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    let res = client
        .post(api_url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header("Authorization", access_token)
        .json(&business_paybill_data)
        .send()
        //.await?; //The "?" after the await returns errors immediately and hence will not be captured on match clause below
        .await;

    match res {
        Err(e) => {
            //println!("server not responding");
            println!("server not responding: {:?}", e.to_string());
        }
        Ok(response) => {
            match response.status() {
                StatusCode::OK => {
                    let date_from_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                    let k = String::from(""); //Default value.
                    let m: u32 = 0; //Default value.

                    let my_output = response.json::<BusinessPayBillResponseData>().await?;

                    let originator_conversation_id =
                        &my_output.OriginatorConversationID.as_ref().unwrap_or(&k);
                    let conversation_id = &my_output.ConversationID.as_ref().unwrap_or(&k);
                    let response_code = &my_output.ResponseCode.as_ref().unwrap_or(&k);
                    let response_description =
                        &my_output.ResponseDescription.as_ref().unwrap_or(&k);

                    //
                    business_paybill_response_data.OriginatorConversationID =
                        Some(originator_conversation_id.to_string());
                    business_paybill_response_data.ConversationID =
                        Some(conversation_id.to_string());
                    business_paybill_response_data.ResponseCode = Some(response_code.to_string());
                    business_paybill_response_data.ResponseDescription =
                        Some(response_description.to_string());
                    /*
                    println!(
                        "business_to_customer_response_data: {:?}",
                        &business_to_customer_response_data
                    );
                    */

                    /*
                    create_b2c_acknowledgement(
                        &data,
                        originator_conversation_id.to_string(),
                        conversation_id.to_string(),
                        response_code.to_string(),
                        response_description.to_string(),
                        business_to_customer_data.CommandID,
                        business_to_customer_data.PartyA,
                        business_to_customer_data.PartyB,
                        business_to_customer_data.Amount,
                        request_id,
                        error_code,
                        error_message,
                        date_to_mpesa,
                        date_from_mpesa,
                    );
                    */
                }
                s => {
                    let date_from_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                    let k = String::from(""); //Default value.
                    let m: u32 = 0; //Default value.
                    let my_output = response.json::<BusinessPayBillErrorResponseData>().await?;

                    let request_id = &my_output.requestId.as_ref().unwrap_or(&k);
                    let error_code = &my_output.errorCode.as_ref().unwrap_or(&k);
                    let error_message = &my_output.errorMessage.as_ref().unwrap_or(&k);

                    let originator_conversation_id = request_id.to_string();
                    let conversation_id = String::from("");
                    let response_code = error_code.to_string();
                    let response_description = error_message.to_string();

                    business_paybill_response_data.OriginatorConversationID =
                        Some(originator_conversation_id.to_string());
                    business_paybill_response_data.ConversationID =
                        Some(conversation_id.to_string());
                    business_paybill_response_data.ResponseCode = Some(response_code.to_string());
                    business_paybill_response_data.ResponseDescription =
                        Some(response_description.to_string());

                    //println!("my_output: {:?}", &my_output);

                    /*
                    create_b2c_acknowledgement(
                        &data,
                        originator_conversation_id.to_string(),
                        conversation_id.to_string(),
                        response_code.to_string(),
                        response_description.to_string(),
                        business_to_customer_data.CommandID,
                        business_to_customer_data.PartyA,
                        business_to_customer_data.PartyB,
                        business_to_customer_data.Amount,
                        request_id.to_string(),
                        error_code.to_string(),
                        error_message.to_string(),
                        date_to_mpesa,
                        date_from_mpesa,
                    );
                    */
                }
            }
        }
    };

    Ok(business_paybill_response_data)
}

async fn business_buy_goods(
    business_buy_goods_details: BusinessBuyGoodsInputDetails,
    access_token: String,
    api_url: String,
) -> std::result::Result<BusinessBuyGoodsResponseData, reqwest::Error> {
    //let api_url: String = business_buy_goods_details.api_url;
    let _initiator: String = business_buy_goods_details._initiator;
    let security_credential: String = business_buy_goods_details.security_credential;
    let command_id: String = business_buy_goods_details.command_id;
    let sender_identifier_type: String = business_buy_goods_details.sender_identifier_type;
    let reciever_identifier_type: String = business_buy_goods_details.reciever_identifier_type;
    let _amount: u32 = business_buy_goods_details._amount;
    let party_a: String = business_buy_goods_details.party_a;
    let party_b: String = business_buy_goods_details.party_b;
    let account_reference: String = business_buy_goods_details.account_reference;
    let _requester: String = business_buy_goods_details._requester;
    let _remarks: String = business_buy_goods_details._remarks;
    let queue_time_out_url: String = business_buy_goods_details.queue_time_out_url;
    let result_url: String = business_buy_goods_details.result_url;

    let business_buy_goods_data = BusinessBuyGoodsData {
        Initiator: _initiator,
        SecurityCredential: security_credential,
        CommandID: command_id,
        SenderIdentifierType: sender_identifier_type,
        RecieverIdentifierType: reciever_identifier_type,
        Amount: _amount,
        PartyA: party_a,
        PartyB: party_b,
        AccountReference: account_reference,
        Requester: _requester,
        Remarks: _remarks,
        QueueTimeOutURL: queue_time_out_url,
        ResultURL: result_url,
    };

    let mut business_buy_goods_response_data = BusinessBuyGoodsResponseData {
        OriginatorConversationID: None,
        ConversationID: None,
        ResponseCode: None,
        ResponseDescription: None,
    };

    /*
    println!("access_token: {:?}", &access_token);
    println!("api_url: {:?}", &api_url);
    println!(
        "customer_to_business_data: {:?}",
        &customer_to_business_data
    );
    */
    let client = reqwest::Client::new();
    // "%Y-%m-%d %H:%M:%S" i.e "yyyy-MM-dd HH:mm:ss"
    // "%Y-%m-%d %H:%M:%S%.3f" i.e "yyyy-MM-dd HH:mm:ss.SSS"
    //let date_to_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    let res = client
        .post(api_url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header("Authorization", access_token)
        .json(&business_buy_goods_data)
        .send()
        //.await?; //The "?" after the await returns errors immediately and hence will not be captured on match clause below
        .await;

    match res {
        Err(e) => {
            //println!("server not responding");
            println!("server not responding: {:?}", e.to_string());
        }
        Ok(response) => {
            match response.status() {
                StatusCode::OK => {
                    let date_from_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                    let k = String::from(""); //Default value.
                    let m: u32 = 0; //Default value.

                    let my_output = response.json::<BusinessBuyGoodsResponseData>().await?;

                    let originator_conversation_id =
                        &my_output.OriginatorConversationID.as_ref().unwrap_or(&k);
                    let conversation_id = &my_output.ConversationID.as_ref().unwrap_or(&k);
                    let response_code = &my_output.ResponseCode.as_ref().unwrap_or(&k);
                    let response_description =
                        &my_output.ResponseDescription.as_ref().unwrap_or(&k);

                    //
                    business_buy_goods_response_data.OriginatorConversationID =
                        Some(originator_conversation_id.to_string());
                    business_buy_goods_response_data.ConversationID =
                        Some(conversation_id.to_string());
                    business_buy_goods_response_data.ResponseCode = Some(response_code.to_string());
                    business_buy_goods_response_data.ResponseDescription =
                        Some(response_description.to_string());
                    /*
                    println!(
                        "business_to_customer_response_data: {:?}",
                        &business_to_customer_response_data
                    );
                    */

                    /*
                    create_b2c_acknowledgement(
                        &data,
                        originator_conversation_id.to_string(),
                        conversation_id.to_string(),
                        response_code.to_string(),
                        response_description.to_string(),
                        business_to_customer_data.CommandID,
                        business_to_customer_data.PartyA,
                        business_to_customer_data.PartyB,
                        business_to_customer_data.Amount,
                        request_id,
                        error_code,
                        error_message,
                        date_to_mpesa,
                        date_from_mpesa,
                    );
                    */
                }
                s => {
                    let date_from_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                    let k = String::from(""); //Default value.
                    let m: u32 = 0; //Default value.
                    let my_output = response.json::<BusinessBuyGoodsErrorResponseData>().await?;

                    let request_id = &my_output.requestId.as_ref().unwrap_or(&k);
                    let error_code = &my_output.errorCode.as_ref().unwrap_or(&k);
                    let error_message = &my_output.errorMessage.as_ref().unwrap_or(&k);

                    let originator_conversation_id = request_id.to_string();
                    let conversation_id = String::from("");
                    let response_code = error_code.to_string();
                    let response_description = error_message.to_string();

                    business_buy_goods_response_data.OriginatorConversationID =
                        Some(originator_conversation_id.to_string());
                    business_buy_goods_response_data.ConversationID =
                        Some(conversation_id.to_string());
                    business_buy_goods_response_data.ResponseCode = Some(response_code.to_string());
                    business_buy_goods_response_data.ResponseDescription =
                        Some(response_description.to_string());

                    //println!("my_output: {:?}", &my_output);

                    /*
                    create_b2c_acknowledgement(
                        &data,
                        originator_conversation_id.to_string(),
                        conversation_id.to_string(),
                        response_code.to_string(),
                        response_description.to_string(),
                        business_to_customer_data.CommandID,
                        business_to_customer_data.PartyA,
                        business_to_customer_data.PartyB,
                        business_to_customer_data.Amount,
                        request_id.to_string(),
                        error_code.to_string(),
                        error_message.to_string(),
                        date_to_mpesa,
                        date_from_mpesa,
                    );
                    */
                }
            }
        }
    };

    Ok(business_buy_goods_response_data)
}
