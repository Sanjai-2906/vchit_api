use axum::{Json, extract::State};
use oracle::Connection;

use crate::{
    AppConfig,
    models::{DueModel, GetDueModel},
};

pub async fn get_due_amount(
    State(config): State<AppConfig>,
    Json(data): Json<GetDueModel>,
) -> Json<DueModel> {
    let conn = Connection::connect(
        &config.oracle_user,
        &config.oracle_password,
        &config.oracle_connect_string,
    )
    .unwrap();
    let row = conn
        .query_row(
            "SELECT CURRENTDUE, NEXTBALANCE FROM CHITLIST WHERE PARTYMASTID=:1",
            &[&data.member_id],
        )
        .unwrap();

    let balance: f64 = row.get(0).unwrap();
    let next_balance: f64 = row.get(1).unwrap();
    Json(DueModel {
        balance,
        next_balance,
    })
}
