use crate::AppConfig;
use crate::models::GetCollectionModel;
use axum::Json;
use axum::extract::{Path, State};
use oracle::Connection;

pub async fn get_collections(
    State(config): State<AppConfig>,
    Path(agent_name): Path<String>,
) -> Json<Vec<GetCollectionModel>> {
    let conn = Connection::connect(
        &config.oracle_user,
        &config.oracle_password,
        &config.oracle_connect_string,
    )
    .unwrap();
    let rows = conn
        .query(
            "SELECT PARTYMASTID, PARTYID, GROUPNO, 
             AMOUNT, TYPE, DOCID  FROM MOB WHERE COLLECTEDBY = :1
             ORDER BY MOBID DESC",
            &[&agent_name],
        )
        .unwrap();
    let mut collection_list = Vec::new();
    for row_result in rows {
        let row = row_result.unwrap();

        let model = GetCollectionModel {
            party_mast_id: row.get(0).unwrap(),
            party_id: row.get(1).unwrap(),
            group_no: row.get(2).unwrap(),
            amount: row.get(3).unwrap(),
            collection_type: row.get(4).unwrap(),
            doc_id: row.get(5).unwrap(),
        };

        collection_list.push(model);
    }
    return Json(collection_list);
}
