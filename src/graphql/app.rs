use async_graphql::http::GraphiQLSource;
use async_graphql_axum::GraphQL;
use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};
use sqlx::PgPool;

use crate::graphql::schema::{AppSchema, build_schema};

pub async fn create_app(db: PgPool) -> Router {
    // Build the Schema
    let schema: AppSchema = build_schema(db.clone());

    // Create the router
    create_router(schema).await
}

pub async fn create_router(schema: AppSchema) -> Router {
    Router::new().route("/graphql", get(graphql).post_service(GraphQL::new(schema)))
}

async fn graphql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
