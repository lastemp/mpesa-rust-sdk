# unpack validation c2b payment

This functionality unpacks json "validation c2b" request data.

## validation_c2b

This code sample shows how to unpack json "validation c2b" data.

```rust
use mpesa_rust_sdk::models::C2bData;

#[post("/validationc2b")]
pub(crate) async fn validation_c2b(
    validation_data: web::Json<C2bData>,
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
```
