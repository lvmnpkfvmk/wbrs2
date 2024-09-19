use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{Pool, Postgres};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use crate::{
    handler::{create_welcome, get_welcome},
    model::Welcome,
};
pub struct AppState {
    pub db: Pool<Postgres>,
    pub cache: HashMap<String, Welcome>,
}

pub fn create_router(app_state: Arc<Mutex<AppState>>) -> Router {
    Router::new()
        .route("/welcome", post(create_welcome))
        .route("/welcome/:id", get(get_welcome))
        .with_state(app_state)
}
