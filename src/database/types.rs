use async_graphql::Enum;
use sqlx::Type;

/// The actions a user is allowed to perform
#[derive(Clone, Copy, Debug, Enum, Eq, Ord, PartialEq, PartialOrd, Type)]
#[sqlx(type_name = "action", rename_all = "lowercase")]
pub enum Action {
    Deny,
    Read,
    Modify,
    Admin,
}
