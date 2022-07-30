use super::fs::{self, Entry};
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
        let entries = fs::list(&config.path, sanitized).await?;

        Ok(entries)
    }
}
