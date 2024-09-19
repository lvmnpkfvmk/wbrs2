use axum::http::{header::CONTENT_TYPE, Method};
use l0::route::{create_router, AppState};
use sqlx::postgres::PgPoolOptions;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let app = create_router(Arc::new(Mutex::new(AppState {
        db: pool.clone(),
        cache: HashMap::new(),
    })))
    .layer(cors);

    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
