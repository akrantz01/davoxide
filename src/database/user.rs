use super::{permission::Permission, types::Action};
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use async_graphql::{ComplexObject, Context, FieldResult, SimpleObject};
use rand_core::OsRng;
use sqlx::{Error, PgPool, Result};
use uuid::Uuid;

/// An individual user with access to the application
#[derive(Clone, Debug, PartialEq, SimpleObject)]
#[graphql(complex)]
pub struct User {
    pub username: String,
    pub name: String,
    #[graphql(skip)]
    pub access_token: Option<String>,
    pub default_access: Action,
}

impl User {
    /// Get a list of all the users
    pub async fn list(db: &PgPool) -> Result<Vec<User>> {
        let mut conn = db.acquire().await?;
        sqlx::query_as!(User, "SELECT username, name, access_token, default_access as \"default_access: _\" FROM users")
            .fetch_all(&mut conn)
            .await
    }

    /// Create a user if they do not already exist
    pub async fn create_if_not_exists(db: &PgPool, username: &str, name: &str) -> Result<User> {
        let mut conn = db.acquire().await?;
        sqlx::query_as!(
            User,
            "INSERT INTO users (username, name) VALUES ($1, $2) \
            ON CONFLICT (username) DO UPDATE SET name = excluded.name \
            RETURNING username, name, access_token, default_access as \"default_access: _\"",
            username,
            name
        )
        .fetch_one(&mut conn)
        .await
    }

    /// Find a user by their username
    pub async fn get(db: &PgPool, username: &str) -> Result<Option<User>> {
        let mut conn = db.acquire().await?;
        let result = sqlx::query_as!(
            User,
            "SELECT username, name, access_token, default_access as \"default_access: _\" FROM users \
            WHERE username = $1",
            username
        )
        .fetch_one(&mut conn)
        .await;

        match result {
            Ok(user) => Ok(Some(user)),
            Err(Error::RowNotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Permanently delete a user
    pub async fn delete(db: &PgPool, username: &str) -> Result<()> {
        let mut conn = db.acquire().await?;
        sqlx::query!("DELETE FROM users WHERE username = $1", username)
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    /// Check if the provided token is valid. If the user has no access token, the provided token
    /// will always be reported as invalid
    pub fn access_token_valid(&self, token: &str) -> bool {
        if let Some(hash) = &self.access_token {
            let hash = PasswordHash::new(hash).unwrap();

            // Check the hash
            let argon2 = Argon2::default();
            argon2.verify_password(token.as_bytes(), &hash).is_ok()
        } else {
            false
        }
    }

    /// Re-generate a user's access token
    pub async fn regenerate_access_token(&self, db: &PgPool) -> Result<String> {
        let token = Uuid::new_v4().to_string();

        // Hash the token
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let hash = argon2
            .hash_password(token.as_bytes(), &salt)
            .unwrap()
            .to_string();

        // Update the user
        let mut conn = db.acquire().await?;
        sqlx::query!(
            "UPDATE users SET access_token = $1 WHERE username = $2",
            hash,
            self.username
        )
        .execute(&mut conn)
        .await?;

        Ok(token)
    }

    /// Remove a user's access token
    pub async fn revoke_access_token(&self, db: &PgPool) -> Result<()> {
        let mut conn = db.acquire().await?;
        sqlx::query!(
            "UPDATE users SET access_token = null WHERE username = $1",
            self.username
        )
        .execute(&mut conn)
        .await?;

        Ok(())
    }

    /// Change the default action for the user
    pub async fn set_default_action(&mut self, db: &PgPool, action: Action) -> Result<()> {
        let mut conn = db.acquire().await?;
        sqlx::query!(
            "UPDATE users SET default_access = $1 WHERE username = $2",
            action as _,
            self.username
        )
        .execute(&mut conn)
        .await?;

        Ok(())
    }

    /// Check if the current user is an admin
    pub fn is_admin(&self) -> bool {
        self.default_access == Action::Admin
    }

    /// Get all the permissions for the user
    pub async fn permissions(&self, db: &PgPool) -> Result<Vec<Permission>> {
        let mut conn = db.acquire().await?;
        let permissions = sqlx::query_as!(
            Permission,
            "SELECT id, applies_to, path, action as \"action: _\", affects_children FROM permissions \
            WHERE applies_to = $1",
            self.username
        )
        .fetch_all(&mut conn)
        .await?;

        Ok(permissions)
    }

    /// Assign a permission to a user
    pub async fn assign_permission(
        &self,
        db: &PgPool,
        path: &str,
        action: Action,
        affects_children: bool,
    ) -> Result<Permission> {
        let mut conn = db.acquire().await?;
        let permission = sqlx::query_as!(
            Permission,
            "INSERT INTO permissions (applies_to, path, action, affects_children) \
            VALUES ($1, $2, $3, $4) \
            RETURNING id, applies_to, path, action as \"action: _\", affects_children",
            self.username,
            path,
            action as _,
            affects_children
        )
        .fetch_one(&mut conn)
        .await?;

        Ok(permission)
    }
}

#[ComplexObject]
impl User {
    #[graphql(name = "permissions")]
    async fn permissions_resolver(&self, ctx: &Context<'_>) -> FieldResult<Vec<Permission>> {
        let db = ctx.data::<PgPool>()?;
        let permissions = self.permissions(db).await?;
        Ok(permissions)
    }
}
