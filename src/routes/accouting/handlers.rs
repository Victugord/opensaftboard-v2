
use crate::config::{Taxonomies, TaxonomyType};
use crate::support::ApiState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use tokio::time::Instant;
use tracing::debug;

#[derive(serde::Serialize)]
struct Account {
    id: i64,
}

#[derive(serde::Serialize)]
struct Tax {
    pub id: i32,
    pub hash: String,
}

#[derive(Debug,serde::Serialize)]
pub struct GleLine {
    pub id: i64,                          // bigint in SQL -> i64 in Rust
    pub account_id: String,               // text in SQL -> String in Rust
    pub description: String,              // text in SQL -> String in Rust
    pub transaction_type: String,         // text in SQL -> String in Rust
    pub ammount: f64,                     // double precision -> f64
    pub mov_type: String,                    // text in SQL -> String in Rust
    pub year: i32,                        // integer in SQL -> i32 in Rust
    pub month: i32,                       // integer in SQL -> i32 in Rust (default: 0)
    pub period: i32,                      // integer in SQL -> i32 in Rust (default: 0)
}


pub async fn get_sales(state: State<ApiState>) -> impl IntoResponse {

    let m_taxonomies = Taxonomies::new(TaxonomyType::Micro);

    let _ = m_taxonomies.get_debits_by_dr("Clientes",None);
    let start = Instant::now();

    let lines = sqlx::query_as!(GleLine, "
            SELECT id,
                   account_id,
                   description,
                   transaction_type,
                   ammount,
                   type as mov_type,
                   year,
                   month,
                   period
            FROM gle_lines WHERE consumer_id = 177 AND transaction_type = $1 ", "A")
        .fetch_all(&state.db).await;

    let end = Instant::now();
    let elapsed = (end - start).as_micros();
    debug!("{:<12} - get_sales - {end:?}", "ROUTE");
    debug!("{:<12} - get_sales - {start:?}", "ROUTE");
    debug!("{:<12} - get_sales - {elapsed:?}", "ROUTE");

    let valid_lines = match lines {
        Ok(lines) => lines,
        Err(e) => {
            debug!("{:<12} - get_sales - {e:?}", "ERROR");
            return Json(json!({"error":e.to_string()}))
        }
    };
    Json(json!(valid_lines))
}