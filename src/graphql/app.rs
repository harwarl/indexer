use async_graphql::http::GraphiQLSource;
use async_graphql_axum::GraphQL;
use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};
use sqlx::PgPool;

use crate::graphql::schema::{AppSchema, build_schema};

/// Creates and configures the application router.
///
/// Builds the GraphQL schema using the provided database pool and
/// returns a fully configured Axum router.
pub async fn create_app(db: PgPool) -> Router {
    // Build the Schema
    let schema: AppSchema = build_schema(db.clone());

    // Create the router
    create_router(schema).await
}

/// Creates the application's routing table.
///
/// Registers the GraphQL endpoint and associated services.
pub async fn create_router(schema: AppSchema) -> Router {
    Router::new().route("/graphql", get(graphql).post_service(GraphQL::new(schema)))
}

/// Serves the GraphiQL web interface.
///
/// This endpoint provides an interactive UI for exploring and
/// testing the GraphQL API.
async fn graphql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
