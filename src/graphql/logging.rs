use async_graphql::{
    extensions::{
        Extension, ExtensionContext, ExtensionFactory, NextExecute, NextParseQuery, NextResolve,
        ResolveInfo,
    },
    parser::types::{ExecutableDocument, OperationType, Selection},
    Response, ServerResult, Value, Variables,
};
use std::{sync::Arc, time::Instant};
use tracing::{debug, info};

pub(crate) struct Logger;

impl ExtensionFactory for Logger {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(LoggerExtension)
    }
}

struct LoggerExtension;

#[async_trait::async_trait]
impl Extension for LoggerExtension {
    async fn parse_query(
        &self,
        ctx: &ExtensionContext<'_>,
        query: &str,
        variables: &Variables,
        next: NextParseQuery<'_>,
    ) -> ServerResult<ExecutableDocument> {
        let now = Instant::now();

        debug!(target: "davoxide::graphql", "started parsing request");
        let result = next.run(ctx, query, variables).await;
        debug!(target: "davoxide::graphql", latency = format_args!("{} ms", now.elapsed().as_millis()), "finished parsing request");

        let document = result?;
        let is_schema = document
            .operations
            .iter()
            .filter(|(_, operation)| operation.node.ty == OperationType::Query)
            .any(|(_, operation)| operation.node.selection_set.node.items.iter().any(|selection| matches!(&selection.node, Selection::Field(field) if field.node.name.node == "__schema")));
        if !is_schema {
            info!(
                target: "davoxide::graphql",
                document = ctx.stringify_execute_doc(&document, variables),
                "query"
            )
        }

        Ok(document)
    }

    async fn execute(
        &self,
        ctx: &ExtensionContext<'_>,
        operation_name: Option<&str>,
        next: NextExecute<'_>,
    ) -> Response {
        let now = Instant::now();

        debug!(target: "davoxide::graphql", "started executing operation");
        let response = next.run(ctx, operation_name).await;
        debug!(target: "davoxide::graphql", latency = format_args!("{} ms", now.elapsed().as_millis()), "finished executing operation");

        response
    }

    async fn resolve(
        &self,
        ctx: &ExtensionContext<'_>,
        info: ResolveInfo<'_>,
        next: NextResolve<'_>,
    ) -> ServerResult<Option<Value>> {
        let now = Instant::now();

        debug!(target: "davoxide::graphql", "started resolving field");
        let result = next.run(ctx, info).await;
        debug!(target: "davoxide::graphql", latency = format_args!("{} ms", now.elapsed().as_millis()), "finished resolving field");

        result
    }
}
