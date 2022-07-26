use crate::{
    database::{User, UserManager},
    error::Error,
};
use async_graphql::{Context, Object, Result};
use sea_orm::DatabaseConnection;

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

        let db = ctx.data::<DatabaseConnection>()?;
        Ok(UserManager::list(db).await?)
    }
}
