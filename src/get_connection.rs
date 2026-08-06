use tokio::time::{Duration,sleep};

use axum::http::StatusCode;
use oracle::Connection;


pub async fn get_connection(pool: &oracle::pool::Pool) -> Result<Connection, (StatusCode, String)> {
    let mut conn = None;
    let mut retries = 10;
    let retry_delay = Duration::from_millis(500);

    while retries > 0 {
        match pool.get() {
            Ok(connection) => {
                conn = Some(connection);
                break;
            }
            Err(err) => {
                eprintln!(
                    "Database Connection Error: {:?}",
                    err,
                );
                retries -= 1;

                if retries > 0 {
                    sleep(retry_delay).await;
                }
            }
        }
    }

    match conn {
        Some(c) => Ok(c),
        None => Err((
            StatusCode::SERVICE_UNAVAILABLE,
            "Server is busy. Please try your request again shortly.".to_string(),
        )),
    }
}
