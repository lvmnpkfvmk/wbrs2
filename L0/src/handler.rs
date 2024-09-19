use crate::model::{Delivery, Item, Payment, Welcome};
use crate::route::AppState;
use axum::extract::Path;
use axum::{extract::State, http::StatusCode, Json};
use chrono::DateTime;
use sqlx::types::chrono;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn create_welcome(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(welcome): Json<Welcome>,
) -> Result<StatusCode, (StatusCode, String)> {
    let order_uid = welcome.order_uid.clone();

    // Insert the welcome data into the database
    let result = sqlx::query(
        r#"
        INSERT INTO welcomes (
            order_uid, track_number, entry, locale, internal_signature,
            customer_id, delivery_service, shardkey, sm_id, date_created, oof_shard
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
    )
    .bind(&welcome.order_uid)
    .bind(&welcome.track_number)
    .bind(&welcome.entry)
    .bind(&welcome.locale)
    .bind(&welcome.internal_signature)
    .bind(&welcome.customer_id)
    .bind(&welcome.delivery_service)
    .bind(&welcome.shardkey)
    .bind(welcome.sm_id)
    .bind(welcome.date_created)
    .bind(&welcome.oof_shard)
    .execute(&state.lock().await.db)
    .await;

    // Insert the delivery data
    let delivery_result = sqlx::query(
        r#"
        INSERT INTO deliveries (
            order_uid, name, phone, zip, city, address, region, email
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
    )
    .bind(&welcome.order_uid)
    .bind(&welcome.delivery.name)
    .bind(&welcome.delivery.phone)
    .bind(&welcome.delivery.zip)
    .bind(&welcome.delivery.city)
    .bind(&welcome.delivery.address)
    .bind(&welcome.delivery.region)
    .bind(&welcome.delivery.email)
    .execute(&state.lock().await.db)
    .await;

    if let Err(e) = delivery_result {
        eprintln!("Database error inserting delivery: {}", e);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create delivery".to_string(),
        ));
    }

    // Insert the payment data
    let payment_result = sqlx::query(
        r#"
        INSERT INTO payments (
            order_uid, transaction, request_id, currency, provider,
            amount, payment_dt, bank, delivery_cost, goods_total, custom_fee
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
    )
    .bind(&welcome.order_uid)
    .bind(&welcome.payment.transaction)
    .bind(&welcome.payment.request_id)
    .bind(&welcome.payment.currency)
    .bind(&welcome.payment.provider)
    .bind(welcome.payment.amount)
    .bind(welcome.payment.payment_dt)
    .bind(&welcome.payment.bank)
    .bind(welcome.payment.delivery_cost)
    .bind(welcome.payment.goods_total)
    .bind(welcome.payment.custom_fee)
    .execute(&state.lock().await.db)
    .await;

    if let Err(e) = payment_result {
        eprintln!("Database error inserting payment: {}", e);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create payment".to_string(),
        ));
    }

    // Insert the items data
    for item in &welcome.items {
        let item_result = sqlx::query(
            r#"
            INSERT INTO items (
                order_uid, chrt_id, track_number, price, rid, name,
                sale, size, total_price, nm_id, brand, status
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
        )
        .bind(&welcome.order_uid)
        .bind(item.chrt_id)
        .bind(&item.track_number)
        .bind(item.price)
        .bind(&item.rid)
        .bind(&item.name)
        .bind(item.sale)
        .bind(&item.size)
        .bind(item.total_price)
        .bind(item.nm_id)
        .bind(&item.brand)
        .bind(item.status)
        .execute(&state.lock().await.db)
        .await;

        if let Err(e) = item_result {
            eprintln!("Database error inserting item: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create item".to_string(),
            ));
        }
    }

    match result {
        Ok(_) => {
            // If the insertion was successful, update the cache
            state.lock().await.cache.insert(order_uid, welcome);
            Ok(StatusCode::CREATED)
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create welcome".to_string(),
            ))
        }
    }
}

pub async fn get_welcome(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(order_uid): Path<String>,
) -> Result<Json<Welcome>, (StatusCode, String)> {
    // First, check if the welcome data is in the cache
    if let Some(welcome) = state.lock().await.cache.get(&order_uid) {
        return Ok(Json(welcome.clone()));
    }

    let welcome = sqlx::query_as::<_, Welcome>("SELECT * FROM welcomes WHERE order_uid = $1")
        .bind(&order_uid)
        .fetch_optional(&state.lock().await.db)
        .await
        .map_err(|e| {
            eprintln!("Database error fetching welcome: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch welcome".to_string(),
            )
        })?;

    let delivery = sqlx::query_as::<_, Delivery>("SELECT * FROM deliveries WHERE order_uid = $1")
        .bind(&order_uid)
        .fetch_one(&state.lock().await.db)
        .await
        .map_err(|e| {
            eprintln!("Database error fetching delivery: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch delivery".to_string(),
            )
        })?;

    let payment = sqlx::query_as::<_, Payment>("SELECT * FROM payments WHERE order_uid = $1")
        .bind(&order_uid)
        .fetch_one(&state.lock().await.db)
        .await
        .map_err(|e| {
            eprintln!("Database error fetching payment: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch payment".to_string(),
            )
        })?;

    let items = sqlx::query_as::<_, Item>("SELECT * FROM items WHERE order_uid = $1")
        .bind(&order_uid)
        .fetch_all(&state.lock().await.db)
        .await
        .map_err(|e| {
            eprintln!("Database error fetching items: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch items".to_string(),
            )
        })?;

    // Construct Welcome struct
    let welcome = match welcome {
        Some(w) => Welcome {
            order_uid: order_uid.clone(),
            track_number: w.track_number,
            entry: w.entry,
            delivery,
            payment,
            items,
            locale: w.locale,
            internal_signature: w.internal_signature,
            customer_id: w.customer_id,
            delivery_service: w.delivery_service,
            shardkey: w.shardkey,
            sm_id: w.sm_id,
            date_created: w.date_created,
            oof_shard: w.oof_shard,
        },
        _ => return Err((StatusCode::NOT_FOUND, "Welcome not found".to_string())),
    };

    // Update the cache with the fetched data
    state.lock().await.cache.insert(order_uid, welcome.clone());

    Ok(Json(welcome))
}
