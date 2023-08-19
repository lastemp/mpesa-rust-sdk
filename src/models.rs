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
    pub OriginatorConversationID: String,
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
    //pub CallbackMetadata: Item,
    pub CallbackMetadata: Option<Item>,
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
/*
pub struct RegisterUrlInputDetails {
    //pub access_token: String,
    //pub api_url: String,
    pub short_code: String,
    pub response_type: String,
    pub confirmation_url: String,
    pub validation_url: String,
}
*/
pub struct RegisterUrlInputDetails {
    api_url: String,
    short_code: String,
    response_type: String,
    confirmation_url: String,
    validation_url: String,
}

impl RegisterUrlInputDetails {
    pub fn new(
        api_url: String,
        short_code: String,
        response_type: String,
        confirmation_url: String,
        validation_url: String,
    ) -> Result<Self, String> {
        if api_url.is_empty() || api_url.replace(" ", "").trim().len() == 0 {
            return Err(String::from("api url is empty"));
        }

        if short_code.is_empty() || short_code.replace(" ", "").trim().len() == 0 {
            return Err(String::from("short code is empty"));
        }

        if response_type.is_empty() || response_type.replace(" ", "").trim().len() == 0 {
            return Err(String::from("response type is empty"));
        }

        if confirmation_url.is_empty() || confirmation_url.replace(" ", "").trim().len() == 0 {
            return Err(String::from("confirmation url is empty"));
        }

        if validation_url.is_empty() || validation_url.replace(" ", "").trim().len() == 0 {
            return Err(String::from("validation url is empty"));
        }

        Ok(Self {
            api_url,
            short_code,
            response_type,
            confirmation_url,
            validation_url,
        })
    }

    pub fn get_api_url(&self) -> String {
        let api_url = &self.api_url;
        api_url.to_string()
    }

    pub fn get_short_code(&self) -> String {
        let short_code = &self.short_code;
        short_code.to_string()
    }

    pub fn get_response_type(&self) -> String {
        let response_type = &self.response_type;
        response_type.to_string()
    }

    pub fn get_confirmation_url(&self) -> String {
        let confirmation_url = &self.confirmation_url;
        confirmation_url.to_string()
    }

    pub fn get_validation_url(&self) -> String {
        let validation_url = &self.validation_url;
        validation_url.to_string()
    }
}

// This struct holds  Business To Customer processing data
#[derive(Debug)]
pub struct BusinessToCustomerInputDetails {
    api_url: String,
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
}

impl BusinessToCustomerInputDetails {
    pub fn new(
        api_url: String,
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
    ) -> Result<Self, String> {
        if api_url.is_empty() || api_url.replace(" ", "").trim().len() == 0 {
            return Err(String::from("api url is empty"));
        }

        if originator_conversation_id.is_empty()
            || originator_conversation_id.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("originator conversation id is empty"));
        }

        if initiator_name.is_empty() || initiator_name.replace(" ", "").trim().len() == 0 {
            return Err(String::from("initiator name is empty"));
        }

        if security_credential.is_empty() || security_credential.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("security credential is empty"));
        }

        if command_id.is_empty() || command_id.replace(" ", "").trim().len() == 0 {
            return Err(String::from("command id is empty"));
        }

        // SalaryPayment, BusinessPayment, PromotionPayment
        if command_id
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("SalaryPayment"))
            || command_id
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("BusinessPayment"))
            || command_id
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("PromotionPayment"))
        {
            // command id is valid
        } else {
            return Err(String::from("command id has invalid value"));
        }

        if amount == 0 {
            return Err(String::from("amount has invalid value"));
        }

        if party_a == 0 {
            return Err(String::from("party a has invalid value"));
        }

        // party_a (5-6 digits) e.g. 123454
        if party_a.to_string().len() == 5 || party_a.to_string().len() == 6 {
        } else {
            return Err(String::from("party a has invalid value"));
        }

        if party_b.is_empty() || party_b.replace(" ", "").trim().len() == 0 {
            return Err(String::from("party b is empty"));
        }

        if _remarks.is_empty() || _remarks.replace(" ", "").trim().len() == 0 {
            return Err(String::from("remarks is empty"));
        }

        if queue_time_out_url.is_empty() || queue_time_out_url.replace(" ", "").trim().len() == 0 {
            return Err(String::from("queue_time_out url is empty"));
        }

        if result_url.is_empty() || result_url.replace(" ", "").trim().len() == 0 {
            return Err(String::from("result url is empty"));
        }

        // _occassion is optional parameter

        Ok(Self {
            api_url,
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
        })
    }

    pub fn get_api_url(&self) -> String {
        let api_url = &self.api_url;
        api_url.to_string()
    }

    pub fn get_originator_conversation_id(&self) -> String {
        let originator_conversation_id = &self.originator_conversation_id;
        originator_conversation_id.to_string()
    }

    pub fn get_initiator_name(&self) -> String {
        let initiator_name = &self.initiator_name;
        initiator_name.to_string()
    }

    pub fn get_security_credential(&self) -> String {
        let security_credential = &self.security_credential;
        security_credential.to_string()
    }

    pub fn get_command_id(&self) -> String {
        let command_id = &self.command_id;
        command_id.to_string()
    }

    pub fn get_amount(&self) -> u32 {
        let amount = &self.amount;
        *amount
    }

    pub fn get_party_a(&self) -> u32 {
        let party_a = &self.party_a;
        *party_a
    }

    pub fn get_party_b(&self) -> String {
        let party_b = &self.party_b;
        party_b.to_string()
    }

    pub fn get_remarks(&self) -> String {
        let _remarks = &self._remarks;
        _remarks.to_string()
    }

    pub fn get_queue_time_out_url(&self) -> String {
        let queue_time_out_url = &self.queue_time_out_url;
        queue_time_out_url.to_string()
    }

    pub fn get_result_url(&self) -> String {
        let result_url = &self.result_url;
        result_url.to_string()
    }

    pub fn get_occassion(&self) -> String {
        let _occassion = &self._occassion;
        _occassion.to_string()
    }
}

