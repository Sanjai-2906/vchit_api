use axum::Json;
use axum::extract::Path;
use oracle::Connection;

// use crate::models::CollectionModel;
use crate::models::GetCollectionModel;

// use crate::state::COLLECTIONS;

pub async fn get_collections(Path(agent_name): Path<String>) -> Json<Vec<GetCollectionModel>> {
    println!("Collection - Agent Name: {}",agent_name);
    let conn = Connection::connect("vvcpl", "log", "velcloud.in:1521/XE").unwrap();

    let rows = conn
        .query(
            "SELECT PARTYMASTID, PARTYID, GROUPNO, 
             AMOUNT, TYPE  FROM MOB WHERE COLLECTEDBY = :1
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
        };

        collection_list.push(model);
    }
    // let collections = COLLECTIONS.lock().await;
    // let collection_list = collections.to_vec();

    // return Json(collection_list);
    return Json(collection_list);
}
