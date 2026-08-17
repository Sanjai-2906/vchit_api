use std::time::Duration;
use axum::http::StatusCode;
use oracle::Connection;
use tokio::time::sleep;

pub async fn get_connection(pool: &oracle::pool::Pool) -> Result<Connection, (StatusCode, String)> {
    let mut retries = 5;
    let retry_delay = Duration::from_millis(300);

    while retries > 0 {
        match pool.get() {
            Ok(conn) => {
                // Verify that the pooled connection is still alive and responsive
                match conn.ping() {
                    Ok(_) => return Ok(conn),
                    Err(err) => {
                        eprintln!(
                            "Stale database connection detected via ping, discarding and retrying: {:?}",
                            err
                        );
                        // Dropping `conn` causes ODPI-C to clean up and replace the broken session
                    }
                }
            }
            Err(err) => {
                eprintln!("Database Connection Pool Error: {:?}", err);
            }
        }

        retries -= 1;
        if retries > 0 {
            sleep(retry_delay).await;
        }
    }

    Err((
        StatusCode::SERVICE_UNAVAILABLE,
        "Database is currently unavailable. Please try your request again shortly.".to_string(),
    ))
}
