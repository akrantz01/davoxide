use super::types::Action;
use async_graphql::SimpleObject;
use sqlx::{PgPool, Result};

#[derive(Clone, Debug, PartialEq, SimpleObject)]
pub struct Permission {
    pub id: i32,
    #[graphql(skip)]
    pub applies_to: String,
    pub path: String,
    pub action: Action,
    pub affects_children: bool,
}

impl Permission {
    /// Remove a permission
    pub async fn delete(db: &PgPool, id: i32) -> Result<()> {
        let mut conn = db.acquire().await?;
        sqlx::query!("DELETE FROM permissions WHERE id = $1", id)
            .execute(&mut conn)
            .await?;

        Ok(())
    }
}
