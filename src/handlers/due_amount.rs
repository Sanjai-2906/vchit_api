use axum::Json;
use oracle::Connection;

use crate::models::{DueModel, GetDueModel};

pub async fn get_due_amount(Json(data): Json<GetDueModel>) -> Json<DueModel> {
    println!("Get Due Amount: {:?}", data);
    let conn = Connection::connect("vvcpl", "log", "velcloud.in:1521/XE").unwrap();

    let mut rows = conn
        .query(
            "SELECT CURRENTDUE, NEXTBALANCE FROM CHITLIST WHERE PARTYMASTID=:1",
            &[&data.member_id],
        )
        .unwrap();
    let (balance, next_balance) = match rows.next() {
        Some(row_result) => {
            let row = row_result.unwrap();

            (
                row.get::<_, Option<f64>>(0).unwrap(),
                row.get::<_, Option<f64>>(1).unwrap(),
            )
        }
        None => (Some(0.0), Some(0.0)),
    };

    Json(DueModel {
        balance,
        next_balance,
    })
}
