use async_graphql::{
    extensions::{
        Extension, ExtensionContext, ExtensionFactory, NextExecute, NextParseQuery, NextResolve,
        NextValidation, ResolveInfo,
    },
    futures_util::TryFutureExt,
    parser::types::ExecutableDocument,
    Response, ServerError, ServerResult, ValidationResult, Value, Variables,
};
use std::sync::Arc;
use tracing::{error, span, Instrument, Level, Span};

pub struct Tracing;

impl ExtensionFactory for Tracing {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(TracingExtension)
    }
}

struct TracingExtension;

#[async_trait::async_trait]
impl Extension for TracingExtension {
    async fn parse_query(
        &self,
        ctx: &ExtensionContext<'_>,
        query: &str,
        variables: &Variables,
        next: NextParseQuery<'_>,
    ) -> ServerResult<ExecutableDocument> {
        let span = span!(
            target: "davoxide::graphql",
            Level::INFO,
            "parse",
        );
        async move {
            let res = next.run(ctx, query, variables).await;
            if let Ok(doc) = &res {
                Span::current().record(
                    "source",
                    &ctx.stringify_execute_doc(doc, variables).as_str(),
                );
            }
            res
        }
        .instrument(span)
        .await
    }

    async fn validation(
        &self,
        ctx: &ExtensionContext<'_>,
        next: NextValidation<'_>,
    ) -> Result<ValidationResult, Vec<ServerError>> {
        let span = span!(
            target: "davoxide::graphql",
            Level::INFO,
            "validation"
        );
        next.run(ctx).instrument(span).await
    }

    async fn execute(
        &self,
        ctx: &ExtensionContext<'_>,
        operation_name: Option<&str>,
        next: NextExecute<'_>,
    ) -> Response {
        let span = span!(
            target: "davoxide::graphql",
            Level::INFO,
            "execute",
            operation = %operation_name.unwrap_or_default(),
        );
        next.run(ctx, operation_name).instrument(span).await
    }

    async fn resolve(
        &self,
        ctx: &ExtensionContext<'_>,
        info: ResolveInfo<'_>,
        next: NextResolve<'_>,
    ) -> ServerResult<Option<Value>> {
        let span = span!(
            target: "davoxide::graphql",
            Level::INFO,
            "field",
            path = %info.path_node,
            parent_type = %info.parent_type,
            return_type = %info.return_type,
        );
        next.run(ctx, info)
            .map_err(|err| {
                error!(
                    target: "davoxide::graphql",
                    error = %err.message,
                    "error"
                );
                err
            })
            .instrument(span)
            .await
    }
}
