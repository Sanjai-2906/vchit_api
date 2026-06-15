use axum::Json;
use oracle::Connection;

use crate::models::CollectionModel;

pub async fn add_collection(Json(collection): Json<CollectionModel>) {
    println!("User Collection : {:#?}",collection);
    let conn = Connection::connect("vvcpl", "log", "velcloud.in:1521/XE").unwrap();
    
    conn.execute(
        "INSERT INTO MOB (
            DOCID, DOCDATE, COLLECTEDBY, GROUPNO, PARTYMASTID, PARTYID, 
            MOBILE, AMOUNT, TYPE, DUEAMOUNT, BAL, CHEQUEDATE, CHEQUENO
         )
         VALUES (
            :1, TO_DATE(:2, 'YYYY-MM-DD'), :3, :4, :5, :6, 
            :7, :8, :9, :10, :11, TO_DATE(:12, 'YYYY-MM-DD'), :13
         )",
        &[ 
            &collection.doc_id,
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