use crate::{
    database::{Action, User},
    error::Result,
    security::{check_permissions, sanitize_path},
};
use axum::{body::Body, http::Request, response::Response, Extension};
use dav_server::{body::Body as DavBody, localfs::LocalFs, memls::MemLs, DavHandler, DavMethod};
use percent_encoding::percent_decode_str;
use sqlx::PgPool;
use std::path::Path;

/// Build the file system interface the server should use
pub fn filesystem(base: &Path) -> DavHandler {
    let fs = LocalFs::new(base, false, false, false);
    let ls = MemLs::new();

    DavHandler::builder()
        .strip_prefix("/dav")
        .filesystem(fs)
        .locksystem(ls)
        .build_handler()
}

/// Handle WebDAV requests
pub async fn handler(
    Extension(webdav): Extension<DavHandler>,
    Extension(user): Extension<User>,
    Extension(db): Extension<PgPool>,
    req: Request<Body>,
) -> Result<Response<DavBody>> {
    let raw_path = req.uri().path().strip_prefix("/dav").unwrap();
    let decoded_path = percent_decode_str(raw_path)
        .decode_utf8_lossy()
        .into_owned();
    let path = sanitize_path(decoded_path.into())?;

    // Check the user's permissions
    let required = required_permission(req.method().try_into()?);
    check_permissions(&db, &user, &path, required).await?;

    Ok(webdav.handle(req).await)
}

/// Get the required permission for the requested method
fn required_permission(method: DavMethod) -> Action {
    use DavMethod::*;

    match method {
        Get | Head | Options | PropFind => Action::Read,
        Copy | Delete | MkCol | Move | Patch | PropPatch | Put => Action::Modify,
        Lock | Unlock => Action::Admin,
    }
}
