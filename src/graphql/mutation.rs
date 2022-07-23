use crate::database::{User, UserManager};
use async_graphql::{Context, Object, Result};
use sea_orm::DatabaseConnection;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn regenerate_access_token(&self, ctx: &Context<'_>) -> Result<String> {
        let user = ctx.data::<User>()?;
        let db = ctx.data::<DatabaseConnection>()?;

        let token = UserManager::regenerate_access_token(db, user).await?;
        Ok(token)
    }

    async fn revoke_access_token(&self, ctx: &Context<'_>) -> Result<bool> {
        let user = ctx.data::<User>()?;
        let db = ctx.data::<DatabaseConnection>()?;

        UserManager::remove_access_token(db, user).await?;
        Ok(true)
    }
}
