use axum::{routing::get, Router};
use serde::Serialize;
use std::net::SocketAddr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(health))]
struct ApiDoc;

#[tokio::main]
async fn main() {
    // Create the Axum router
    let app = Router::new()
        .route("/health", get(health)) // Health check route
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi())); // Swagger UI route

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on http://{}", addr);

    // Start the Axum server (compatible with Hyper)
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// Health check endpoint
#[utoipa::path(get, path = "/health", responses((status = 200, description = "OK")))]
async fn health() -> axum::Json<HealthResponse> {
    axum::Json(HealthResponse {
        status: "ok".to_string(),
    })
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}
