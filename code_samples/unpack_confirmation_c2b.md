# unpack confirmation c2b payment

This functionality unpacks json "confirmation c2b" request data.

## confirmation_c2b

This code sample shows how to unpack json "confirmation c2b" data.

```rust
use mpesa_rust_sdk::models::C2bData;

#[post("/confirmationc2b")]
pub(crate) async fn confirmation_c2b(
    confirmation_data: web::Json<C2bData>,
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
```
