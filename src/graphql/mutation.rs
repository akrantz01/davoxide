use crate::{
    database::{Action, Permission, User},
    error::Error,
};
use async_graphql::{Context, Error as GraphQLError, Object, Result};
use sqlx::PgPool;
use std::path::Path;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn regenerate_access_token(&self, ctx: &Context<'_>) -> Result<String> {
        let user = ctx.data::<User>()?;
        let db = ctx.data::<PgPool>()?;

        let token = user.regenerate_access_token(db).await?;
        Ok(token)
    }

    async fn revoke_access_token(&self, ctx: &Context<'_>) -> Result<bool> {
        let user = ctx.data::<User>()?;
        let db = ctx.data::<PgPool>()?;

        user.revoke_access_token(db).await?;

        Ok(true)
    }

    async fn update_default_permission(
        &self,
        ctx: &Context<'_>,
        user: String,
        action: Action,
    ) -> Result<User> {
        let current_user = ctx.data::<User>()?;
        if !current_user.is_admin() {
            return Err(Error::InvalidPermissions.into());
        }

        let db = ctx.data::<PgPool>()?;

        // Update the user
        let mut user = User::get(db, &user).await?.ok_or(Error::NotFound)?;
        user.set_default_action(db, action).await?;

        Ok(user)
    }

    async fn delete_user(&self, ctx: &Context<'_>, user: String) -> Result<bool> {
        let current_user = ctx.data::<User>()?;
        if !current_user.is_admin() {
            return Err(Error::InvalidPermissions.into());
        } else if current_user.username == user {
            return Err(GraphQLError::new("cannot delete yourself"));
        }

        let db = ctx.data::<PgPool>()?;
        User::delete(db, &user).await?;

        Ok(true)
    }

    async fn assign_permission_to_user(
        &self,
        ctx: &Context<'_>,
        user: String,
        path: String,
        action: Action,
        affects_children: bool,
    ) -> Result<Permission> {
        let db = ctx.data::<PgPool>()?;

        let current_user = ctx.data::<User>()?;
        if !current_user.is_admin() {
            return Err(Error::InvalidPermissions.into());
        }

        // Remove any trailing slashes
        let path = Path::new(&path)
            .components()
            .as_path()
            .display()
            .to_string();

        // Assign the permission
        let user = User::get(db, &user).await?.ok_or(Error::NotFound)?;
        let permission = user
            .assign_permission(db, &path, action, affects_children)
            .await?;
        Ok(permission)
    }

    async fn remove_permission(&self, ctx: &Context<'_>, permission_id: i32) -> Result<bool> {
        let current_user = ctx.data::<User>()?;
        if !current_user.is_admin() {
            return Err(Error::InvalidPermissions.into());
        }

        let db = ctx.data::<PgPool>()?;
        Permission::delete(db, permission_id).await?;

        Ok(true)
    }
}
