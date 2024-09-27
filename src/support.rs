use std::sync::Arc;
use sqlx::PgPool;
use crate::config::AppEnv;

#[derive(Clone)]
pub struct ApiContext {
    pub config: Arc<AppEnv>,
    pub db: PgPool,
}
impl ApiContext {
    pub fn new(db:PgPool, config:AppEnv) -> Self{
        Self{
            db,
            config: Arc::new(config)
        }
    }
}