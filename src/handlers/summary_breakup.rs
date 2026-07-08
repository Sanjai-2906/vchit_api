use crate::AppConfig;
use crate::models::SummaryBreakupModel;
use axum::{Json, extract::{Path,State}};
use oracle::Connection;

pub async fn summary_breakup(
    State(config): State<AppConfig>,
    Path(agent_name): Path<String>,
) -> Json<SummaryBreakupModel> {
    let conn = Connection::connect(
        &config.oracle_user,
        &config.oracle_password,
        &config.oracle_connect_string,
    )
    .unwrap();
    let mut rows = conn
        .query(
            "SELECT
                    SUM(AMOUNT) AS TOTAL_AMOUNT,
                    SUM(CASE WHEN TYPE = 'Cash' THEN AMOUNT ELSE 0 END) AS CASH_AMOUNT,
                    SUM(CASE WHEN TYPE = 'UPI' THEN AMOUNT ELSE 0 END) AS UPI_AMOUNT,
                    SUM(CASE WHEN TYPE = 'Cheque' THEN AMOUNT ELSE 0 END) AS CHEQUE_AMOUNT
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
