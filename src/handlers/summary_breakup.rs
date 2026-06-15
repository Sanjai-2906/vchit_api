use axum::{Json, extract::Path};
use oracle::Connection;

use crate::models::SummaryBreakupModel;

// use crate::state::COLLECTIONS;

pub async fn summary_breakup(Path(agent_name): Path<String>) -> Json<SummaryBreakupModel> {
    println!("Summary - Agent Name: {}",agent_name);
    let conn = Connection::connect("vvcpl", "log", "velcloud.in:1521/XE").unwrap();

    let mut rows = conn
        .query(
            "SELECT
                    SUM(AMOUNT) AS TOTAL_AMOUNT,
                    SUM(CASE WHEN TYPE = 'CASH' THEN AMOUNT ELSE 0 END) AS CASH_AMOUNT,
                    SUM(CASE WHEN TYPE = 'UPI' THEN AMOUNT ELSE 0 END) AS UPI_AMOUNT,
                    SUM(CASE WHEN TYPE = 'CHEQUE' THEN AMOUNT ELSE 0 END) AS CHEQUE_AMOUNT
                FROM MOB
                WHERE COLLECTEDBY = :1",
            &[&agent_name],
        )
        .unwrap();

    let row = rows.next().unwrap().unwrap();

    let summary = SummaryBreakupModel {
        total_amount: row.get::<_, Option<f64>>(0).unwrap().unwrap_or(0.0),
        cash_amount: row.get::<_, Option<f64>>(1).unwrap().unwrap_or(0.0),
        upi_amount: row.get::<_, Option<f64>>(2).unwrap().unwrap_or(0.0),
        cheque_amount: row.get::<_, Option<f64>>(3).unwrap().unwrap_or(0.0),
    };

    Json(summary)
}
