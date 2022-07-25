use crate::database::{
    entities::{permission, prelude::*, user},
    Action,
};
use sea_orm::{prelude::*, ActiveValue, DatabaseConnection, DbErr};

pub struct PermissionManager;

impl PermissionManager {
    /// Assign a new permission to a user
    pub async fn assign_to_user(
        db: &DatabaseConnection,
        user: &user::Model,
        path: String,
        action: Action,
        affects_children: bool,
    ) -> Result<permission::Model, DbErr> {
        let permission = permission::ActiveModel {
            id: ActiveValue::NotSet,
            applies_to: ActiveValue::Set(user.username.clone()),
            path: ActiveValue::Set(path),
            action: ActiveValue::Set(action),
            affects_children: ActiveValue::Set(affects_children),
        };

        Permission::insert(permission).exec_with_returning(db).await
    }

    /// Remove a permission
    pub async fn remove_permission(db: &DatabaseConnection, id: i32) -> Result<(), DbErr> {
        Permission::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
