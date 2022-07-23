use super::{
    database::{User, UserManager},
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
use sea_orm::DatabaseConnection;

#[derive(Clone, Debug)]
pub struct UserInfo {
    pub username: String,
    pub display_name: String,
}

impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        UserInfo {
            username: user.username,
            display_name: user.name,
        }
    }
}

impl UserInfo {
    /// Extract user information from the request. The following methods are tried, in order:
    /// 1. SSO proxy headers (Remote-User, Remote-Name & Remote-Groups)
    /// 2. Basic authentication
    async fn from_request<B>(req: &Request<B>) -> Result<UserInfo> {
        let headers = req.headers();
        let db = req.extensions().get::<DatabaseConnection>().unwrap();

        // Try proxy auth first
        if headers.contains_key("remote-user")
            && headers.contains_key("remote-name")
            && headers.contains_key("remote-groups")
        {
            let username = string_from_header(req.headers(), "remote-user")?;
            let display_name = string_from_header(req.headers(), "remote-name")?;

            let user = UserManager::create_if_not_exists(db, username, display_name).await?;
            Ok(user.into())

        // Fallback to basic auth
        } else if headers.contains_key("authorization") {
            let credentials = headers
                .typed_get::<Authorization<Basic>>()
                .ok_or(Error::Unauthorized)?;

            UserManager::verify_access_token(db, credentials.username(), credentials.password())
                .await?
                .ok_or(Error::Unauthorized)
                .map(From::from)

        // No credentials found
        } else {
            Err(Error::Unauthorized)
        }
    }
}

/// Check that the user is authenticated for every request
pub async fn middleware<B>(mut req: Request<B>, next: Next<B>) -> Result<Response> {
    // Get the user's information
    let user_info = UserInfo::from_request(&req).await?;
    req.extensions_mut().insert(user_info);

    Ok(next.run(req).await)
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
