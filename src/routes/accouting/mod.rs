mod handlers;

use axum::Router;
use axum::routing::get;
use crate::support::ApiContext;

pub fn get_handlers(ctx:ApiContext) -> Router{
    Router::new().route("/sales", get(handlers::get_sales)).with_state(ctx)
}