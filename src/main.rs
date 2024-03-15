#![deny(clippy::all)]

use std::{env, time::Duration};

use axum::{extract::DefaultBodyLimit, routing::get, Router};
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use structs::common::DatabaseConfig;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod structs;

#[tokio::main]
async fn main() {
    // initialize tracing
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| {
                "rust_axum=debug,axum=debug,tower_http=debug,mongodb=debug".into()
            }),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_config = DatabaseConfig::new();
    let mut client_options = ClientOptions::parse(database_config.uri).await.unwrap();
    client_options.connect_timeout = database_config.connection_timeout;
    client_options.max_pool_size = database_config.max_pool_size;
    client_options.min_pool_size = database_config.min_pool_size;
    let client = Client::with_options(client_options).unwrap();
    let database_name = std::env::var("MONGO_DATABASE").expect("MONGO_DATABASE must be set");
    let database = client.database(&database_name);

    // build our application routes
    let app = Router::new()
        .route(
            "/code",
            get(handlers::code_handler::get_data_by_code)
                .post(handlers::code_handler::generate_code),
        )
        .layer(TraceLayer::new_for_http())
        .layer(DefaultBodyLimit::max(1024))
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .with_state(database);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
