mod handlers;

use axum::Router;
use axum::routing::get;
use crate::support::ApiState;

pub fn get_handlers(ctx: ApiState) -> Router{
    Router::new().route("/sales", get(handlers::get_sales)).with_state(ctx)
}