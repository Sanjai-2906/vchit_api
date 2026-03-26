use axum::Json;

use crate::models::{GetDueModel,DueModel};

pub async fn get_due_amount(Json(data): Json<GetDueModel>) -> Json<DueModel>{
    println!("Get Due Model : {:?}",data);

    let res =  DueModel{
        pending_due: 500.00,
        current_due: 450.00
    };
    
    return Json(res);
}