use crate::{m20250408_093456_create_class, m20250408_093514_create_course};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

/// 班级和课程的关系表
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ClassCourse::Table)
                    .if_not_exists()
                    .col(pk_auto(ClassCourse::Id))
                    .col(integer(ClassCourse::ClassId).not_null())
                    .col(integer(ClassCourse::CourseId).not_null())
                    .col(string(ClassCourse::CourseStartTime).not_null())
                    .col(string(ClassCourse::CourseEndTime).not_null())
                    .col(integer(ClassCourse::CourseTimeMinute).default(0))
                    .to_owned(),
            )
            .await?;

        // 创建索引
        manager
            .create_index(
                sea_query::Index::create()
                    .if_not_exists()
                    .table(ClassCourse::Table)
                    .name("idx_class_sourse_class_id_course_id")
                    .col(ClassCourse::ClassId)
                    .col(ClassCourse::CourseId)
                    .to_owned(),
            )
            .await?;

        // 创建与class表的多对一的关系。class_course是多，class表是一
        manager
            .create_foreign_key(
                sea_query::ForeignKey::create()
                    .take()
                    .name("fk_class_course_class_id")
                    .from(ClassCourse::Table, ClassCourse::ClassId)
                    .to(
                        m20250408_093456_create_class::Class::Table,
                        m20250408_093456_create_class::Class::Id,
                    )
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // 创建与course表的多对一关系。class_course是多，course表是一
        manager
            .create_foreign_key(
                sea_query::ForeignKey::create()
                    .take()
                    .name("fk_class_course_course_id")
                    .from(ClassCourse::Table, ClassCourse::CourseId)
                    .to(
                        m20250408_093514_create_course::Course::Table,
                        m20250408_093514_create_course::Course::Id,
                    )
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ClassCourse::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ClassCourse {
    Table,
    Id,
    ClassId,
    CourseId,
    CourseStartTime,  // 课程开始时间
    CourseEndTime,    // 课程结束时间
    CourseTimeMinute, // 课程时间（分钟）
}
