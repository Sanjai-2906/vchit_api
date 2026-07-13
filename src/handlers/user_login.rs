use crate::AppState;
use crate::models::LoginModel;
use axum::{Json, extract::State, http::StatusCode};
pub async fn user_login(
    State(state): State<AppState>,
    Json(data): Json<LoginModel>,
) -> Result<Json<u64>, (StatusCode, String)> {
    let conn = state.pool.get().map_err(|err| {
        eprintln!("Database Connection Error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database connection failure".to_string(),
        )
    })?;

    let rows = conn
        .query(
            "SELECT auser1id, apassword
             FROM auser1
             WHERE ausername = :1",
            &[&data.name],
        )
        .map_err(|err| {
            eprintln!("Database Query Error: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database query failure".to_string(),
            )
        })?;

    for row_result in rows {
        let row = row_result.map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to read row: {}", err),
            )
        })?;

        let id: Option<u64> = row
            .get(0)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        let db_password: Option<String> = row
            .get(1)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        match (id, db_password) {
            (Some(id), Some(password)) => {
                if password == data.pass {
                    return Ok(Json(id));
                } else {
                    return Err((StatusCode::UNAUTHORIZED, "Password Incorrect".to_string()));
                }
            }
            _ => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "User record contains NULL values".to_string(),
                ));
            }
        }
    }

    Err((StatusCode::UNAUTHORIZED, "User name not found".to_string()))
}
