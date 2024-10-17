use std::sync::Arc;
use sqlx::PgPool;
use crate::config::AppEnv;

#[derive(Clone)]
pub struct ApiState {
    pub config: Arc<AppEnv>,
    pub db: PgPool,
}
impl ApiState {
    pub fn new(db:PgPool, config:AppEnv) -> Self{
        Self{
            db,
            config: Arc::new(config)
        }
    }
}