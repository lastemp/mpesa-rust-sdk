use reqwest::header::HeaderMap;
use reqwest::header::{ACCEPT, CONTENT_TYPE};

use crate::models::models::{
    BusinessBuyGoodsData, BusinessBuyGoodsErrorResponseData, BusinessBuyGoodsResponseData,
    BusinessPayBillData, BusinessPayBillErrorResponseData, BusinessPayBillResponseData,
    BusinessToCustomerData, BusinessToCustomerErrorResponseData, BusinessToCustomerResponseData,
    CustomerToBusinessPaymentData, CustomerToBusinessPaymentErrorResponseData,
    CustomerToBusinessPaymentResponseData, RegisterUrlData, RegisterUrlResponseData,
};

pub fn build_business_to_customer_response_data(
    originator_conversation_id: Option<String>,
    conversation_id: Option<String>,
    response_code: Option<String>,
    response_description: Option<String>,
) -> BusinessToCustomerResponseData {
    BusinessToCustomerResponseData {
        OriginatorConversationID: originator_conversation_id,
        ConversationID: conversation_id,
        ResponseCode: response_code,
        ResponseDescription: response_description,
    }
}

pub fn build_business_to_customer_error_response_data(
    request_id: Option<String>,
    error_code: Option<String>,
    error_message: Option<String>,
) -> BusinessToCustomerErrorResponseData {
    BusinessToCustomerErrorResponseData {
        requestId: request_id,
        errorCode: error_code,
        errorMessage: error_message,
    }
}

pub fn build_customer_to_business_payment_response_data(
    merchant_request_id: Option<String>,
    checkout_request_id: Option<String>,
    response_code: Option<String>,
    response_description: Option<String>,
    customer_message: Option<String>,
) -> CustomerToBusinessPaymentResponseData {
    CustomerToBusinessPaymentResponseData {
        MerchantRequestID: merchant_request_id,
        CheckoutRequestID: checkout_request_id,
        ResponseCode: response_code,
        ResponseDescription: response_description,
        CustomerMessage: customer_message,
    }
}

pub fn build_customer_to_business_payment_error_response_data(
    request_id: Option<String>,
    error_code: Option<String>,
    error_message: Option<String>,
) -> CustomerToBusinessPaymentErrorResponseData {
    CustomerToBusinessPaymentErrorResponseData {
        requestId: request_id,
        errorCode: error_code,
        errorMessage: error_message,
    }
}

pub fn build_business_paybill_response_data(
    originator_conversation_id: Option<String>,
    conversation_id: Option<String>,
    response_code: Option<String>,
    response_description: Option<String>,
) -> BusinessPayBillResponseData {
    BusinessPayBillResponseData {
        OriginatorConversationID: originator_conversation_id,
        ConversationID: conversation_id,
        ResponseCode: response_code,
        ResponseDescription: response_description,
    }
}

pub fn build_business_paybill_error_response_data(
    request_id: Option<String>,
    error_code: Option<String>,
    error_message: Option<String>,
) -> BusinessPayBillErrorResponseData {
    BusinessPayBillErrorResponseData {
        requestId: request_id,
        errorCode: error_code,
        errorMessage: error_message,
    }
}

pub fn build_business_buy_goods_response_data(
    originator_conversation_id: Option<String>,
    conversation_id: Option<String>,
    response_code: Option<String>,
    response_description: Option<String>,
) -> BusinessBuyGoodsResponseData {
    BusinessBuyGoodsResponseData {
        OriginatorConversationID: originator_conversation_id,
        ConversationID: conversation_id,
        ResponseCode: response_code,
        ResponseDescription: response_description,
    }
}

pub fn build_business_buy_goods_error_response_data(
    request_id: Option<String>,
    error_code: Option<String>,
    error_message: Option<String>,
) -> BusinessBuyGoodsErrorResponseData {
    BusinessBuyGoodsErrorResponseData {
        requestId: request_id,
        errorCode: error_code,
        errorMessage: error_message,
    }
}

pub fn build_register_url_data(
    short_code: String,
    response_type: String,
    confirmation_url: String,
    validation_url: String,
) -> RegisterUrlData {
    RegisterUrlData {
        ShortCode: short_code,
        ResponseType: response_type,
        ConfirmationURL: confirmation_url,
        ValidationURL: validation_url,
    }
}

pub fn build_register_url_response_data(
    originator_conversation_id: Option<String>,
    conversation_id: Option<String>,
    response_description: Option<String>,
) -> RegisterUrlResponseData {
    RegisterUrlResponseData {
        OriginatorCoversationID: originator_conversation_id,
        ConversationID: conversation_id,
        ResponseDescription: response_description,
    }
}

pub fn build_business_to_customer_data(
    originator_conversation_id: String,
    initiator_name: String,
    security_credential: String,
    command_id: String,
    amount: u32,
    party_a: u32,
    party_b: String,
    _remarks: String,
    queue_time_out_url: String,
    result_url: String,
    _occassion: String,
) -> BusinessToCustomerData {
    BusinessToCustomerData {
        OriginatorConversationID: originator_conversation_id,
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
    }
}

pub fn build_customer_to_business_data(
    business_short_code: String,
    _password: String,
    time_stamp: String,
    transaction_type: String,
    _amount: u32,
    party_a: u64,
    party_b: u32,
    phone_number: u64,
    call_back_url: String,
    account_reference: String,
    transaction_desc: String,
) -> CustomerToBusinessPaymentData {
    CustomerToBusinessPaymentData {
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
    }
}

pub fn build_business_paybill_data(
    _initiator: String,
    security_credential: String,
    command_id: String,
    sender_identifier_type: String,
    reciever_identifier_type: String,
    _amount: u32,
    party_a: String,
    party_b: String,
    account_reference: String,
    _requester: String,
    _remarks: String,
    queue_time_out_url: String,
    result_url: String,
) -> BusinessPayBillData {
    BusinessPayBillData {
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
    }
}

pub fn build_business_buy_goods_data(
    _initiator: String,
    security_credential: String,
    command_id: String,
    sender_identifier_type: String,
    reciever_identifier_type: String,
    _amount: u32,
    party_a: String,
    party_b: String,
    account_reference: String,
    _requester: String,
    _remarks: String,
    queue_time_out_url: String,
    result_url: String,
) -> BusinessBuyGoodsData {
    BusinessBuyGoodsData {
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
    }
}

pub fn build_headers(access_token: String) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert("Authorization", access_token.parse().unwrap());

    headers
}

pub fn build_headers_generate_auth_token(api_key: String) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(CONTENT_TYPE, "text/plain".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert("Authorization", api_key.parse().unwrap());

    headers
}
