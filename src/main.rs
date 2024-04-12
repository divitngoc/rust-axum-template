use axum::{Router, routing::get};
use tower_http::request_id::PropagateRequestIdLayer;
use crate::common::security;
use crate::common::utils::constants::{TRACING_ID_HEADER};
use crate::config::app_env::RUN_MODE;
use crate::middleware::layers;

mod config;
mod domains;
mod middleware;
mod common;

#[tokio::main]
async fn main() {
    if &*RUN_MODE == "local" {
        let env_file = format!(".env.{}", "local");
        dotenv::from_filename(env_file).ok();
    }

    config::init();

    let app_state = config::init_app_state().await;

    let public_routes = Router::new()
        .merge(domains::auth::handlers::routes())
        .with_state(app_state.clone())
        .route("/favicon.ico", get(domains::favicon::favicon::handle));

    let authenticated_routes = Router::new()
        .merge(domains::users::handlers::routes())
        .merge(domains::organisations::handlers::routes())
        .merge(domains::applications::handlers::routes())
        // middleware, the order of execution is from bottom to top for request and top to bottom for response
        .layer(PropagateRequestIdLayer::new(TRACING_ID_HEADER.clone()))
        .layer(layers::set_span())
        .layer(layers::set_tracing_id())
        .route_layer(axum::middleware::from_fn_with_state(app_state.clone(), security::jwt::authenticate))
        .with_state(app_state);

    // Combine routers, public routes remain accessible without authentication
    let app = Router::new()
        .merge(public_routes)
        .merge(authenticated_routes);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Unable to bind to port");

    tracing::info!("\n------------------------------------------\n      listening on {}\n------------------------------------------",
        listener.local_addr()
                .unwrap()
    );

    axum::serve(listener, app.into_make_service()).await.unwrap();
}