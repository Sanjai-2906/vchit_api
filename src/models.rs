use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize,Default,Clone)]
#[serde(rename_all = "lowercase")]
pub enum Mode{
    #[default]
    CASH,
    UPI,
    CHEQUE
}

#[derive(Debug,Serialize,Deserialize,Default,Clone)]
#[serde(rename_all = "camelCase")]
pub struct CollectionModel{
    pub grp_name: String,
    pub member_name: String,
    pub amount: f32,
    pub mode: Mode,
    pub cheque_no: Option<String>,
    pub bank: Option<String>,
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummaryBreakupModel{
    pub total_amount: f32,
    pub cash_amount: f32,
    pub upi_amount: f32,
    pub cheque_amount: f32,
}
#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDueModel{
    pub grp_name: String,
    pub member_id: i64,
}
#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DueModel{
    pub balance: f64,
}
#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginModel{
    pub name: String,
    pub pass: String,
}
#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupModel{
    pub group_id: i64,
    pub group_name: String,
}
#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberModel{
    pub member_id: i64,
    pub member_name: String,
}
