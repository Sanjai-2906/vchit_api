use axum::{Json, extract::State, http::StatusCode};
use crate::{AppState, get_connection::get_connection, models::CollectionModel};

pub async fn add_collection(
    State(state): State<AppState>,
    Json(collection): Json<CollectionModel>,
) -> Result<StatusCode, (StatusCode, String)> {
    let conn = get_connection(&state.pool).await?;

    conn.execute(
        "INSERT INTO MOB (
            DOCID,
            DOCDATE,
            COLLECTEDBY,
            GROUPNO,
            PARTYMASTID,
            PARTYID,
            MOBILE,
            AMOUNT,
            TYPE,
            DUEAMOUNT,
            BAL,
            CHEQUEDATE,
            CHEQUENO
        )
        VALUES (
            MOB_DOCID_SEQ.NEXTVAL,
            TO_DATE(:1, 'YYYY-MM-DD'),
            :2,
            :3,
            :4,
            :5,
            :6,
            :7,
            :8,
            :9,
            :10,
            TO_DATE(:11, 'YYYY-MM-DD'),
            :12
        )",
        &[
            &collection.doc_date.to_string(),
            &collection.collected_by,
            &collection.group_no,
            &collection.party_mast_id,
            &collection.party_id,
            &collection.mobile,
            &collection.amount,
            &collection.r#type,
            &collection.due_amount,
            &collection.bal,
            &collection.cheque_date,
            &collection.cheque_no,
        ],
    )
    .map_err(|err| {
        eprintln!("Database Insert Error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to insert collection".to_string(),
        )
    })?;

    conn.commit().map_err(|err| {
        eprintln!("Database Commit Error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to commit transaction".to_string(),
        )
    })?;

    Ok(StatusCode::CREATED)
}