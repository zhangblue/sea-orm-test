use crate::m20250408_093421_create_student::Student;
use chrono::NaiveDateTime;
use sea_orm::{EntityTrait, Set};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let connection = manager.get_connection();
        let seed_data_active_module_vec: Vec<entity::student::ActiveModel> = get_seeding_data_vec()
            .into_iter()
            .map(|x| entity::student::ActiveModel {
                id: Set(x.id),
                name: Set(x.name),
                age: Set(x.age),
                gender: Set(x.gender),
                class_id: Set(x.class_id),
                enrollment_date: Set(x.enrollment_date),
            })
            .collect();

        entity::student::Entity::insert_many(seed_data_active_module_vec)
            .exec(connection)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .truncate_table(Table::truncate().table(Student::Table).to_owned())
            .await
    }
}

fn get_seeding_data_vec() -> Vec<StudentSeedModule> {
    vec![
        StudentSeedModule {
            id: 0,
            name: "张三".to_string(),
            age: 12,
            gender: 1,
            class_id: 1,
            enrollment_date: chrono::Utc::now().naive_local(),
        },
        StudentSeedModule {
            id: 1,
            name: "李四".to_string(),
            age: 10,
            gender: 1,
            class_id: 1,
            enrollment_date: chrono::Utc::now().naive_local(),
        },
        StudentSeedModule {
            id: 2,
            name: "王五".to_string(),
            age: 13,
            gender: 0,
            class_id: 2,
            enrollment_date: chrono::Utc::now().naive_local(),
        },
        StudentSeedModule {
            id: 3,
            name: "赵六".to_string(),
            age: 13,
            gender: 0,
            class_id: 2,
            enrollment_date: chrono::Utc::now().naive_local(),
        },
    ]
}

struct StudentSeedModule {
    id: i32,
    name: String,
    age: i16,
    gender: i16,
    class_id: i32,
    enrollment_date: NaiveDateTime,
}
