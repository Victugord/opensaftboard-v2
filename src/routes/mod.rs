mod accouting;

use axum::Router;
use accouting::*;
use crate::support::ApiState;

pub fn accounting_service(ctx: ApiState) -> Router {
    Router::new().nest_service("/", get_handlers(ctx))
}