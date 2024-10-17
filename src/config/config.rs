use serde::Serialize;

#[derive(clap::Parser,Serialize)]
pub struct AppEnv {
    /// The connection URL for the Postgres database this application should use.
    #[clap(long, env)]
    pub database_url: String,

    #[clap(long, env)]
    pub jwt_secret: String,
}