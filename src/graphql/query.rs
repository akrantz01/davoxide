use super::{
    fs::{self, Entry},
    outputs::DownloadUrlResult,
};
use crate::{
    config::Config,
    database::{Action, User},
    error::Error,
    security::{check_permissions, sanitize_path},
};
use async_graphql::{Context, Object, Result};
use sqlx::PgPool;
use std::{path::PathBuf, sync::Arc};

pub struct Query;

#[Object]
impl Query {
    async fn me(&self, ctx: &Context<'_>) -> Result<User> {
        ctx.data::<User>().map(Clone::clone)
    }

    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let current_user = ctx.data::<User>()?;
        if !current_user.is_admin() {
            return Err(Error::InvalidPermissions.into());
        }

        let db = ctx.data::<PgPool>()?;
        let users = User::list(db).await?;

        Ok(users)
    }

    async fn user(&self, ctx: &Context<'_>, username: String) -> Result<User> {
        let current_user = ctx.data::<User>()?;
        if !current_user.is_admin() {
            return Err(Error::InvalidPermissions.into());
        }

        let db = ctx.data::<PgPool>()?;
        let user = User::get(db, &username).await?.ok_or(Error::NotFound)?;

        Ok(user)
    }

    async fn list_directory(&self, ctx: &Context<'_>, path: Option<String>) -> Result<Vec<Entry>> {
        let config = ctx.data::<Arc<Config>>()?;
        let db = ctx.data::<PgPool>()?;
        let user = ctx.data::<User>()?;

        let sub_path = path.map(PathBuf::from).unwrap_or_else(PathBuf::new);
        let sanitized = sanitize_path(sub_path)?;

        // Check if the user has the necessary permissions
        check_permissions(db, user, &sanitized, Action::Read).await?;

        // Get the contents
        let path = config.path.join(sanitized);
        let entries = fs::list(path).await?;

        Ok(entries)
    }

    async fn download_url(&self, ctx: &Context<'_>, path: String) -> Result<DownloadUrlResult> {
        let config = ctx.data::<Arc<Config>>()?;
        let db = ctx.data::<PgPool>()?;
        let user = ctx.data::<User>()?;

        // Check if the user has the necessary permissions
        let sanitized = sanitize_path(path.into())?;
        check_permissions(db, user, &sanitized, Action::Read).await?;

        // Only allow downloading files
        let path = config.path.join(&sanitized);
        if !path.is_file() {
            return Err(Error::NotAFile.into());
        }

        Ok(DownloadUrlResult {
            url: format!("/dav/{}", sanitized.display()),
        })
    }
}
