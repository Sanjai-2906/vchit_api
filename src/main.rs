use std::time::Duration;

use axum::{
    Router,
    routing::{get, post},
};
mod handlers;
mod models;
mod get_connection;

use handlers::{
    add_collection::add_collection, collections::get_collections, due_amount::get_due_amount,
    groups::get_groups, members::get_members, summary_breakup::summary_breakup,
    user_login::user_login,
};
use oracle::pool::{GetMode, Pool, PoolBuilder};
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
}

#[derive(Clone)]
pub struct AppConfig {
    pub oracle_user: String,
    pub oracle_password: String,
    pub oracle_connect_string: String,
}
impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            oracle_user: std::env::var("ORACLE_USER").expect("ORACLE_USER not set"),
            oracle_password: std::env::var("ORACLE_PASSWORD").expect("ORACLE_PASSWORD not set"),
            oracle_connect_string: std::env::var("ORACLE_CONNECT_STRING")
                .expect("ORACLE_CONNECT_STRING not set"),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let config = AppConfig::from_env();

    let mut builder = PoolBuilder::new(
        &config.oracle_user,
        &config.oracle_password,
        &config.oracle_connect_string,
    );

    builder
        .min_connections(1)
        .max_connections(50)
        .connection_increment(1)
        .get_mode(GetMode::TimedWait(Duration::from_secs(10)))
        .ping_interval(Some(Duration::from_secs(30)))
        .map_err(|err: oracle::Error| err.to_string())?
        .ping_timeout(Duration::from_secs(5))
        .map_err(|err: oracle::Error| err.to_string())?
        .timeout(Duration::from_secs(300))
        .map_err(|err: oracle::Error| err.to_string())?
        .max_lifetime_connection(Duration::from_secs(3600))
        .map_err(|err: oracle::Error| err.to_string())?;

    let pool = builder
        .build()
        .map_err(|err: oracle::Error| err.to_string())?;

    let state = AppState { pool };
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/ping", post(ping))
        .route("/login", post(user_login))
        .route("/new_collection", post(add_collection))
        .route("/collectionlist", post(get_collections))
        .route("/due-amount", post(get_due_amount))
        .route("/members", get(get_members))
        .route("/members/{grpName}", get(get_members))
        .route("/groups", get(get_groups))
        .route("/summary", post(summary_breakup))
        .layer(cors)
        // .with_state(config);
        .with_state(state);

    let ip_port = format!("0.0.0.0:5000");
    println!("Server start at {}", ip_port);
    let listener = tokio::net::TcpListener::bind(ip_port).await.unwrap();
    // axum::serve(listener, app).await.unwrap();
    let _ = axum::serve(listener, app)
        .await
        .map_err(|err| err.to_string())?;

    Ok(())
}

async fn ping(msg: String) -> String {
    if msg == "ping" {
        return format!("Pong");
    }
    return format!("Invalid Message");
}