#[derive(Debug)]
pub struct CustomerToBusinessPaymentInputDetails {
    api_url: String,
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
}

impl CustomerToBusinessPaymentInputDetails {
    pub fn new(
        api_url: String,
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
    ) -> Result<Self, String> {
        if api_url.is_empty() || api_url.replace(" ", "").trim().len() == 0 {
            return Err(String::from("api url is empty"));
        }

        if business_short_code.is_empty() || business_short_code.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("business short code is empty"));
        }

        // business_short_code (5-6 digits) e.g. 123454
        if business_short_code.len() == 5 || business_short_code.len() == 6 {
        } else {
            return Err(String::from("party a has invalid value"));
        }

        if _password.is_empty() || _password.replace(" ", "").trim().len() == 0 {
            return Err(String::from("password is empty"));
        }

        if time_stamp.is_empty() || time_stamp.replace(" ", "").trim().len() == 0 {
            return Err(String::from("time stamp is empty"));
        }

        if transaction_type.is_empty() || transaction_type.replace(" ", "").trim().len() == 0 {
            return Err(String::from("transaction type is empty"));
        }

        // CustomerPayBillOnline, CustomerBuyGoodsOnline
        if transaction_type
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("CustomerPayBillOnline"))
            || transaction_type
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("CustomerBuyGoodsOnline"))
        {
            // transaction_type is valid
        } else {
            return Err(String::from("transaction type has invalid value"));
        }

        if _amount == 0 {
            return Err(String::from("amount has invalid value"));
        }

        if party_a == 0 {
            return Err(String::from("party a has invalid value"));
        }

        if party_b == 0 {
            return Err(String::from("party b has invalid value"));
        }

        // party_b (5-6 digits) e.g. 123454
        if party_b.to_string().len() == 5 || party_b.to_string().len() == 6 {
        } else {
            return Err(String::from("party b has invalid value"));
        }

        if phone_number == 0 {
            return Err(String::from("phone number has invalid value"));
        }

        if call_back_url.is_empty() || call_back_url.replace(" ", "").trim().len() == 0 {
            return Err(String::from("call_back url is empty"));
        }

        if account_reference.is_empty() || account_reference.replace(" ", "").trim().len() == 0 {
            return Err(String::from("account reference is empty"));
        }
        // account_reference has a max length of 12 characters
        else if account_reference.trim().len() > 0 && account_reference.trim().len() <= 12 {
            // account_reference is valid
        } else {
            return Err(String::from("account reference has invalid length"));
        }

        if transaction_desc.is_empty() || transaction_desc.replace(" ", "").trim().len() == 0 {
            return Err(String::from("transaction desc is empty"));
        }
        // transaction_desc has a max length of 13 characters
        else if transaction_desc.trim().len() > 0 && transaction_desc.trim().len() <= 13 {
        } else {
            return Err(String::from("transaction desc has invalid value/length"));
        }

        Ok(Self {
            api_url,
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
        })
    }

    pub fn get_api_url(&self) -> String {
        let api_url = &self.api_url;
        api_url.to_string()
    }

    pub fn get_business_short_code(&self) -> String {
        let business_short_code = &self.business_short_code;
        business_short_code.to_string()
    }

    pub fn get_password(&self) -> String {
        let _password = &self._password;
        _password.to_string()
    }

    pub fn get_time_stamp(&self) -> String {
        let time_stamp = &self.time_stamp;
        time_stamp.to_string()
    }

    pub fn get_transaction_type(&self) -> String {
        let transaction_type = &self.transaction_type;
        transaction_type.to_string()
    }

    pub fn get_amount(&self) -> u32 {
        let _amount = &self._amount;
        *_amount
    }

    pub fn get_party_a(&self) -> u64 {
        let party_a = &self.party_a;
        *party_a
    }

    pub fn get_party_b(&self) -> u32 {
        let party_b = &self.party_b;
        *party_b
    }

    pub fn get_phone_number(&self) -> u64 {
        let phone_number = &self.phone_number;
        *phone_number
    }

    pub fn get_call_back_url(&self) -> String {
        let call_back_url = &self.call_back_url;
        call_back_url.to_string()
    }

    pub fn get_account_reference(&self) -> String {
        let account_reference = &self.account_reference;
        account_reference.to_string()
    }

    pub fn get_transaction_desc(&self) -> String {
        let transaction_desc = &self.transaction_desc;
        transaction_desc.to_string()
    }
}

