use serde_derive::{Deserialize, Serialize};
use sqlx::{types::chrono::{DateTime, FixedOffset, Utc}, FromRow, Type};
use chrono::{serde::ts_seconds, NaiveDateTime};

#[derive(Type, FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct Welcome {
    pub order_uid: String,
    pub track_number: String,
    pub entry: String,
    #[sqlx(skip)]
    pub delivery: Delivery,
    #[sqlx(skip)]
    pub payment: Payment,
    #[sqlx(skip)]
    pub items: Vec<Item>,
    pub locale: String,
    pub internal_signature: String,
    pub customer_id: String,
    pub delivery_service: String,
    pub shardkey: String,
    pub sm_id: i64,
    pub date_created: NaiveDateTime,
    pub oof_shard: String,
}

#[derive(Default, Type, FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct Delivery {
    pub name: String,
    pub phone: String,
    pub zip: String,
    pub city: String,
    pub address: String,
    pub region: String,
    pub email: String,
}

#[derive(Default, Type, FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct Payment {
    pub transaction: String,
    pub request_id: String,
    pub currency: String,
    pub provider: String,
    pub amount: i64,
    pub payment_dt: i64,
    pub bank: String,
    pub delivery_cost: i64,
    pub goods_total: i64,
    pub custom_fee: i64,
}

#[derive(Default, Type, FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    pub chrt_id: i64,
    pub track_number: String,
    pub price: i64,
    pub rid: String,
    pub name: String,
    pub sale: i64,
    pub size: String,
    pub total_price: i64,
    pub nm_id: i64,
    pub brand: String,
    pub status: i64,
}
