use crate::{
    database::{Action, Permission, PermissionManager, User, UserManager},
    error::Error,
};
use async_graphql::{Context, Object, Result};
use sea_orm::DatabaseConnection;
use std::path::Path;

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

    async fn assign_permission_to_user(
        &self,
        ctx: &Context<'_>,
        user: String,
        path: String,
        action: Action,
        affects_children: bool,
    ) -> Result<Permission> {
        let current_user = ctx.data::<User>()?;
        if !current_user.is_admin() {
            return Err(Error::InvalidPermissions.into());
        }

        let db = ctx.data::<DatabaseConnection>()?;

        // Ensure the user exists
        let user = UserManager::find_by_username(db, &user)
            .await?
            .ok_or(Error::NotFound)?;

        // Remove any trailing slashes
        let path = Path::new(&path)
            .components()
            .as_path()
            .display()
            .to_string();

        // Assign the permission
        let created =
            PermissionManager::assign_to_user(db, &user, path, action, affects_children).await?;
        Ok(created)
    }

    async fn remove_permission(&self, ctx: &Context<'_>, permission_id: i32) -> Result<bool> {
        let current_user = ctx.data::<User>()?;
        if !current_user.is_admin() {
            return Err(Error::InvalidPermissions.into());
        }

        let db = ctx.data::<DatabaseConnection>()?;
        PermissionManager::remove_permission(db, permission_id).await?;

        Ok(true)
    }
}
