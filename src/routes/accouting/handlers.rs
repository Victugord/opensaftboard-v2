use crate::config::{Taxonomies, TaxonomyType};
use crate::support::ApiContext;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use tokio::time::Instant;

#[derive(serde::Serialize)]
struct Account {
    id: i64,
}

pub async fn get_sales(state: State<ApiContext>) -> impl IntoResponse {
    let m_taxonomies = Taxonomies::new(TaxonomyType::Micro);

    let sales_taxonomies = m_taxonomies.get_by_dr("Vendas e serviços prestados");

    let sales_codes: Vec<i32> = sales_taxonomies
        .into_iter()
        .filter_map(|t| t.taxonomy_code.parse::<i32>().ok()) // Parse to i32 and filter out any errors
        .collect();

    let query_start = Instant::now();
    let accounts = sqlx::query_as!(
        Account,
        "SELECT id FROM gla_accounts WHERE taxonomy_code = ANY($1) and year = 2024",
        sales_codes.as_slice()
    )
    .fetch_all(&state.db)
    .await
    .unwrap();

    println!("get_sales: {:?}", query_start.elapsed());

    Json(json!({"env":accounts}))
}
