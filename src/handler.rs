use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use chrono::DateTime;
use sqlx::types::chrono;
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::model::Welcome;
use crate::route::AppState;


pub async fn create_welcome(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(welcome): Json<Welcome>,
) -> Result<StatusCode, (StatusCode, String)> {
    let order_uid = welcome.order_uid.clone();
    let date_created = DateTime::parse_from_rfc3339(&welcome.date_created).unwrap();

    // Insert the welcome data into the database
    let result = sqlx::query(
        r#"
        INSERT INTO welcomes (
            order_uid, track_number, entry, locale, internal_signature,
            customer_id, delivery_service, shardkey, sm_id, date_created, oof_shard
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#
    )
    .bind(&welcome.order_uid)
    .bind(&welcome.track_number)
    .bind(&welcome.entry)
    .bind(&welcome.locale)
    .bind(&welcome.internal_signature)
    .bind(&welcome.customer_id)
    .bind(&welcome.delivery_service)
    .bind(&welcome.shardkey)
    .bind(&welcome.sm_id)
    .bind(date_created)
    .bind(&welcome.oof_shard)
    .execute(&state.lock().await.db)
    .await;

    match result {
        Ok(_) => {
            // If the insertion was successful, update the cache
            state.lock().await.cache.insert(order_uid, welcome);
            Ok(StatusCode::CREATED)
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to create welcome".to_string()))
        }
    }

}
