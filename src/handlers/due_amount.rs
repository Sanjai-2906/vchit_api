use axum::Json;
use rand::prelude::*;

use crate::models::{GetDueModel,DueModel};

pub async fn get_due_amount(Json(data): Json<GetDueModel>) -> Json<DueModel>{
    let mut rng1 = rand::rng();
    let mut rng2 = rand::rng();

    let mut pend_due: Vec<i32> = (200..500).collect();
    let mut current_due: Vec<i32> = (250..700).collect();
    pend_due.shuffle(&mut rng1);
    current_due.shuffle(&mut rng2);

    let res =  DueModel{
        // pending_due: 500.00,
        pending_due: *pend_due.choose(&mut rng1).unwrap() as f32,
        // current_due: 450.00
        current_due: *current_due.choose(&mut rng2).unwrap() as f32,
    };

    return Json(res);
}