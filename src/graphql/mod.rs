use crate::{config::Config, database::User};
use async_graphql::{extensions, EmptySubscription, Schema as BaseSchema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::Extension;
use sqlx::PgPool;
use std::sync::Arc;

mod fs;
mod logging;
mod mutation;
mod outputs;
mod query;
mod tracing;

type Schema = BaseSchema<query::Query, mutation::Mutation, EmptySubscription>;

/// Build the schema for the GraphQL handler
pub fn schema(config: Arc<Config>, db: PgPool) -> Schema {
    Schema::build(query::Query, mutation::Mutation, EmptySubscription)
        .data(config)
        .data(db)
        .extension(extensions::Analyzer)
        .extension(tracing::Tracing)
        .extension(logging::Logger)
        .finish()
}

/// Handle graphql requests
pub async fn handler(
    Extension(user): Extension<User>,
    Extension(schema): Extension<Schema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    // Add the current user
    let mut req = req.into_inner();
    req.data.insert(user);

    schema.execute(req).await.into()
}
