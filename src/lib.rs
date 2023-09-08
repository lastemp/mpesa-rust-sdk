pub mod models {
    pub mod models;
}
mod util {
    pub mod util;
}
mod authorization {
    pub mod generate_auth_token;
}
mod customer_to_business {
    pub mod customer_to_business_payment;
    pub mod register_url;
}
mod business_to_customer {
    pub mod business_to_customer;
}
mod business_paybill {
    pub mod business_paybill;
}
mod business_buy_goods {
    pub mod business_buy_goods;
}
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use models::models::{
    B2CResultParametersOutputDetails, BusinessBuyGoodsErrorResponseData,
    BusinessBuyGoodsFailedResultParameter, BusinessBuyGoodsInputDetails,
    BusinessBuyGoodsReferenceItem, BusinessBuyGoodsReferenceItemOutputDetails,
    BusinessBuyGoodsResponseData, BusinessBuyGoodsResultParametersOutputDetails,
    BusinessBuyGoodsTimeoutParametersOutputDetails, BusinessPayBillErrorResponseData,
    BusinessPayBillFailedResultParameter, BusinessPayBillInputDetails,
    BusinessPayBillReferenceItem, BusinessPayBillReferenceItemOutputDetails,
    BusinessPayBillResponseData, BusinessPayBillResultParametersOutputDetails,
    BusinessPayBillTimeoutParametersOutputDetails, BusinessToCustomerErrorResponseData,
    BusinessToCustomerInputDetails, BusinessToCustomerResponseData,
    C2BPaymentResultParametersOutputDetails, CustomerToBusinessPaymentErrorResponseData,
    CustomerToBusinessPaymentInputDetails, CustomerToBusinessPaymentResponseData, ItemDetails,
    MixedTypeValue, ReferenceItemDetails, RegisterUrlInputDetails, RegisterUrlResponseData,
    ResultParameter,
};

const AUTHORISATION_BEARER: &str = "Bearer";

#[derive(Debug)]
pub struct MpesaGateway {
    consumer_key: String,
    consumer_secret: String,
    auth_token_url: String,
}

