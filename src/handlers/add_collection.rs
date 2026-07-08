use axum::{Json, extract::State};
use oracle::Connection;

use crate::{AppConfig, models::CollectionModel};

pub async fn add_collection(
    State(config): State<AppConfig>,
    Json(collection): Json<CollectionModel>,
) {
    let conn = Connection::connect(
        &config.oracle_user,
        &config.oracle_password,
        &config.oracle_connect_string,
    )
    .unwrap();
    conn.execute(
        "INSERT INTO MOB (
        DOCID, DOCDATE, COLLECTEDBY, GROUPNO, PARTYMASTID, PARTYID,
        MOBILE, AMOUNT, TYPE, DUEAMOUNT, BAL, CHEQUEDATE, CHEQUENO
    )
    VALUES (
        MOB_DOCID_SEQ.NEXTVAL,
        TO_DATE(:1, 'YYYY-MM-DD'),
        :2, :3, :4, :5, :6,
        :7, :8, :9, :10,
        TO_DATE(:11, 'YYYY-MM-DD'),
        :12
    )",
        &[
            &collection.doc_date,
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
    .unwrap();

    conn.commit().unwrap();
}
