use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ValidationResponseData {
    pub ResultCode: String,
    pub ResultDesc: String,
}

#[derive(Serialize)]
pub struct ConfirmationResponseData {
    pub ResultCode: u8,
    pub ResultDesc: String,
}

#[derive(Serialize, Debug)]
pub struct RegisterUrlData {
    pub ShortCode: String,
    pub ResponseType: String,
    pub ConfirmationURL: String,
    pub ValidationURL: String,
}

#[derive(Serialize, Debug)]
pub struct BusinessToCustomerData {
    pub InitiatorName: String,
    pub SecurityCredential: String,
    pub CommandID: String,
    pub Amount: u32,
    pub PartyA: u32,
    pub PartyB: String,
    pub Remarks: String,
    pub QueueTimeOutURL: String,
    pub ResultURL: String,
    pub Occassion: String,
}

#[derive(Serialize, Debug)]
pub struct CustomerToBusinessPaymentData {
    pub BusinessShortCode: String,
    pub Password: String,
    pub Timestamp: String,
    pub TransactionType: String,
    pub Amount: u32,
    pub PartyA: u64,
    pub PartyB: u32,
    pub PhoneNumber: u64,
    pub CallBackURL: String,
    pub AccountReference: String,
    pub TransactionDesc: String,
}

#[derive(Serialize, Debug)]
pub struct BusinessPayBillData {
    pub Initiator: String,
    pub SecurityCredential: String,
    pub CommandID: String,
    pub SenderIdentifierType: String,
    pub RecieverIdentifierType: String,
    pub Amount: u32,
    pub PartyA: String,
    pub PartyB: String,
    pub AccountReference: String,
    pub Requester: String,
    pub Remarks: String,
    pub QueueTimeOutURL: String,
    pub ResultURL: String,
}

#[derive(Serialize, Debug)]
pub struct BusinessBuyGoodsData {
    pub Initiator: String,
    pub SecurityCredential: String,
    pub CommandID: String,
    pub SenderIdentifierType: String,
    pub RecieverIdentifierType: String,
    pub Amount: u32,
    pub PartyA: String,
    pub PartyB: String,
    pub AccountReference: String,
    pub Requester: String,
    pub Remarks: String,
    pub QueueTimeOutURL: String,
    pub ResultURL: String,
}

#[derive(Deserialize)]
pub struct C2bData {
    pub TransactionType: String,
    pub TransID: String,
    pub TransTime: String,
    pub TransAmount: String,
    pub BusinessShortCode: String,
    pub BillRefNumber: String,
    pub InvoiceNumber: Option<String>,
    pub OrgAccountBalance: String,
    pub ThirdPartyTransID: String,
    pub MSISDN: String,
    pub FirstName: String,
    pub MiddleName: String,
    pub LastName: String,
}

