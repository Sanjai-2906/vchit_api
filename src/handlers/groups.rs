use crate::{AppState, get_connection::get_connection, models::GroupModel};
use axum::{Json, extract::State, http::StatusCode};

pub async fn get_groups(
    // State(config): State<AppConfig>,
    State(state): State<AppState>,
) -> Result<Json<Vec<GroupModel>>, (StatusCode, String)> {
    let conn = get_connection(&state.pool).await?;

    let rows = {
        conn.query(
            "select chitbasicid, chitgroupno
         from chitlist
         group by chitbasicid, chitgroupno
         order by 2",
            &[],
        )
        .map_err(|err| {
            eprintln!("Database Query Error: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database operational error occurred".to_string(),
            )
        })?
    };

    let mut groups = Vec::new();

    for (index, row_result) in rows.into_iter().enumerate() {
        let row = row_result.map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse row at index: {}", err),
            )
        })?;

        let group_id: Option<i64> = row
            .get(0)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        let group_name: Option<String> = row
            .get(1)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        match (group_id, group_name) {
            (Some(id), Some(name)) => {
                groups.push(GroupModel {
                    group_id: id,
                    group_name: name,
                });
            }
            _ => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    format!(
                        "Database requirement constraint failed: Row index {} contains a NULL value.",
                        index
                    ),
                ));
            }
        }
    }

    Ok(Json(groups))
}