#[derive(Debug)]
pub struct BusinessPayBillInputDetails {
    api_url: String,
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
}

impl BusinessPayBillInputDetails {
    pub fn new(
        api_url: String,
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
    ) -> Result<Self, String> {
        if api_url.is_empty() || api_url.replace(" ", "").trim().len() == 0 {
            return Err(String::from("api url is empty"));
        }

        if _initiator.is_empty() || _initiator.replace(" ", "").trim().len() == 0 {
            return Err(String::from("initiator is empty"));
        }

        if security_credential.is_empty() || security_credential.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("security credential is empty"));
        }

        if command_id.is_empty() || command_id.replace(" ", "").trim().len() == 0 {
            return Err(String::from("command id is empty"));
        }

        // BusinessPayBill
        if command_id
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("BusinessPayBill"))
        {
            // command id is valid
        } else {
            return Err(String::from("command has invalid value"));
        }

        if sender_identifier_type.is_empty()
            || sender_identifier_type.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("sender identifier type is empty"));
        }

        if reciever_identifier_type.is_empty()
            || reciever_identifier_type.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("reciever identifier type is empty"));
        }

        if _amount == 0 {
            return Err(String::from("amount has invalid value"));
        }

        if party_a.is_empty() || party_a.replace(" ", "").trim().len() == 0 {
            return Err(String::from("party a is empty"));
        }

        // party_a (5-6 digits) e.g. 123454
        if party_a.to_string().len() == 5 || party_a.to_string().len() == 6 {
        } else {
            return Err(String::from("party a has invalid value"));
        }

        if party_b.is_empty() || party_b.replace(" ", "").trim().len() == 0 {
            return Err(String::from("party b is empty"));
        }

        // party_b (5-6 digits) e.g. 123454
        if party_b.to_string().len() == 5 || party_b.to_string().len() == 6 {
        } else {
            return Err(String::from("party b has invalid value"));
        }

        if account_reference.is_empty() || account_reference.replace(" ", "").trim().len() == 0 {
            return Err(String::from("account reference is empty"));
        }
        // account_reference has a max length of 13 characters
        else if account_reference.trim().len() > 0 && account_reference.trim().len() <= 13 {
            // account_reference is valid
        } else {
            return Err(String::from("account reference has invalid length"));
        }

        if _requester.is_empty() || _requester.replace(" ", "").trim().len() == 0 {
            return Err(String::from("_requester is empty"));
        }

        if _remarks.is_empty() || _remarks.replace(" ", "").trim().len() == 0 {
            return Err(String::from("remarks is empty"));
        }
        // _remarks has a max length of 100 characters
        else if _remarks.trim().len() > 0 && _remarks.trim().len() <= 100 {
            // _remarks is valid
        } else {
            return Err(String::from("remarks has invalid length"));
        }

        if queue_time_out_url.is_empty() || queue_time_out_url.replace(" ", "").trim().len() == 0 {
            return Err(String::from("queue_time_out url is empty"));
        }

        if result_url.is_empty() || result_url.replace(" ", "").trim().len() == 0 {
            return Err(String::from("result url is empty"));
        }

        Ok(Self {
            api_url,
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
        })
    }

    pub fn get_api_url(&self) -> String {
        let api_url = &self.api_url;
        api_url.to_string()
    }

    pub fn get_initiator(&self) -> String {
        let _initiator = &self._initiator;
        _initiator.to_string()
    }

    pub fn get_security_credential(&self) -> String {
        let security_credential = &self.security_credential;
        security_credential.to_string()
    }

    pub fn get_command_id(&self) -> String {
        let command_id = &self.command_id;
        command_id.to_string()
    }

    pub fn get_sender_identifier_type(&self) -> String {
        let sender_identifier_type = &self.sender_identifier_type;
        sender_identifier_type.to_string()
    }

    pub fn get_reciever_identifier_type(&self) -> String {
        let reciever_identifier_type = &self.reciever_identifier_type;
        reciever_identifier_type.to_string()
    }

    pub fn get_amount(&self) -> u32 {
        let _amount = &self._amount;
        *_amount
    }

    pub fn get_party_a(&self) -> String {
        let party_a = &self.party_a;
        party_a.to_string()
    }

    pub fn get_party_b(&self) -> String {
        let party_b = &self.party_b;
        party_b.to_string()
    }

    pub fn get_account_reference(&self) -> String {
        let account_reference = &self.account_reference;
        account_reference.to_string()
    }

    pub fn get_requester(&self) -> String {
        let _requester = &self._requester;
        _requester.to_string()
    }

    pub fn get_remarks(&self) -> String {
        let _remarks = &self._remarks;
        _remarks.to_string()
    }

    pub fn get_queue_time_out_url(&self) -> String {
        let queue_time_out_url = &self.queue_time_out_url;
        queue_time_out_url.to_string()
    }

    pub fn get_result_url(&self) -> String {
        let result_url = &self.result_url;
        result_url.to_string()
    }
}

