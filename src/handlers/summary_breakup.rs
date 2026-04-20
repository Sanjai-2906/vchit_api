use axum::Json;

use crate::models::{Mode, SummaryBreakupModel};

use crate::state::COLLECTIONS;

pub async fn summary_breakup() -> Json<SummaryBreakupModel> {

    let collections = COLLECTIONS.lock().await;

    let mut total_amount = 0.0;
    let mut cash_amount = 0.0;
    let mut upi_amount = 0.0;
    let mut cheque_amount = 0.0;

    for item in collections.iter() {
        total_amount += item.amount;

        match item.mode {
            Mode::CASH => cash_amount += item.amount,
            Mode::UPI => upi_amount += item.amount,
            Mode::CHEQUE => cheque_amount += item.amount,
        }
    }

    let summary = SummaryBreakupModel {
        total_amount,
        cash_amount,
        upi_amount,
        cheque_amount,
    };

    Json(summary)
}
