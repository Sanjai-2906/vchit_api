use axum::Json;

use crate::models::CollectionModel;

pub async fn add_collection(Json(collection): Json<CollectionModel>) {
    
}
