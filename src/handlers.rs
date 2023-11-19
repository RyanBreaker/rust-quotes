use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Serialize, FromRow)]
pub struct Quote {
    id: Uuid,
    book: String,
    quote: String,
    inserted_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl Quote {
    fn new(book: String, quote: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
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

pub async fn read_quotes(State(pool): State<PgPool>) -> Result<Json<Vec<Quote>>, StatusCode> {
    let res = sqlx::query_as::<_, Quote>("SELECT * FROM quotes")
        .fetch_all(&pool)
        .await;

    match res {
        Ok(quotes) => Ok(Json(quotes)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_quotes(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateQuote>,
) -> StatusCode {
    let now = chrono::Utc::now();
    let res = sqlx::query(
        r#"
        UPDATE quotes
        SET book = $1, quote = $2, updated_at = $3
        WHERE id = $4
        "#,
    )
    .bind(payload.book)
    .bind(payload.quote)
    .bind(now)
    .bind(id)
    .execute(&pool)
    .await
    .map(|res| match res.rows_affected() {
        0 => StatusCode::NOT_FOUND,
        _ => StatusCode::OK,
    });

    match res {
        Ok(status) => status,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
