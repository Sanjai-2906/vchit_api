use crate::{AppState, models::MemberModel};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

pub async fn get_members(
    // State(config): State<AppConfig>,
    State(state): State<AppState>,
    grp_name: Option<Path<String>>,
) -> Result<Json<Vec<MemberModel>>, (StatusCode, String)> {
    // let conn = Connection::connect(
    //     &config.oracle_user,
    //     &config.oracle_password,
    //     &config.oracle_connect_string,
    // )
    // .map_err(|err| {
    //     eprintln!("Database Connection Error: {:?}", err);
    //     (
    //         StatusCode::INTERNAL_SERVER_ERROR,
    //         "Database connection failure".to_string(),
    //     )
    // })?;
    let conn = state.pool.get().map_err(|err| {
        eprintln!("Database Connection Error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database connection failure".to_string(),
        )
    })?;

    let mut members = Vec::new();

    let rows = match grp_name {
        Some(Path(grp)) => conn
            .query(
                "SELECT
                        PARTYMASTID,
                        PARTYID,
                        MAX(MOBILE) AS MOBILE
                    FROM CHITLIST
                    WHERE CHITGROUPNO = :1
                    GROUP BY PARTYMASTID, PARTYID
                    ORDER BY PARTYID",
                &[&grp],
            )
            .map_err(|err| {
                eprintln!("Database Query Error (with group): {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database operational error occurred".to_string(),
                )
            })?,
        None => conn
            .query(
                "select partymastid, partyid, mobile
                 from chitlist",
                &[],
            )
            .map_err(|err| {
                eprintln!("Database Query Error (all): {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database operational error occurred".to_string(),
                )
            })?,
    };

    for (index, row_result) in rows.into_iter().enumerate() {
        let row = row_result.map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse row at index: {}", err),
            )
        })?;

        let member_id_opt: Option<i64> = row.get(0).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Column type mismatch at index 0: {}", e),
            )
        })?;

        let member_name: Option<String> = row.get(1).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Column type mismatch at index 1: {}", e),
            )
        })?;

        let mobile: Option<String> = row.get(2).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Column type mismatch at index 2: {}", e),
            )
        })?;

        let member_id = match member_id_opt {
            Some(id) => id,
            None => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    format!(
                        "Database constraint failure: Missing PARTYMASTID at row index {}.",
                        index
                    ),
                ));
            }
        };

        if let Some(name) = member_name {
            members.push(MemberModel {
                member_id,
                member_name: name,
                mobile,
            });
        }
    }

    Ok(Json(members))
}
