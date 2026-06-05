use axum::{Json, http::StatusCode, response::IntoResponse};
use oracle::Connection;

use crate::models::LoginModel;

pub async fn user_login(Json(data): Json<LoginModel>) -> impl IntoResponse {
    let conn = match Connection::connect("vvcpl", "log", "velcloud.in:1521/XE") {
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
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database query failed").into_response();
        }
    };
    for row_result in rows {
        if let Ok(row) = row_result {
            let db_password: Option<String> = row.get(1).unwrap_or(None);

            if let Some(stored_password) = db_password {
                if stored_password == data.pass {
                    return (StatusCode::OK, "Success").into_response();
                } else {
                    return (StatusCode::UNAUTHORIZED, "Password Incorrect").into_response();
                }
            }
        }
    }
    (StatusCode::UNAUTHORIZED, "User name not found").into_response()
}
