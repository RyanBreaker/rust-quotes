mod handlers;

use axum::routing::{delete, get, post, put};
use axum::Router;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let addr = format!("0.0.0.0:{port}");

    let db_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL env");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    let app = Router::new()
        .route("/", get(handlers::health))
        .route("/quotes", post(handlers::create_quote))
        .route("/quotes", get(handlers::read_quotes))
        .route("/quotes/:id", put(handlers::update_quote))
        .route("/quotes/:id", delete(handlers::delete_quote))
        .with_state(pool);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
