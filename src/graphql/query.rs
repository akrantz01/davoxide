use super::{
    fs::{self, Entry},
    outputs::DownloadUrlResult,
};
use crate::{config::Config, database::User, error::Error};
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

        // Build the full path
        let sub_path = path.map(PathBuf::from).unwrap_or_else(PathBuf::new);
        let path = fs::build_path(sub_path, &config.path)?;

        // Get the contents
        let entries = fs::list(path).await?;
        Ok(entries)
    }

    async fn download_url(&self, ctx: &Context<'_>, path: String) -> Result<DownloadUrlResult> {
        let config = ctx.data::<Arc<Config>>()?;

        // Only allow downloading files
        let path = fs::build_path(path.into(), &config.path)?;
        if !path.is_file() {
            return Err(Error::NotAFile.into());
        }

        let download_path = path.strip_prefix(&config.path).unwrap();
        Ok(DownloadUrlResult {
            url: format!("/dav/{}", download_path.display()),
        })
    }
}
