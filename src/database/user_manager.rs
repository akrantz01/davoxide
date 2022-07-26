use super::{
    entities::{prelude::*, user},
    Action,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sea_orm::{prelude::*, sea_query::OnConflict, ActiveValue, DatabaseConnection, DbErr};
use uuid::Uuid;

pub struct UserManager;

impl UserManager {
    /// Find a user by their username
    pub async fn find_by_username(
        db: &DatabaseConnection,
        username: &str,
    ) -> Result<Option<user::Model>, DbErr> {
        User::find()
            .filter(user::Column::Username.eq(username))
            .one(db)
            .await
    }

    /// Re-generate a user's access token
    pub async fn regenerate_access_token(
        db: &DatabaseConnection,
        user: &user::Model,
    ) -> Result<String, DbErr> {
        let token = Uuid::new_v4().to_string();

        // Hash the password
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let hash = argon2
            .hash_password(token.as_bytes(), &salt)
            .unwrap()
            .to_string();

        // Update the user
        let mut user = user.as_active_model();
        user.access_token = ActiveValue::Set(Some(hash));
        user.update(db).await?;

        Ok(token)
    }

    /// Remove a user's access token
    pub async fn remove_access_token(
        db: &DatabaseConnection,
        user: &user::Model,
    ) -> Result<(), DbErr> {
        let mut user = user.as_active_model();
        user.access_token = ActiveValue::Set(None);

        user.update(db).await?;
        Ok(())
    }

    /// Check that a user's access token is valid
    pub async fn verify_access_token(
        db: &DatabaseConnection,
        username: &str,
        token: &str,
    ) -> Result<Option<user::Model>, DbErr> {
        let user = match UserManager::find_by_username(db, username).await? {
            Some(u) => u,
            None => return Ok(None),
        };

        // Get the access token
        let hash = match &user.access_token {
            Some(hash) => PasswordHash::new(hash).unwrap(),
            None => return Ok(None),
        };

        let argon = Argon2::default();
        match argon.verify_password(token.as_bytes(), &hash) {
            Ok(_) => Ok(Some(user)),
            Err(_) => Ok(None),
        }
    }

    /// Create the user if they do not already exist
    pub async fn create_if_not_exists(
        db: &DatabaseConnection,
        username: String,
        name: String,
    ) -> Result<user::Model, DbErr> {
        let input = user::ActiveModel {
            username: ActiveValue::Set(username),
            name: ActiveValue::Set(name),
            access_token: ActiveValue::NotSet,
            default_access: ActiveValue::NotSet,
        };

        User::insert(input)
            .on_conflict(
                OnConflict::column(user::Column::Username)
                    .update_column(user::Column::Name)
                    .to_owned(),
            )
            .exec_with_returning(db)
            .await
    }

    /// Get a list of all the users
    pub async fn list(db: &DatabaseConnection) -> Result<Vec<user::Model>, DbErr> {
        User::find().all(db).await
    }

    /// Change the default action for the given user
    pub async fn change_default_action(
        db: &DatabaseConnection,
        user: &user::Model,
        action: Action,
    ) -> Result<user::Model, DbErr> {
        let mut user = user.as_active_model();
        user.default_access = ActiveValue::Set(action);

        user.update(db).await
    }

    /// Delete a user
    pub async fn delete(db: &DatabaseConnection, user: String) -> Result<(), DbErr> {
        User::delete_by_id(user).exec(db).await?;
        Ok(())
    }
}
