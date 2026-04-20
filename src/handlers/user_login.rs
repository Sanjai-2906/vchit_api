use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::models::LoginModel;


pub async fn user_login(Json(data) : Json<LoginModel>) -> impl IntoResponse{
    if data.name != "admin" {
        return (StatusCode::UNAUTHORIZED, "Unknown name").into_response();
    }

    if data.pass != "1234" {
        return (StatusCode::UNAUTHORIZED, "Password wrong").into_response();
    }

    (StatusCode::OK, "Success").into_response()
}