impl MpesaGateway {
    pub fn new(
        consumer_key: String,
        consumer_secret: String,
        auth_token_url: String,
    ) -> Result<Self, String> {
        if consumer_key.is_empty() || consumer_key.replace(" ", "").trim().len() == 0 {
            return Err(String::from("consumer key is empty"));
        }

        if consumer_secret.is_empty() || consumer_secret.replace(" ", "").trim().len() == 0 {
            return Err(String::from("consumer secret is empty"));
        }

        if auth_token_url.is_empty() || auth_token_url.replace(" ", "").trim().len() == 0 {
            return Err(String::from("auth_token url is empty"));
        }

        Ok(Self {
            consumer_key,
            consumer_secret,
            auth_token_url,
        })
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

    pub fn get_b2c_result_parameters_output_details(
        &self,
        result_parameters: &ResultParameter,
    ) -> B2CResultParametersOutputDetails {
        let mut transaction_amount: f32 = 0.0;
        let mut transaction_receipt = String::from("");
        let mut b2c_recipient_is_registered_customer = String::from("");
        let mut b2c_charges_paid_account_available_funds: f32 = 0.0;
        let mut receiver_party_public_name = String::from("");
        let mut transaction_completed_date_time = String::from("");
        let mut b2c_utility_account_available_funds: f32 = 0.0;
        let mut b2c_working_account_available_funds: f32 = 0.0;

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

        let b2c_result_parameters_output_details = B2CResultParametersOutputDetails {
            TransactionAmount: transaction_amount,
            TransactionReceipt: transaction_receipt,
            B2CRecipientIsRegisteredCustomer: b2c_recipient_is_registered_customer,
            B2CChargesPaidAccountAvailableFunds: b2c_charges_paid_account_available_funds,
            ReceiverPartyPublicName: receiver_party_public_name,
            TransactionCompletedDateTime: transaction_completed_date_time,
            B2CUtilityAccountAvailableFunds: b2c_utility_account_available_funds,
            B2CWorkingAccountAvailableFunds: b2c_working_account_available_funds,
        };

        b2c_result_parameters_output_details
    }

    pub fn get_c2b_payment_result_parameters_output_details(
        &self,
        list_of_items: &Vec<ItemDetails>,
    ) -> Option<C2BPaymentResultParametersOutputDetails> {
        let mut transaction_amount: f32 = 0.0;
        let mut transaction_receipt = String::from("");
        let mut transaction_date = String::from("");
        let mut phone_number = String::from("");

        let c2b_payment_result_parameters_output_details =
            if !list_of_items.is_empty() && list_of_items.len() > 0 {
                for _item in list_of_items.iter() {
                    let _name = &_item.Name;
                    let _value = &_item.Value;

                    // Amount
                    if _name
                        .to_string()
                        .to_lowercase()
                        .eq_ignore_ascii_case(&String::from("Amount"))
                    {
                        transaction_amount = match _value {
                            MixedTypeValue::StringValue(s) => 0.0,
                            MixedTypeValue::IntegerValue(i) => *i as f32,
                            MixedTypeValue::FloatValue(f) => *f,
                            _ => 0.0,
                        }
                    }

                    // TransactionReceipt
                    if _name
                        .to_string()
                        .to_lowercase()
                        .eq_ignore_ascii_case(&String::from("MpesaReceiptNumber"))
                    {
                        transaction_receipt = match _value {
                            MixedTypeValue::StringValue(s) => s.to_string(),
                            _ => String::from(""),
                        }
                    }

                    //transaction_date
                    if _name
                        .to_string()
                        .to_lowercase()
                        .eq_ignore_ascii_case(&String::from("TransactionDate"))
                    {
                        transaction_date = match _value {
                            MixedTypeValue::FloatValue(f) => f.to_string(),
                            _ => String::from(""),
                        }
                    }

                    // phone_number
                    if _name
                        .to_string()
                        .to_lowercase()
                        .eq_ignore_ascii_case(&String::from("PhoneNumber"))
                    {
                        phone_number = match _value {
                            MixedTypeValue::FloatValue(f) => f.to_string(),
                            _ => String::from(""),
                        }
                    }
                }

                let c2b_payment_result_parameters_output_details =
                    C2BPaymentResultParametersOutputDetails {
                        Amount: transaction_amount,
                        MpesaReceiptNumber: transaction_receipt,
                        TransactionDate: transaction_date,
                        PhoneNumber: phone_number,
                    };

                Some(c2b_payment_result_parameters_output_details)
            } else {
                None
            };

        c2b_payment_result_parameters_output_details
    }

    pub fn get_business_paybill_result_parameters_output_details(
        &self,
        result_parameters: &ResultParameter,
    ) -> BusinessPayBillResultParametersOutputDetails {
        let mut debit_account_balance = String::from("");
        let mut transaction_amount = String::from("");
        let mut debit_party_affected_account_balance = String::from("");
        let mut trans_completed_time = String::from("");
        let mut debit_party_charges = String::from("");
        let mut receiver_party_public_name = String::from("");
        let mut _currency = String::from("");
        let mut initiator_account_current_balance = String::from("");
        let mut bill_reference_number = String::from("");
        let mut queue_timeout_url = String::from("");

        for result_parameter in result_parameters.ResultParameter.iter() {
            let _key = &result_parameter.Key;
            let _value = &result_parameter.Value;

            // DebitAccountBalance
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("DebitAccountBalance"))
            {
                debit_account_balance = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // Amount
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("Amount"))
            {
                transaction_amount = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // DebitPartyAffectedAccountBalance
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("DebitPartyAffectedAccountBalance"))
            {
                debit_party_affected_account_balance = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // TransCompletedTime
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("TransCompletedTime"))
            {
                trans_completed_time = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // DebitPartyCharges
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("DebitPartyCharges"))
            {
                debit_party_charges = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // ReceiverPartyPublicName
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

            // Currency
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("Currency"))
            {
                _currency = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // InitiatorAccountCurrentBalance
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("InitiatorAccountCurrentBalance"))
            {
                initiator_account_current_balance = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }
        }

        let business_paybill_result_parameters_output_details =
            BusinessPayBillResultParametersOutputDetails {
                DebitAccountBalance: debit_account_balance,
                Amount: transaction_amount,
                DebitPartyAffectedAccountBalance: debit_party_affected_account_balance,
                TransCompletedTime: trans_completed_time,
                DebitPartyCharges: debit_party_charges,
                ReceiverPartyPublicName: receiver_party_public_name,
                Currency: _currency,
                InitiatorAccountCurrentBalance: initiator_account_current_balance,
            };

        business_paybill_result_parameters_output_details
    }