#[derive(Debug)]
pub struct BusinessBuyGoodsInputDetails {
    api_url: String,
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
}

impl BusinessBuyGoodsInputDetails {
    pub fn new(
        api_url: String,
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
    ) -> Result<Self, String> {
        if api_url.is_empty() || api_url.replace(" ", "").trim().len() == 0 {
            return Err(String::from("api url is empty"));
        }

        if _initiator.is_empty() || _initiator.replace(" ", "").trim().len() == 0 {
            return Err(String::from("initiator is empty"));
        }

        if security_credential.is_empty() || security_credential.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("security credential is empty"));
        }

        if command_id.is_empty() || command_id.replace(" ", "").trim().len() == 0 {
            return Err(String::from("command id is empty"));
        }

        // BusinessBuyGoods
        if command_id
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("BusinessBuyGoods"))
        {
            // command id is valid
        } else {
            return Err(String::from("command has invalid value"));
        }

        if sender_identifier_type.is_empty()
            || sender_identifier_type.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("sender identifier type is empty"));
        }

        if reciever_identifier_type.is_empty()
            || reciever_identifier_type.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("reciever identifier type is empty"));
        }

        if _amount == 0 {
            return Err(String::from("amount has invalid value"));
        }

        if party_a.is_empty() || party_a.replace(" ", "").trim().len() == 0 {
            return Err(String::from("party a is empty"));
        }

        // party_a (5-6 digits) e.g. 123454
        if party_a.to_string().len() == 5 || party_a.to_string().len() == 6 {
        } else {
            return Err(String::from("party a has invalid value"));
        }

        if party_b.is_empty() || party_b.replace(" ", "").trim().len() == 0 {
            return Err(String::from("party b is empty"));
        }

        // party_b (5-6 digits) e.g. 123454
        if party_b.to_string().len() == 5 || party_b.to_string().len() == 6 {
        } else {
            return Err(String::from("party b has invalid value"));
        }

        if account_reference.is_empty() || account_reference.replace(" ", "").trim().len() == 0 {
            return Err(String::from("account reference is empty"));
        }
        // account_reference has a max length of 13 characters
        else if account_reference.trim().len() > 0 && account_reference.trim().len() <= 13 {
            // account_reference is valid
        } else {
            return Err(String::from("account reference has invalid length"));
        }

        if _requester.is_empty() || _requester.replace(" ", "").trim().len() == 0 {
            return Err(String::from("_requester is empty"));
        }

        if _remarks.is_empty() || _remarks.replace(" ", "").trim().len() == 0 {
            return Err(String::from("remarks is empty"));
        }
        // _remarks has a max length of 100 characters
        else if _remarks.trim().len() > 0 && _remarks.trim().len() <= 100 {
            // _remarks is valid
        } else {
            return Err(String::from("remarks has invalid length"));
        }

        if queue_time_out_url.is_empty() || queue_time_out_url.replace(" ", "").trim().len() == 0 {
            return Err(String::from("queue_time_out url is empty"));
        }

        if result_url.is_empty() || result_url.replace(" ", "").trim().len() == 0 {
            return Err(String::from("result url is empty"));
        }

        Ok(Self {
            api_url,
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
        })
    }

    pub fn get_api_url(&self) -> String {
        let api_url = &self.api_url;
        api_url.to_string()
    }

    pub fn get_initiator(&self) -> String {
        let _initiator = &self._initiator;
        _initiator.to_string()
    }

    pub fn get_security_credential(&self) -> String {
        let security_credential = &self.security_credential;
        security_credential.to_string()
    }

    pub fn get_command_id(&self) -> String {
        let command_id = &self.command_id;
        command_id.to_string()
    }

    pub fn get_sender_identifier_type(&self) -> String {
        let sender_identifier_type = &self.sender_identifier_type;
        sender_identifier_type.to_string()
    }

    pub fn get_reciever_identifier_type(&self) -> String {
        let reciever_identifier_type = &self.reciever_identifier_type;
        reciever_identifier_type.to_string()
    }

    pub fn get_amount(&self) -> u32 {
        let _amount = &self._amount;
        *_amount
    }

    pub fn get_party_a(&self) -> String {
        let party_a = &self.party_a;
        party_a.to_string()
    }

    pub fn get_party_b(&self) -> String {
        let party_b = &self.party_b;
        party_b.to_string()
    }

    pub fn get_account_reference(&self) -> String {
        let account_reference = &self.account_reference;
        account_reference.to_string()
    }

    pub fn get_requester(&self) -> String {
        let _requester = &self._requester;
        _requester.to_string()
    }

    pub fn get_remarks(&self) -> String {
        let _remarks = &self._remarks;
        _remarks.to_string()
    }

    pub fn get_queue_time_out_url(&self) -> String {
        let queue_time_out_url = &self.queue_time_out_url;
        queue_time_out_url.to_string()
    }

    pub fn get_result_url(&self) -> String {
        let result_url = &self.result_url;
        result_url.to_string()
    }
}

