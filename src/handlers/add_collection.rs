use chrono::Local;

use crate::{
    AppState,
    get_connection::get_connection,
    models::{CollectionModel, SmsParams},
};
use axum::{Json, extract::State, http::StatusCode};
use reqwest::Client;

pub async fn add_collection(
    State(state): State<AppState>,
    Json(collection): Json<CollectionModel>,
) -> Result<StatusCode,(StatusCode,String)> {
    println!("User Data: {:?}", collection);
    let conn = get_connection(&state.pool).await?;
    let doc_id: i64 = conn
        .query_row_as("SELECT MOB_DOCID_SEQ.NEXTVAL FROM MOB", &[])
        .map_err(|err| {
            eprintln!("Failed to get DOCID: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to generate receipt number".to_string(),
            )
        })?;
    conn.execute(
        "INSERT INTO MOB (
            DOCDATE,
            COLLECTEDBY,
            GROUPNO,
            PARTYMASTID,
            PARTYID,
            MOBILE,
            AMOUNT,
            TYPE,
            DUEAMOUNT,
            BAL,
            CHEQUEDATE,
            CHEQUENO,
            DOCID
        )
        VALUES (
            TO_DATE(:1, 'YYYY-MM-DD'),
            :2,
            :3,
            :4,
            :5,
            :6,
            :7,
            :8,
            :9,
            :10,
            TO_DATE(:11, 'YYYY-MM-DD'),
            :12,
            :13
         )",
        &[
            &collection.doc_date.to_string(),
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
            &doc_id,
        ],
    )
    .map_err(|err| {
        println!("Database Insert Error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to insert collection".to_string(),
        )
    })?;

    conn.commit().map_err(|err| {
        println!("Database Commit Error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to commit transaction".to_string(),
        )
    })?;
    println!("Doc id: {}", doc_id);
    // Build SMS
    let sms = format!(
        "Dear Mr. {}, received Rs.{} for Group {} on {}, Receipt No:{} -VALUMOORTHY VELAYUTHAM CHITS (P) LTD.",
        collection.party_id,
        collection.amount,
        collection.group_no,
        Local::now().format("%d-%m-%Y").to_string(),
        doc_id,
    );

    let mobile = collection
        .mobile
        .as_ref()
        .ok_or((StatusCode::CONFLICT, "Mobile number is Null".to_string()))?;

    let params = SmsParams {
        key: "cb2c3cee7073db699b9921ab5d738ce7".into(),
        route: "2".into(),
        sender: "VVCPLd".into(),
        number: mobile.clone(),
        templateid: "1607100000000102140".into(),
        sms,
    };

    println!("{:?}", params);

    let client = Client::new();

    let response = match client
        .get("http://bulksms.velcloud.in/api/smsapi")
        .query(&params)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => {
            eprintln!("SMS Error: {:?}", err);
            return Ok(StatusCode::CREATED); // or return Err(...)
        }
    };

    println!("SMS Status: {}", response.status());

    let body = response.text().await.unwrap_or_default();
    println!("SMS Response: {}", body);

    Ok(StatusCode::CREATED)
}
