use std::env;

use axum::{routing::get, Router};
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use todolister::models::app_state::{AppState, DbPool};

pub type Error = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let database_url: String = env::var("DATABASE_URL").expect("database url must be set in environment");
    let connection_manager: ConnectionManager<PgConnection> = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = Pool::builder().build(connection_manager).expect("Failed to create connection pool");

    let app_state = AppState {
        pool
    };

    let app = Router::new()
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
        )
        .route("/", get(|| async { "hello world" }))
        .with_state(app_state);

    let listener : TcpListener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}