use axum::{routing::post, Router};
use sqlx::{Pool, Postgres};
use tokio::sync::Mutex;
use std::{collections::HashMap, sync::Arc};

use crate::{handler::create_welcome, model::Welcome};
pub struct AppState {
    pub db: Pool<Postgres>,
    pub cache: HashMap<String, Welcome>,
}

pub fn create_router(app_state: Arc<Mutex<AppState>>) -> Router {
    Router::new()
    .route("/welcome", post(create_welcome))
    .with_state(app_state)
}
