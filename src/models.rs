use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CollectionModel {
    // pub doc_id: String,
    pub doc_date: String,
    pub collected_by: String,
    pub group_no: String,
    pub party_mast_id: u64,
    pub party_id: String,
    pub mobile: Option<String>,
    pub amount: f64,
    #[serde(rename = "type")]
    pub r#type: String,
    pub due_amount: Option<f64>,
    pub bal: f64,
    pub cheque_date: Option<String>,
    pub cheque_no: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummaryBreakupModel {
    pub total_amount: f64,
    pub cash_amount: f64,
    pub upi_amount: f64,
    pub cheque_amount: f64,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDueModel {
    pub grp_name: String,
    pub member_id: i64,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DueModel {
    pub balance: f64,
    pub next_balance: f64,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginModel {
    pub name: String,
    pub pass: String,
    pub logged_at: NaiveDateTime,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupModel {
    pub group_id: i64,
    pub group_name: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberModel {
    pub member_id: i64,
    pub member_name: String,
    pub mobile: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionResponseModel {
    pub party_id: String,
    pub doc_id: String,
    pub party_mast_id: i64,
    pub group_no: String,
    pub collection_type: String,
    pub cheque_no: Option<String>,
    pub cheque_date: Option<NaiveDate>,
    pub amount: f64,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionRequestModel {
    pub agent_name: String,
    pub doc_date: String,
}
