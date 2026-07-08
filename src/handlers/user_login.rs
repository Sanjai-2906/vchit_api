use crate::AppConfig;
use crate::models::LoginModel;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use oracle::Connection;

pub async fn user_login(
    State(config): State<AppConfig>,
    Json(data): Json<LoginModel>,
) -> impl IntoResponse {
    let conn = match Connection::connect(
        &config.oracle_user,
        &config.oracle_password,
        &config.oracle_connect_string,
    ) {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database connection error",
            )
                .into_response();
        }
    };
    let stmt_result = conn.query(
        "select auser1id, apassword from auser1 where ausername = :1",
        &[&data.name],
    );
    let rows = match stmt_result {
        Ok(r) => r,
        Err(_) => {
            println!("User name not found");
            return (StatusCode::UNAUTHORIZED, "User name not found").into_response();
        }
    };
    for row_result in rows {
        if let Ok(row) = row_result {
            let db_password: Option<String> = row.get(1).unwrap_or(None);
            let id: Option<u64> = row.get(0).unwrap_or(None);

            if let Some(stored_password) = db_password {
                if stored_password == data.pass {
                    return (StatusCode::OK, Json(id)).into_response();
                } else {
                    println!("password wrong");
                    return (StatusCode::UNAUTHORIZED, "Password Incorrect").into_response();
                }
            }
        }
    }
    return (StatusCode::OK, "Executed").into_response();
}
