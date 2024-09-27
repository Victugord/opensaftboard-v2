mod accouting;

use axum::handler::Handler;
use axum::Router;
use accouting::*;
use crate::support::ApiContext;

pub fn accounting_service(ctx:ApiContext) -> Router {
    Router::new().nest_service("/", get_handlers(ctx))
}