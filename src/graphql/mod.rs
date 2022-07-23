use crate::database::User;
use async_graphql::{EmptySubscription, Schema as BaseSchema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::Extension;
use sea_orm::DatabaseConnection;

mod mutation;
mod query;

type Schema = BaseSchema<query::Query, mutation::Mutation, EmptySubscription>;

/// Build the schema for the GraphQL handler
pub fn schema(db: DatabaseConnection) -> Schema {
    Schema::build(query::Query, mutation::Mutation, EmptySubscription)
        .data(db)
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