#[derive(Deserialize, Debug)]
pub struct AuthTokenResponseData {
    pub access_token: Option<String>,
    pub expires_in: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct RegisterUrlResponseData {
    pub OriginatorCoversationID: Option<String>,
    pub ConversationID: Option<String>,
    pub ResponseDescription: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct BusinessToCustomerResponseData {
    pub OriginatorConversationID: Option<String>,
    pub ConversationID: Option<String>,
    pub ResponseCode: Option<String>,
    pub ResponseDescription: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct BusinessToCustomerErrorResponseData {
    pub requestId: Option<String>,
    pub errorCode: Option<String>,
    pub errorMessage: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ReferenceItemDetails {
    pub Key: String,
    pub Value: String,
}

#[derive(Deserialize, Debug)]
pub struct ReferenceItem {
    pub ReferenceItem: ReferenceItemDetails,
}

#[derive(Deserialize, Debug)]
pub struct ResultParameterDetails {
    pub Key: String,
    pub Value: MixedTypeValue,
}

#[derive(Deserialize, Debug)]
pub struct ResultParameter {
    pub ResultParameter: Vec<ResultParameterDetails>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MixedTypeValue {
    StringValue(String),
    IntegerValue(i32),
    FloatValue(f32),
}

#[derive(Deserialize, Debug)]
pub struct B2CResultDetails {
    pub ResultType: u8,
    pub ResultCode: u32,
    pub ResultDesc: String,
    pub OriginatorConversationID: String,
    pub ConversationID: String,
    pub TransactionID: String,
    pub ResultParameters: ResultParameter,
    pub ReferenceData: ReferenceItem,
}

#[derive(Deserialize, Debug)]
pub struct B2CResultData {
    pub Result: B2CResultDetails,
}

#[derive(Deserialize, Debug)]
pub struct B2CFailedDetails {
    pub ResultType: u8,
    pub ResultCode: u32,
    pub ResultDesc: String,
    pub OriginatorConversationID: String,
    pub ConversationID: String,
    pub TransactionID: String,
    pub ReferenceData: ReferenceItem,
}

#[derive(Deserialize, Debug)]
pub struct B2CFailedData {
    pub Result: B2CFailedDetails,
}

#[derive(Deserialize, Debug)]
pub struct CustomerToBusinessPaymentResponseData {
    pub MerchantRequestID: Option<String>,
    pub CheckoutRequestID: Option<String>,
    pub ResponseCode: Option<String>,
    pub ResponseDescription: Option<String>,
    pub CustomerMessage: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CustomerToBusinessPaymentErrorResponseData {
    pub requestId: Option<String>,
    pub errorCode: Option<String>,
    pub errorMessage: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ItemDetails {
    pub Name: String,
    pub Value: MixedTypeValue,
}

#[derive(Deserialize, Debug)]
pub struct Item {
    pub Item: Vec<ItemDetails>,
}

#[derive(Deserialize, Debug)]
pub struct CustomerToBusinessPaymentResultDetails {
    pub MerchantRequestID: String,
    pub CheckoutRequestID: String,
    pub ResultCode: u32,
    pub ResultDesc: String,
    pub CallbackMetadata: Item,
}

#[derive(Deserialize, Debug)]
pub struct CustomerToBusinessPaymentStkCallBackData {
    pub stkCallback: CustomerToBusinessPaymentResultDetails,
}

#[derive(Deserialize, Debug)]
pub struct CustomerToBusinessPaymentResultData {
    pub Body: CustomerToBusinessPaymentStkCallBackData,
}

#[derive(Deserialize, Debug)]
pub struct BusinessPayBillResponseData {
    pub OriginatorConversationID: Option<String>,
    pub ConversationID: Option<String>,
    pub ResponseCode: Option<String>,
    pub ResponseDescription: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct BusinessPayBillErrorResponseData {
    pub requestId: Option<String>,
    pub errorCode: Option<String>,
    pub errorMessage: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct BusinessBuyGoodsResponseData {
    pub OriginatorConversationID: Option<String>,
    pub ConversationID: Option<String>,
    pub ResponseCode: Option<String>,
    pub ResponseDescription: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct BusinessBuyGoodsErrorResponseData {
    pub requestId: Option<String>,
    pub errorCode: Option<String>,
    pub errorMessage: Option<String>,
}

// BusinessPayBill

#[derive(Deserialize, Debug)]
pub struct BusinessPayBillReferenceItemDetails {
    pub Key: String,
    pub Value: MixedTypeValue,
}

#[derive(Deserialize, Debug)]
pub struct BusinessPayBillReferenceItem {
    pub ReferenceItem: Vec<BusinessPayBillReferenceItemDetails>,
}

#[derive(Deserialize, Debug)]
pub struct BusinessPayBillResultDetails {
    pub ResultType: String,
    pub ResultCode: String,
    pub ResultDesc: String,
    pub OriginatorConversationID: String,
    pub ConversationID: String,
    pub TransactionID: String,
    pub ResultParameters: ResultParameter,
    pub ReferenceData: BusinessPayBillReferenceItem,
}

#[derive(Deserialize, Debug)]
pub struct BusinessPayBillResultData {
    pub Result: BusinessPayBillResultDetails,
}

#[derive(Deserialize, Debug)]
pub struct BusinessPayBillFailedResultParameter {
    pub ResultParameter: ResultParameterDetails,
}

#[derive(Deserialize, Debug)]
pub struct BusinessPayBillFailedDetails {
    pub ResultType: u8,
    pub ResultCode: u32,
    pub ResultDesc: String,
    pub OriginatorConversationID: String,
    pub ConversationID: String,
    pub TransactionID: String,
    pub ResultParameters: BusinessPayBillFailedResultParameter,
    pub ReferenceData: ReferenceItem,
}

#[derive(Deserialize, Debug)]
pub struct BusinessPayBillFailedData {
    pub Result: BusinessPayBillFailedDetails,
}

// BusinessBuyGoods

#[derive(Deserialize, Debug)]
pub struct BusinessBuyGoodsReferenceItemDetails {
    pub Key: String,
    pub Value: MixedTypeValue,
}

#[derive(Deserialize, Debug)]
pub struct BusinessBuyGoodsReferenceItem {
    pub ReferenceItem: Vec<BusinessBuyGoodsReferenceItemDetails>,
}

#[derive(Deserialize, Debug)]
pub struct BusinessBuyGoodsResultDetails {
    pub ResultType: String,
    pub ResultCode: String,
    pub ResultDesc: String,
    pub OriginatorConversationID: String,
    pub ConversationID: String,
    pub TransactionID: String,
    pub ResultParameters: ResultParameter,
    pub ReferenceData: BusinessBuyGoodsReferenceItem,
}

#[derive(Deserialize, Debug)]
pub struct BusinessBuyGoodsResultData {
    pub Result: BusinessBuyGoodsResultDetails,
}

#[derive(Deserialize, Debug)]
pub struct BusinessBuyGoodsFailedResultParameter {
    pub ResultParameter: ResultParameterDetails,
}

#[derive(Deserialize, Debug)]
pub struct BusinessBuyGoodsFailedDetails {
    pub ResultType: u8,
    pub ResultCode: u32,
    pub ResultDesc: String,
    pub OriginatorConversationID: String,
    pub ConversationID: String,
    pub TransactionID: String,
    pub ResultParameters: BusinessBuyGoodsFailedResultParameter,
    pub ReferenceData: ReferenceItem,
}

#[derive(Deserialize, Debug)]
pub struct BusinessBuyGoodsFailedData {
    pub Result: BusinessBuyGoodsFailedDetails,
}

// This struct holds  Register Url processing data
pub struct RegisterUrlInputDetails {
    //pub access_token: String,
    //pub api_url: String,
    pub short_code: String,
    pub response_type: String,
    pub confirmation_url: String,
    pub validation_url: String,
}

// This struct holds  Business To Customer processing data
#[derive(Debug)]
pub struct BusinessToCustomerInputDetails {
    //pub api_url: String,
    pub initiator_name: String,
    pub security_credential: String,
    pub command_id: String,
    pub amount: u32,
    pub party_a: u32,
    pub party_b: String,
    pub _remarks: String,
    pub queue_time_out_url: String,
    pub result_url: String,
    pub _occassion: String,
}

#[derive(Debug)]
pub struct CustomerToBusinessPaymentInputDetails {
    //pub api_url: String,
    pub business_short_code: String,
    pub _password: String,
    pub time_stamp: String,
    pub transaction_type: String,
    pub _amount: u32,
    pub party_a: u64,
    pub party_b: u32,
    pub phone_number: u64,
    pub call_back_url: String,
    pub account_reference: String,
    pub transaction_desc: String,
}

#[derive(Debug)]
pub struct BusinessPayBillInputDetails {
    //pub api_url: String,
    pub _initiator: String,
    pub security_credential: String,
    pub command_id: String,
    pub sender_identifier_type: String,
    pub reciever_identifier_type: String,
    pub _amount: u32,
    pub party_a: String,
    pub party_b: String,
    pub account_reference: String,
    pub _requester: String,
    pub _remarks: String,
    pub queue_time_out_url: String,
    pub result_url: String,
}

#[derive(Debug)]
pub struct BusinessBuyGoodsInputDetails {
    //pub api_url: String,
    pub _initiator: String,
    pub security_credential: String,
    pub command_id: String,
    pub sender_identifier_type: String,
    pub reciever_identifier_type: String,
    pub _amount: u32,
    pub party_a: String,
    pub party_b: String,
    pub account_reference: String,
    pub _requester: String,
    pub _remarks: String,
    pub queue_time_out_url: String,
    pub result_url: String,
}
