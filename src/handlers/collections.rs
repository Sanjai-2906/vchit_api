use crate::AppState;
use crate::models::{CollectionRequestModel, CollectionResponseModel};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use chrono::NaiveDate;
pub async fn get_collections(
    State(state): State<AppState>,
    Json(user_data): Json<CollectionRequestModel>,
) -> Result<Json<Vec<CollectionResponseModel>>, (StatusCode, String)> {
    let conn = state.pool.get()
    .map_err(|err| {
        eprintln!("Database Connection Error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database connection failure".to_string(),
        )
    })?;

    let rows = conn
        .query(
            "SELECT PARTYMASTID,
                    PARTYID,
                    GROUPNO,
                    AMOUNT,
                    TYPE,
                    CHEQUENO,
                    CHEQUEDATE,
                    DOCID
             FROM MOB
             WHERE COLLECTEDBY = :1
               AND DOCDATE >= TO_DATE(:2, 'YYYY-MM-DD')
               AND DOCDATE < TO_DATE(:2, 'YYYY-MM-DD') + 1
             ORDER BY MOBID DESC",
            &[&user_data.agent_name, &user_data.doc_date],
        )
        .map_err(|err| {
            eprintln!("Database Query Error: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database operational error occurred".to_string(),
            )
        })?;

    let mut collections = Vec::new();

    for (index, row_result) in rows.into_iter().enumerate() {
        let row = row_result.map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse row at index {}: {}", index, err),
            )
        })?;

        let party_mast_id: Option<i64> = row
            .get(0)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        let party_id: Option<String> = row
            .get(1)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        let group_no: Option<String> = row
            .get(2)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        let amount: Option<f64> = row
            .get(3)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        let collection_type: Option<String> = row
            .get(4)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        
       let cheque_no: Option<String> = row
            .get(5)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        let cheque_date: Option<NaiveDate> = row
            .get(6)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        let doc_id: Option<String> = row
            .get(7)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        match (
            party_mast_id,
            party_id,
            group_no,
            amount,
            collection_type,
            doc_id,
        ) {
            (
                Some(party_mast_id),
                Some(party_id),
                Some(group_no),
                Some(amount),
                Some(collection_type),
                Some(doc_id),
            ) => {
                collections.push(CollectionResponseModel {
                    party_mast_id,
                    party_id,
                    group_no,
                    amount,
                    collection_type,
                    cheque_no,
                    cheque_date,
                    doc_id,
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

    Ok(Json(collections))
}
