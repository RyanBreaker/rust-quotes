use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize)]
pub struct Quote {
    id: uuid::Uuid,
    book: String,
    quote: String,
    inserted_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl Quote {
    fn new(book: String, quote: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            book,
            quote,
            inserted_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateQuote {
    book: String,
    quote: String,
}

pub async fn health() -> StatusCode {
    StatusCode::OK
}

pub async fn create_quote(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateQuote>,
) -> Result<(StatusCode, Json<Quote>), StatusCode> {
    let quote = Quote::new(payload.book, payload.quote);
    let res = sqlx::query(
        r#"
        INSERT INTO quotes (id, book, quote, inserted_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(quote.id)
    .bind(&quote.book)
    .bind(&quote.quote)
    .bind(quote.inserted_at)
    .bind(quote.updated_at)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => Ok((StatusCode::CREATED, Json(quote))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
