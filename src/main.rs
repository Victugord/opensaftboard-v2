mod routes;
mod config;
mod support;
mod middleware;
mod error;

use std::sync::Arc;
use aide::axum::ApiRouter;
use aide::openapi::{OpenApi, Tag};
use aide::transform::TransformOpenApi;
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
use axum::{Extension, Router};
use crate::middleware::mw_auth::{mw_ctx_require, mw_ctx_resolver};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    aide::gen::on_error(|error| {
        println!("{error}");
    });

    aide::gen::extract_schemas(true);


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
    // Create the application router with defined routes
    let mut api = OpenApi::default();
    let api_router = ApiRouter::new()
        .nest_service("/accounting", routes::accounting_service(ctx.clone()))
        .route_layer(axum::middleware::from_fn(mw_ctx_require));

    let app = ApiRouter::new()
        .nest_service("/api", api_router)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(axum::middleware::from_fn_with_state(ctx.clone(), mw_ctx_resolver))
        .finish_api_with(&mut api, api_docs)
        .layer(Extension(Arc::new(api)));

    // Start the server and run it asynchronously
    run_server(app).await;
}

fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("Aide axum Open API")
        .summary("An example Todo application")
        .description("Hello")
        .tag(Tag {
            name: "todo".into(),
            description: Some("Todo Management".into()),
            ..Default::default()
        })
        .security_scheme(
            "ApiKey",
            aide::openapi::SecurityScheme::ApiKey {
                location: aide::openapi::ApiKeyLocation::Header,
                name: "X-Auth-Key".into(),
                description: Some("A key that is ignored.".into()),
                extensions: Default::default(),
            },
        )
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
