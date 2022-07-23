use crate::database::User;
use async_graphql::{Context, Object, Result};

pub struct Query;

#[Object]
impl Query {
    async fn me(&self, ctx: &Context<'_>) -> Result<User> {
        ctx.data::<User>().map(Clone::clone)
    }
}
