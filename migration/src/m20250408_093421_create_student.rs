use crate::m20250408_093456_create_class;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

/// 学生表
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Student::Table)
                    .if_not_exists()
                    .col(pk_auto(Student::Id))
                    .col(string(Student::Name).not_null())
                    .col(tiny_unsigned(Student::Age).default(0))
                    .col(tiny_integer(Student::Gender).default(-1))
                    .col(integer(Student::ClassId).not_null())
                    .col(date_time(Student::EnrollmentDate))
                    .to_owned(),
            )
            .await?;

        // 创建和class表的一对多的关系。student表是多，class表是一
        manager
            .create_foreign_key(
                sea_query::ForeignKey::create()
                    .take()
                    .name("fk_student_class_id")
                    .from(Student::Table, Student::ClassId)
                    .to(
                        m20250408_093456_create_class::Class::Table,
                        m20250408_093456_create_class::Class::Id,
                    )
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Student::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Student {
    Table,
    Id,             // 学号
    Name,           // 姓名
    Age,            // 年龄
    Gender,         // 性别。0是女，1是男
    EnrollmentDate, // 入学日期
    ClassId,        // 班级
}
