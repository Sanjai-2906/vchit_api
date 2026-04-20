use axum::Json;

use crate::models::CollectionModel;

use crate::state::COLLECTIONS;

pub async fn get_collections() -> Json<Vec<CollectionModel>> {
    let collections = COLLECTIONS.lock().await;
    let collection_list = collections.to_vec();

    return Json(collection_list);
}