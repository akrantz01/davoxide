use crate::tables::*;
use sea_orm_migration::prelude::*;
use sea_query::extension::postgres::Type;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Action::Type)
                    .values(ACTION_VARIANTS.to_vec())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(
                        ColumnDef::new(User::DefaultAccess)
                            .enumeration(Action::Type, &ACTION_VARIANTS)
                            .not_null()
                            .default(ToString::to_string(&Action::Modify)),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Permission::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Permission::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Permission::AppliesTo).string().not_null())
                    .col(ColumnDef::new(Permission::Path).string().not_null())
                    .col(
                        ColumnDef::new(Permission::Action)
                            .enumeration(Action::Type, &ACTION_VARIANTS)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Permission::AffectsChildren)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Permission::Table, Permission::AppliesTo)
                            .to(User::Table, User::Username),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Permission::Table)
                    .col(Permission::AppliesTo)
                    .col(Permission::Path)
                    .col(Permission::Action)
                    .col(Permission::AffectsChildren)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .drop_column(User::DefaultAccess)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Permission::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(Action::Type).to_owned())
            .await?;

        Ok(())
    }
}