    pub fn get_business_paybill_Reference_item_output_details(
        &self,
        reference_data: &BusinessPayBillReferenceItem,
    ) -> BusinessPayBillReferenceItemOutputDetails {
        let mut debit_account_balance = String::from("");
        let mut transaction_amount = String::from("");
        let mut debit_party_affected_account_balance = String::from("");
        let mut trans_completed_time = String::from("");
        let mut debit_party_charges = String::from("");
        let mut receiver_party_public_name = String::from("");
        let mut _currency = String::from("");
        let mut initiator_account_current_balance = String::from("");
        let mut bill_reference_number = String::from("");
        let mut queue_timeout_url = String::from("");

        for reference_item in reference_data.ReferenceItem.iter() {
            let _key = &reference_item.Key;
            let _value = &reference_item.Value;

            // BillReferenceNumber
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("BillReferenceNumber"))
            {
                bill_reference_number = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // QueueTimeoutURL
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("QueueTimeoutURL"))
            {
                queue_timeout_url = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }
        }

        let business_paybill_result_parameters_output_details =
            BusinessPayBillReferenceItemOutputDetails {
                BillReferenceNumber: bill_reference_number,
                QueueTimeoutURL: queue_timeout_url,
            };

        business_paybill_result_parameters_output_details
    }

    pub fn get_business_buy_goods_result_parameters_output_details(
        &self,
        result_parameters: &ResultParameter,
    ) -> BusinessBuyGoodsResultParametersOutputDetails {
        let mut debit_account_balance = String::from("");
        let mut transaction_amount = String::from("");
        let mut debit_party_affected_account_balance = String::from("");
        let mut trans_completed_time = String::from("");
        let mut debit_party_charges = String::from("");
        let mut receiver_party_public_name = String::from("");
        let mut _currency = String::from("");
        let mut initiator_account_current_balance = String::from("");
        let mut bill_reference_number = String::from("");
        let mut queue_timeout_url = String::from("");

        for result_parameter in result_parameters.ResultParameter.iter() {
            let _key = &result_parameter.Key;
            let _value = &result_parameter.Value;

            // DebitAccountBalance
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("DebitAccountBalance"))
            {
                debit_account_balance = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // Amount
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("Amount"))
            {
                transaction_amount = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // DebitPartyAffectedAccountBalance
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("DebitPartyAffectedAccountBalance"))
            {
                debit_party_affected_account_balance = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // TransCompletedTime
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("TransCompletedTime"))
            {
                trans_completed_time = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // DebitPartyCharges
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("DebitPartyCharges"))
            {
                debit_party_charges = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // ReceiverPartyPublicName
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

            // Currency
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("Currency"))
            {
                _currency = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // InitiatorAccountCurrentBalance
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("InitiatorAccountCurrentBalance"))
            {
                initiator_account_current_balance = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }
        }

        let business_buy_goods_result_parameters_output_details =
            BusinessBuyGoodsResultParametersOutputDetails {
                DebitAccountBalance: debit_account_balance,
                Amount: transaction_amount,
                DebitPartyAffectedAccountBalance: debit_party_affected_account_balance,
                TransCompletedTime: trans_completed_time,
                DebitPartyCharges: debit_party_charges,
                ReceiverPartyPublicName: receiver_party_public_name,
                Currency: _currency,
                InitiatorAccountCurrentBalance: initiator_account_current_balance,
            };

        business_buy_goods_result_parameters_output_details
    }

    pub fn get_business_buy_goods_reference_item_output_details(
        &self,
        reference_data: &BusinessBuyGoodsReferenceItem,
    ) -> BusinessBuyGoodsReferenceItemOutputDetails {
        let mut debit_account_balance = String::from("");
        let mut transaction_amount = String::from("");
        let mut debit_party_affected_account_balance = String::from("");
        let mut trans_completed_time = String::from("");
        let mut debit_party_charges = String::from("");
        let mut receiver_party_public_name = String::from("");
        let mut _currency = String::from("");
        let mut initiator_account_current_balance = String::from("");
        let mut bill_reference_number = String::from("");
        let mut queue_timeout_url = String::from("");

        for reference_item in reference_data.ReferenceItem.iter() {
            let _key = &reference_item.Key;
            let _value = &reference_item.Value;

            // BillReferenceNumber
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("BillReferenceNumber"))
            {
                bill_reference_number = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // QueueTimeoutURL
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("QueueTimeoutURL"))
            {
                queue_timeout_url = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }
        }

        let business_paybill_result_parameters_output_details =
            BusinessBuyGoodsReferenceItemOutputDetails {
                BillReferenceNumber: bill_reference_number,
                QueueTimeoutURL: queue_timeout_url,
            };

        business_paybill_result_parameters_output_details
    }

    pub fn get_business_paybill_timeout_parameters_output_details(
        &self,
        result_parameter: &BusinessPayBillFailedResultParameter,
        reference_data: &ReferenceItemDetails,
    ) -> BusinessPayBillTimeoutParametersOutputDetails {
        let mut bo_completed_time = String::from("");
        let mut queue_timeout_url = String::from("");

        let result_parameter_key = &result_parameter.ResultParameter.Key;
        let result_parameter_value = &result_parameter.ResultParameter.Value;

        let reference_data_key = &reference_data.Key;
        let reference_data_value = &reference_data.Value;

        // BOCompletedTime
        if result_parameter_key
            .to_string()
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("BOCompletedTime"))
        {
            bo_completed_time = match result_parameter_value {
                MixedTypeValue::FloatValue(s) => s.to_string(),
                _ => String::from(""),
            }
        }

        // QueueTimeoutURL
        if reference_data_key
            .to_string()
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("QueueTimeoutURL"))
        {
            queue_timeout_url = match reference_data_value {
                s => s.to_string(),
                _ => String::from(""),
            }
        }

        let business_paybill_timeout_parameters_output_details =
            BusinessPayBillTimeoutParametersOutputDetails {
                BOCompletedTime: bo_completed_time,
                QueueTimeoutURL: queue_timeout_url,
            };

        business_paybill_timeout_parameters_output_details
    }

    pub fn get_business_buy_goods_timeout_parameters_output_details(
        &self,
        result_parameter: &BusinessBuyGoodsFailedResultParameter,
        reference_data: &ReferenceItemDetails,
    ) -> BusinessBuyGoodsTimeoutParametersOutputDetails {
        let mut bo_completed_time = String::from("");
        let mut queue_timeout_url = String::from("");

        let result_parameter_key = &result_parameter.ResultParameter.Key;
        let result_parameter_value = &result_parameter.ResultParameter.Value;

        let reference_data_key = &reference_data.Key;
        let reference_data_value = &reference_data.Value;

        // BOCompletedTime
        if result_parameter_key
            .to_string()
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("BOCompletedTime"))
        {
            bo_completed_time = match result_parameter_value {
                MixedTypeValue::FloatValue(s) => s.to_string(),
                _ => String::from(""),
            }
        }

        // QueueTimeoutURL
        if reference_data_key
            .to_string()
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("QueueTimeoutURL"))
        {
            queue_timeout_url = match reference_data_value {
                s => s.to_string(),
                _ => String::from(""),
            }
        }

        let business_buy_goods_timeout_parameters_output_details =
            BusinessBuyGoodsTimeoutParametersOutputDetails {
                BOCompletedTime: bo_completed_time,
                QueueTimeoutURL: queue_timeout_url,
            };

        business_buy_goods_timeout_parameters_output_details
    }

    /*
    fn build_business_to_customer_response_data(
        &self,
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

    fn build_business_to_customer_error_response_data(
        &self,
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
    */

    async fn get_auth_token(&self) -> std::result::Result<String, String> {
        let api_key = self.get_api_key();

        let api_url = &self.auth_token_url;

        let _result =
            authorization::generate_auth_token::get_auth_token(api_key, api_url.to_string()).await;

        _result
    }

    fn parse_auth_token(&self, access_token_result: String) -> String {
        let access_token: String = if !access_token_result.is_empty()
            && access_token_result.replace(" ", "").trim().len() > 0
        {
            let mut access_token = AUTHORISATION_BEARER.to_string();
            let k = " "; // Separator
            access_token.push_str(k);
            access_token.push_str(&access_token_result);

            access_token
        } else {
            String::from("")
        };

        access_token
    }

    pub async fn register_url(
        &self,
        register_url_details: RegisterUrlInputDetails,
    ) -> std::result::Result<RegisterUrlResponseData, String> {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let access_token: String = self.parse_auth_token(access_token_result);

                let _result = customer_to_business::register_url::register_url(
                    register_url_details,
                    access_token,
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }

    pub async fn b2c(
        &self,
        business_to_customer_details: BusinessToCustomerInputDetails,
    ) -> std::result::Result<
        (
            Option<BusinessToCustomerResponseData>,
            Option<BusinessToCustomerErrorResponseData>,
        ),
        String,
    > {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let access_token: String = self.parse_auth_token(access_token_result);

                let _result = business_to_customer::business_to_customer::b2c(
                    business_to_customer_details,
                    access_token,
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }

    pub async fn c2b_payment(
        &self,
        customer_to_business_details: CustomerToBusinessPaymentInputDetails,
    ) -> std::result::Result<
        (
            Option<CustomerToBusinessPaymentResponseData>,
            Option<CustomerToBusinessPaymentErrorResponseData>,
        ),
        String,
    > {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let access_token: String = self.parse_auth_token(access_token_result);

                let _result = customer_to_business::customer_to_business_payment::c2b_payment(
                    customer_to_business_details,
                    access_token,
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }

    pub async fn business_paybill(
        &self,
        business_paybill_details: BusinessPayBillInputDetails,
    ) -> std::result::Result<
        (
            Option<BusinessPayBillResponseData>,
            Option<BusinessPayBillErrorResponseData>,
        ),
        String,
    > {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let access_token: String = self.parse_auth_token(access_token_result);

                let _result = business_paybill::business_paybill::pay_bill(
                    business_paybill_details,
                    access_token,
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }

    pub async fn business_buy_goods(
        &self,
        business_buy_goods_details: BusinessBuyGoodsInputDetails,
    ) -> std::result::Result<
        (
            Option<BusinessBuyGoodsResponseData>,
            Option<BusinessBuyGoodsErrorResponseData>,
        ),
        String,
    > {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let access_token: String = self.parse_auth_token(access_token_result);

                let _result = business_buy_goods::business_buy_goods::buy_goods(
                    business_buy_goods_details,
                    access_token,
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/
