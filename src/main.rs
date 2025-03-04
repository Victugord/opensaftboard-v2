mod routes;
mod config;
mod support;
mod middleware;
mod error;

use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber::filter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::config::AppEnv;
use crate::support::ApiState;
use axum::Router;
use crate::middleware::mw_auth::{mw_ctx_require, mw_ctx_resolver};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();



    let app_env = AppEnv::parse();


    let filter = filter::Targets::new()
        .with_target("tower_http::trace::make_span", Level::DEBUG)
        .with_target("tower_http::trace::on_response", Level::TRACE)
        .with_target("tower_http::trace::on_request", Level::TRACE)
        .with_default(Level::DEBUG);

    let tracing_layer = tracing_subscriber::fmt::layer();
    // Initialize the tracing subscriber for logging
    tracing_subscriber::registry()
        .with(tracing_layer)
        .with(filter)
        .init();

    let db = PgPoolOptions::new()
        // The default connection limit for a Postgres server is 100 connections, minus 3 for superusers.
        // Since we're using the default superuser we don't have to worry about this too much,
        // although we should leave some connections available for manual access.
        //
        // If you're deploying your application with multiple replicas, then the total
        // across all replicas should not exceed the Postgres connection limit.
        .min_connections(3)
        .connect(&app_env.database_url)
        .await
        .expect("could not connect to database_url");


    let ctx = ApiState::new(db, app_env);
    let api_router = Router::new()
        .nest_service("/accounting", routes::accouting::accounting_routes(ctx.clone()))
        .route_layer(axum::middleware::from_fn(mw_ctx_require));
    let app = Router::new()
        .fallback_service(api_router)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(axum::middleware::from_fn_with_state(ctx.clone(), mw_ctx_resolver));

    // Start the server and run it asynchronously
    run_server(app).await;
}


/// Starts the server and listens for incoming connections
///
/// # Example
///
/// ```
/// let app = create_app();
/// run_server(app).await;
/// ```
async fn run_server(app: Router) {
    // Get the port from environment variable or use default 3000
    let port = std::env::var("PORT").unwrap_or_else(|_| String::from("3000"));
    let addr = format!("0.0.0.0:{}", port);

    // Bind the TCP listener to the specified address
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    
    println!("Server listening on {}", addr);

    // Serve the application using Axum
    axum::serve(listener, app).await.unwrap();
}
