use crate::models::SummaryBreakupModel;
use crate::{AppState, models::CollectionRequestModel};
use axum::{
    Json,
    extract::State,
    http::StatusCode,
};
pub async fn summary_breakup(
    State(state): State<AppState>,
    Json(user_data): Json<CollectionRequestModel>,
) -> Result<Json<SummaryBreakupModel>, (StatusCode, String)> {
    let conn = state.pool.get().map_err(|err| {
        eprintln!("Database Connection Error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database connection failure".to_string(),
        )
    })?;
    let mut rows = conn
        .query(
            "SELECT
                    SUM(AMOUNT) AS TOTAL_AMOUNT,
                    SUM(CASE WHEN TYPE = 'Cash' THEN AMOUNT ELSE 0 END) AS CASH_AMOUNT,
                    SUM(CASE WHEN TYPE = 'UPI' THEN AMOUNT ELSE 0 END) AS UPI_AMOUNT,
                    SUM(CASE WHEN TYPE = 'Cheque' THEN AMOUNT ELSE 0 END) AS CHEQUE_AMOUNT
                FROM MOB
                WHERE COLLECTEDBY = :1
                    AND DOCDATE >= TO_DATE(:2, 'YYYY-MM-DD')
                    AND DOCDATE < TO_DATE(:2, 'YYYY-MM-DD') + 1",
            &[&user_data.agent_name, &user_data.doc_date],
        )
        .map_err(|err| {
            eprintln!("Database Query Error: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database operational error occurred".to_string(),
            )
        })?;

    let row = rows.next().unwrap().unwrap();

    let summary = SummaryBreakupModel {
        total_amount: row.get::<_, Option<f64>>(0).unwrap().unwrap_or(0.0),
        cash_amount: row.get::<_, Option<f64>>(1).unwrap().unwrap_or(0.0),
        upi_amount: row.get::<_, Option<f64>>(2).unwrap().unwrap_or(0.0),
        cheque_amount: row.get::<_, Option<f64>>(3).unwrap().unwrap_or(0.0),
    };

    Ok(Json(summary))
}
