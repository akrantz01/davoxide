use axum::{
    headers::{
        authorization::{Authorization, Basic},
        HeaderMapExt,
    },
    http::{header, HeaderMap, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

#[derive(Clone, Debug)]
pub struct UserInfo {
    pub username: String,
    pub display_name: String,
    pub groups: Vec<String>,
}

impl UserInfo {
    /// Extract user information from the request. The following methods are tried, in order:
    /// 1. SSO proxy headers (Remote-User, Remote-Name & Remote-Groups)
    /// 2. Basic authentication
    fn from_request<B>(req: &Request<B>) -> Result<UserInfo, Response> {
        let headers = req.headers();

        // Try proxy auth first
        if headers.contains_key("remote-user")
            && headers.contains_key("remote-name")
            && headers.contains_key("remote-groups")
        {
            let username = string_from_header(req.headers(), "remote-user")?;
            let display_name = string_from_header(req.headers(), "remote-name")?;
            let groups = string_from_header(req.headers(), "remote-groups")?
                .split(",")
                .map(str::trim)
                .map(ToOwned::to_owned)
                .collect();

            Ok(UserInfo {
                username,
                display_name,
                groups,
            })

        // Fallback to basic auth
        } else if headers.contains_key("authorization") {
            if let Some(credentials) = req.headers().typed_get::<Authorization<Basic>>() {
                Ok(UserInfo {
                    username: credentials.username().to_owned(),
                    display_name: String::new(),
                    groups: vec![],
                })
            } else {
                Err(unauthorized())
            }

        // No credentials found
        } else {
            Err(unauthorized())
        }
    }
}

pub async fn middleware<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, Response> {
    let user_info = UserInfo::from_request(&req)?;
    req.extensions_mut().insert(user_info);

    Ok(next.run(req).await)
}

fn string_from_header(headers: &HeaderMap, name: &str) -> Result<String, Response> {
    let value = headers
        .get(name)
        .ok_or_else(unauthorized)?
        .to_str()
        .map_err(|_| unauthorized())?
        .to_owned();
    Ok(value)
}

fn unauthorized() -> Response {
    (
        StatusCode::UNAUTHORIZED,
        [(header::WWW_AUTHENTICATE, "Basic realm=\"davoxide\"")],
        "unauthorized",
    )
        .into_response()
}
