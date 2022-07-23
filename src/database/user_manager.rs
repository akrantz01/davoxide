use super::entities::{prelude::*, user};
use sea_orm::{prelude::*, sea_query::OnConflict, ActiveValue, DatabaseConnection, DbErr};

pub struct UserManager;

impl UserManager {
    /// Find a user by their ID
    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id).one(db).await
    }

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

        // TODO: use hashes instead of plaintext
        if matches!(&user.access_token, Some(hash) if hash == token) {
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    /// Create the user if they do not already exist
    pub async fn create_if_not_exists(
        db: &DatabaseConnection,
        username: String,
        name: String,
    ) -> Result<user::Model, DbErr> {
        let input = user::ActiveModel {
            id: ActiveValue::NotSet,
            username: ActiveValue::Set(username),
            name: ActiveValue::Set(name),
            access_token: ActiveValue::NotSet,
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
}
