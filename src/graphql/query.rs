use crate::{database::User, error::Error};
use async_graphql::{Context, Object, Result};
use sqlx::PgPool;

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
}
