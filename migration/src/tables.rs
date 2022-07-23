use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum User {
    Table,
    Username,
    Name,
    AccessToken,
}