#[derive(Debug)]
pub struct B2CResultParametersOutputDetails {
    pub TransactionAmount: f32,
    pub TransactionReceipt: String,
    pub B2CRecipientIsRegisteredCustomer: String,
    pub B2CChargesPaidAccountAvailableFunds: f32,
    pub ReceiverPartyPublicName: String,
    pub TransactionCompletedDateTime: String,
    pub B2CUtilityAccountAvailableFunds: f32,
    pub B2CWorkingAccountAvailableFunds: f32,
}

#[derive(Debug)]
pub struct C2BPaymentResultParametersOutputDetails {
    pub Amount: f32,
    pub MpesaReceiptNumber: String,
    pub TransactionDate: String,
    pub PhoneNumber: String,
}

#[derive(Debug)]
pub struct BusinessPayBillResultParametersOutputDetails {
    pub DebitAccountBalance: String,
    pub Amount: String,
    pub DebitPartyAffectedAccountBalance: String,
    pub TransCompletedTime: String,
    pub DebitPartyCharges: String,
    pub ReceiverPartyPublicName: String,
    pub Currency: String,
    pub InitiatorAccountCurrentBalance: String,
}

#[derive(Debug)]
pub struct BusinessPayBillReferenceItemOutputDetails {
    pub BillReferenceNumber: String,
    pub QueueTimeoutURL: String,
}

#[derive(Debug)]
pub struct BusinessBuyGoodsResultParametersOutputDetails {
    pub DebitAccountBalance: String,
    pub Amount: String,
    pub DebitPartyAffectedAccountBalance: String,
    pub TransCompletedTime: String,
    pub DebitPartyCharges: String,
    pub ReceiverPartyPublicName: String,
    pub Currency: String,
    pub InitiatorAccountCurrentBalance: String,
}

#[derive(Debug)]
pub struct BusinessBuyGoodsReferenceItemOutputDetails {
    pub BillReferenceNumber: String,
    pub QueueTimeoutURL: String,
}

#[derive(Debug)]
pub struct BusinessPayBillTimeoutParametersOutputDetails {
    pub BOCompletedTime: String,
    pub QueueTimeoutURL: String,
}

#[derive(Debug)]
pub struct BusinessBuyGoodsTimeoutParametersOutputDetails {
    pub BOCompletedTime: String,
    pub QueueTimeoutURL: String,
}
