use crate::{
    database::User,
    error::{Error, Result},
};
use axum::{
    headers::{
        authorization::{Authorization, Basic},
        HeaderMap, HeaderMapExt,
    },
    http::Request,
    middleware::Next,
    response::Response,
};
use sqlx::PgPool;

/// Check that the user is authenticated for every request
pub async fn middleware<B>(mut req: Request<B>, next: Next<B>) -> Result<Response> {
    // Get the user's information
    let user = load_user(&req).await?;
    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}

/// Extract user information from the request. The following methods are tried, in order:
/// 1. SSO proxy headers (Remote-User & Remote-Name)
/// 2. Basic authentication
async fn load_user<B>(req: &Request<B>) -> Result<User> {
    let headers = req.headers();
    let db = req.extensions().get::<PgPool>().unwrap();

    // Try proxy auth first
    if headers.contains_key("remote-user") && headers.contains_key("remote-name") {
        let username = string_from_header(req.headers(), "remote-user")?;
        let display_name = string_from_header(req.headers(), "remote-name")?;

        let user = User::create_if_not_exists(db, &username, &display_name).await?;
        Ok(user)

        // Fallback to basic auth
    } else if headers.contains_key("authorization") {
        let credentials = headers
            .typed_get::<Authorization<Basic>>()
            .ok_or(Error::Unauthorized)?;

        let user = User::get(db, &credentials.username())
            .await?
            .ok_or(Error::Unauthorized)?;

        if user.access_token_valid(&credentials.password()) {
            Ok(user)
        } else {
            Err(Error::Unauthorized)
        }

        // No credentials found
    } else {
        Err(Error::Unauthorized)
    }
}

fn string_from_header(headers: &HeaderMap, name: &str) -> Result<String> {
    let value = headers
        .get(name)
        .ok_or(Error::Unauthorized)?
        .to_str()
        .map_err(|_| Error::Unauthorized)?
        .to_owned();
    Ok(value)
}
