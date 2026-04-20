use crate::models::CollectionModel;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

pub static COLLECTIONS: Lazy<Mutex<Vec<CollectionModel>>> = Lazy::new(|| Mutex::new(Vec::new()));
