use axum::{
    routing::{get, post, patch},
    Router,
};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod web;
mod api;
mod services;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::new("info"))
        .init();

    // Load configuration
    let settings = config::settings::load().expect("Failled to load configuration");

    // Build the app with routes
    let app = Router::new()
        .nest("/x-nmos/registration/v1.3", api::is04::reigstration_router())
        .nest("/x-nmos/query/v1.3", api::is04::query_router())
        .nest("/x-nmos/connection/v1.1", api::is05::connection_router())
        .nest("/x-nmos/connection/v1.0", api::is05::system_router())
        .layer(tower_http::trace::TraceLayer::new_for_http())

    // Define listening address
    let addr = SocketAdd::from(([0, 0, 0, 0], settings.http_port));
    tracing::info!("NMOS Node listening on {addr}");

    // Start background services (e.g., heartbeat, mdns)
    tokio::spawn(services::registry_sync::start(settings.clone()));
    tokio::spawn(services::mdns::advertise(settings.clone()));

    // Serve the app
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
