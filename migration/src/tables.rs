use sea_orm_migration::prelude::*;
use std::fmt::{self, Display, Formatter};

pub const ACTION_VARIANTS: [Action; 4] =
    [Action::Deny, Action::Read, Action::Modify, Action::Admin];

#[derive(Clone)]
pub enum Action {
    Type,
    Deny,
    Read,
    Modify,
    Admin,
}

impl Action {
    fn name(&self) -> &'static str {
        match self {
            Self::Type => "action",
            Self::Deny => "deny",
            Self::Read => "read",
            Self::Modify => "modify",
            Self::Admin => "admin",
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Iden for Action {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "{}", self.name()).unwrap();
    }
}

#[derive(Iden)]
pub enum Permission {
    Table,
    Id,
    AppliesTo,
    Path,
    Action,
    AffectsChildren,
}

#[derive(Iden)]
pub enum User {
    Table,
    Username,
    Name,
    AccessToken,
    DefaultAccess,
}
