use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

/// 课程种类表
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Course::Table)
                    .if_not_exists()
                    .col(pk_auto(Course::Id))
                    .col(string(Course::CourseName).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Course::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Course {
    Table,
    Id,         // 课程编号
    CourseName, // 课程名称
}
