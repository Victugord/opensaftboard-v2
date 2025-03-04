mod handlers;
use axum::{routing::get, Router};

use crate::support::ApiState;
pub fn accounting_routes(ctx: ApiState) -> Router {
    Router::new()
        .route("/sales", get(handlers::get_sales))
        .with_state(ctx)
}
