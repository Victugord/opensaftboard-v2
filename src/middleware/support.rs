use crate::error::{Error,Result};
use crate::middleware::mw_auth::CtxExtError;

#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: i64,
}

impl Ctx {
    pub fn new(user_id: i64) -> Result<Self> {
        if user_id == 0 {
            Err(Error::CtxExt(CtxExtError::CtxCannotNewRootCtx))
        } else {
            Ok(Self {
                user_id,
            })
        }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}