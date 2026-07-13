use axum::{Json, extract::State, http::StatusCode};
use crate::{
    AppState,
    models::{DueModel, GetDueModel},
};

// pub async fn get_due_amount(
//     State(config): State<AppConfig>,
//     Json(data): Json<GetDueModel>,
// ) -> Json<DueModel> {
//     let conn = Connection::connect(
//         &config.oracle_user,
//         &config.oracle_password,
//         &config.oracle_connect_string,
//     )
//     .unwrap();
//     let row = conn
//         .query_row(
//             "SELECT CURRENTDUE, NEXTBALANCE FROM CHITLIST WHERE PARTYMASTID=:1",
//             &[&data.member_id],
//         )
//         .unwrap();

//     let balance: f64 = row.get(0).unwrap();
//     let next_balance: f64 = row.get(1).unwrap();
//     Json(DueModel {
//         balance,
//         next_balance,
//     })
// }

pub async fn get_due_amount(
    State(state): State<AppState>,
    Json(data): Json<GetDueModel>,
) -> Result<Json<DueModel>, (StatusCode, String)> {
    let conn = state.pool.get().map_err(|err| {
        eprintln!("Database Connection Error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database connection failure".to_string(),
        )
    })?;

    let row = conn
        .query_row(
            "SELECT CURRENTDUE, NEXTBALANCE
             FROM CHITLIST
             WHERE PARTYMASTID = :1",
            &[&data.member_id],
        )
        .map_err(|err| {
            eprintln!("Database Query Error: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database query failed".to_string(),
            )
        })?;

    let balance: Option<f64> = row
        .get(0)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let next_balance: Option<f64> = row
        .get(1)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match (balance, next_balance) {
        (Some(balance), Some(next_balance)) => Ok(Json(DueModel {
            balance,
            next_balance,
        })),
        _ => Err((
            StatusCode::BAD_REQUEST,
            "CURRENTDUE or NEXTBALANCE is NULL".to_string(),
        )),
    }
}
