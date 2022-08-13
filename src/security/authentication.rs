use crate::{
    database::User,
    error::{Error, Result},
};
use axum::{
    headers::{
        authorization::{Authorization, Basic},
        HeaderMapExt,
    },
    http::Request,
    middleware::Next,
    response::Response,
};
use sqlx::PgPool;
use tracing::info;

/// Check that there is an authenticated user
pub async fn ensure_authenticated<B>(req: Request<B>, next: Next<B>) -> Result<Response> {
    if let Some(user) = req.extensions().get::<User>() {
        info!(user = %user.username, "authenticated");
        Ok(next.run(req).await)
    } else {
        Err(Error::Unauthorized)
    }
}

#[async_trait::async_trait]
pub trait Extract {
    async fn extract<B>(req: &Request<B>) -> Option<User>
    where
        B: Sync;
}

/// Attempt to extract a user from the request.
/// NOTE: this does not fail if the user could not be loaded
pub async fn extract<B, E>(mut req: Request<B>, next: Next<B>) -> Result<Response>
where
    B: Sync,
    E: Extract,
{
    if let Some(user) = E::extract(&req).await {
        req.extensions_mut().insert(user);
    }

    Ok(next.run(req).await)
}

/// Extract the user from SSO proxy headers (Remote-User & Remote-Name)
pub struct SSOAuth;

#[async_trait::async_trait]
impl Extract for SSOAuth {
    async fn extract<B>(req: &Request<B>) -> Option<User>
    where
        B: Sync,
    {
        let headers = req.headers();
        let db = req.extensions().get::<PgPool>().unwrap();

        let username = headers.get("remote-user")?.to_str().ok()?;
        let display_name = headers.get("remote-name")?.to_str().ok()?;

        User::create_if_not_exists(db, username, display_name)
            .await
            .ok()
    }
}

/// Extract the user from HTTP basic authentication
pub struct BasicAuth;

#[async_trait::async_trait]
impl Extract for BasicAuth {
    async fn extract<B>(req: &Request<B>) -> Option<User>
    where
        B: Sync,
    {
        let headers = req.headers();
        let db = req.extensions().get::<PgPool>().unwrap();

        let credentials = headers.typed_get::<Authorization<Basic>>()?;
        let user = User::get(db, credentials.username()).await.ok()??;

        match user.access_token_valid(credentials.password()) {
            true => Some(user),
            false => None,
        }
    }
}
