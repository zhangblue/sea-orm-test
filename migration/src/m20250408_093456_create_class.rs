use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

/// 班级表
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Class::Table)
                    .if_not_exists()
                    .col(pk_auto(Class::Id))
                    .col(small_unsigned(Class::Class).not_null())
                    .col(small_unsigned(Class::Grade).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Class::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Class {
    Table,
    Id,    // 序号
    Class, // 班级
    Grade, // 年级
}
