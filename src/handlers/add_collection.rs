use axum::Json;

use crate::models::CollectionModel;
use crate::state::COLLECTIONS;

pub async fn add_collection(Json(collection): Json<CollectionModel>) {

    let mut collections = COLLECTIONS.lock().await;
    collections.push(collection);
}
