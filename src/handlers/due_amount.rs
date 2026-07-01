use axum::Json;
use oracle::Connection;

use crate::models::{DueModel, GetDueModel};

pub async fn get_due_amount(Json(data): Json<GetDueModel>) -> Json<DueModel> {
    println!("Get Due Amount: {:?}", data);
    let conn = Connection::connect("vvcpl", "log", "velcloud.in:1521/XE").unwrap();

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
